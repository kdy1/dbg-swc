use clap::{Args, Subcommand};

#[derive(Debug, Subcommand)]
pub enum TestCommand {
    MinifiedBundle(TestMinifiedBundleCommand),
}

#[derive(Debug, Args)]
pub struct TestMinifiedBundleCommand {}
