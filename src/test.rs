use std::path::PathBuf;

use anyhow::Result;
use clap::{Args, Subcommand};
use swc_timer::timer;

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
        }
    }
}

#[derive(Debug, Args)]
pub struct TestMinifiedBundleCommand {
    entry: PathBuf,
}

impl TestMinifiedBundleCommand {
    fn run(self) -> Result<Output> {
        let bundle = {
            let _timer = timer!("bundle");
        };

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
