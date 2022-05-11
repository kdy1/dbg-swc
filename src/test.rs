use std::{
    fs,
    process::{Command, Stdio},
    sync::Arc,
};

use anyhow::{bail, Context, Result};
use clap::{Args, Subcommand};
use swc_common::SourceMap;
use swc_ecma_minifier::option::MinifyOptions;
use swc_timer::timer;
use tracing::info;

use crate::{bundle::bundle, util::print_js};

/// Execute a javascript file after performing some preprocessing.
#[derive(Debug, Subcommand)]
pub enum TestCommand {
    MinifiedBundle(TestMinifiedBundleCommand),
}

impl TestCommand {
    pub fn run(self, cm: Arc<SourceMap>) -> Result<()> {
        let _timer = timer!("test");

        let output = {
            let _timer = timer!("process");

            match self {
                TestCommand::MinifiedBundle(cmd) => cmd.run(cm),
            }?
        };

        {
            let _timer = timer!("run");
            let stdout = output
                .runtime
                .execute(&output.code)
                .context("failed to execute generated code")?;

            info!("----- Stdout -----\n{}", stdout);
        }

        Ok(())
    }
}

#[derive(Debug, Args)]
pub struct TestMinifiedBundleCommand {
    entry: String,
}

impl TestMinifiedBundleCommand {
    fn run(self, cm: Arc<SourceMap>) -> Result<Output> {
        let bundle = bundle(cm.clone(), &self.entry)?;

        let minified = {
            let _timer = timer!("minify");
            swc_ecma_minifier::optimize(
                bundle.module,
                cm.clone(),
                None,
                None,
                &MinifyOptions {
                    compress: Some(Default::default()),
                    mangle: Some(Default::default()),
                    ..Default::default()
                },
                &swc_ecma_minifier::option::ExtraOptions {
                    unresolved_mark: bundle.unresolved_mark,
                    top_level_mark: bundle.top_level_mark,
                },
            )
        };

        let code =
            print_js(cm.clone(), &minified, true).context("failed to convert ast to code")?;

        fs::write("output.js", code.as_bytes()).context("failed to write code as file")?;

        Ok(Output {
            code,
            runtime: JsRuntime::Deno,
        })
    }
}

pub struct Output {
    pub code: String,
    pub runtime: JsRuntime,
}

pub enum JsRuntime {
    Node,
    Deno,
}

impl JsRuntime {
    pub fn execute(&self, code: &str) -> Result<String> {
        match self {
            JsRuntime::Node => todo!("node.execute"),
            JsRuntime::Deno => {
                let mut cmd = Command::new("deno");
                cmd.arg("eval").arg("--no-check");

                cmd.arg(code);

                cmd.stderr(Stdio::inherit());

                let output = cmd.output().context("failed to get output from deno")?;

                if !output.status.success() {
                    bail!("deno exited with status {}", output.status);
                }

                Ok(
                    (String::from_utf8(output.stdout).context("deno emitted non-utf8 string")?)
                        .trim()
                        .to_string(),
                )
            }
        }
    }
}
