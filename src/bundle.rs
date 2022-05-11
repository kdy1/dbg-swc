use std::{collections::HashMap, sync::Arc};

use anyhow::{Context, Result};
use clap::Subcommand;
use swc_bundler::{Bundler, Hook, ModuleRecord};
use swc_common::{FileName, SourceMap, Span, GLOBALS};
use swc_ecma_ast::{KeyValueProp, Module};
use swc_timer::timer;
use url::Url;

use crate::util::wrap_task;

#[derive(Debug, Subcommand)]
pub enum BundleCommand {}

pub fn bundle(cm: Arc<SourceMap>, entry_url: &Url) -> Result<Module> {
    wrap_task(|| {
        let _timer = timer!("bundle");

        GLOBALS.with(|globals| {
            let bundler = Bundler::new(
                globals,
                cm,
                loader,
                resolver,
                swc_bundler::Config {
                    require: true,
                    disable_inliner: true,
                    disable_hygiene: false,
                    disable_fixer: false,
                    disable_dce: true,
                    external_modules: vec![],
                    module: swc_bundler::ModuleType::Es,
                },
                box BundlerHook,
            );

            let mut entries = HashMap::default();
            entries.insert("main", FileName::Url(entry_url.clone()));
            let mut modules = bundler.bundle(entries).context("Bundler.bundle failed")?;
            let built = modules.remove(0);

            Ok(built.module)
        })
    })
    .with_context(|| format!("failed to bundle `{}`", entry_url))
}

struct BundlerHook;

impl Hook for BundlerHook {
    fn get_import_meta_props(
        &self,
        span: Span,
        module_record: &ModuleRecord,
    ) -> Result<Vec<KeyValueProp>> {
        Ok(vec![])
    }
}
