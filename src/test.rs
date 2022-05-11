use std::{
    path::PathBuf,
    process::{Command, Stdio},
};

use anyhow::{bail, Context, Result};
use clap::{Args, Subcommand};
use swc_bundler::Bundler;
use swc_timer::timer;
use tracing::info;

use crate::bundle::bundle;

/// Execute a javascript file after performing some preprocessing.
#[derive(Debug, Subcommand)]
pub enum TestCommand {
    MinifiedBundle(TestMinifiedBundleCommand),
}

impl TestCommand {
    pub fn run(self) -> Result<()> {
        let _timer = timer!("test");

        let output = {
            let _timer = timer!("process");

            match self {
                TestCommand::MinifiedBundle(cmd) => cmd.run(),
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
    fn run(self) -> Result<Output> {
        let bundle = bundle(&self.entry)?;

        let minified = {
            let _timer = timer!("minify");
        };
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
