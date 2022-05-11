use anyhow::{Context, Result};
use clap::Subcommand;
use swc_ecma_ast::Module;
use swc_timer::timer;

use crate::util::task;

#[derive(Debug, Subcommand)]
pub enum BundleCommand {}

pub fn bundle(entry_url: &str) -> Result<Module> {
    task(|| {
        let _timer = timer!("bundle");
    })
    .with_context(|| format!("failed to bundle `{}`", entry_url))
}
