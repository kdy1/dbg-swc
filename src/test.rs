use clap::{Args, Subcommand};

/// Execute a javascript file after performing some preprocessing.
#[derive(Debug, Subcommand)]
pub enum TestCommand {
    MinifiedBundle(TestMinifiedBundleCommand),
}

#[derive(Debug, Args)]
pub struct TestMinifiedBundleCommand {}
