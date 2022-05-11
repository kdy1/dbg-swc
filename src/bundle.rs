use std::sync::Arc;

use anyhow::{Context, Result};
use clap::Subcommand;
use swc_bundler::Bundler;
use swc_common::{SourceMap, GLOBALS};
use swc_ecma_ast::Module;
use swc_timer::timer;

use crate::util::wrap_task;

#[derive(Debug, Subcommand)]
pub enum BundleCommand {}

pub fn bundle(cm: Arc<SourceMap>, entry_url: &str) -> Result<Module> {
    wrap_task(|| {
        let _timer = timer!("bundle");

        GLOBALS.with(|globals| {
            let bundler = Bundler::new(
                globals,
                cm,
                loader,
                resolver,
                swc_bundler::Config {
                    require: (),
                    disable_inliner: (),
                    disable_hygiene: (),
                    disable_fixer: (),
                    disable_dce: (),
                    external_modules: (),
                    module: (),
                },
                box BundlerHook,
            );
        })
    })
    .with_context(|| format!("failed to bundle `{}`", entry_url))
}

struct BundlerHook;
