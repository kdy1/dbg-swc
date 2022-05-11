use std::sync::Arc;

use anyhow::{bail, Result};
use swc_common::{errors::HANDLER, SourceFile, SourceMap};
use swc_ecma_ast::{EsVersion, Module};
use swc_ecma_parser::{parse_file_as_module, Syntax};

/// Type annotation
pub fn wrap_task<T, F>(op: F) -> Result<T>
where
    F: FnOnce() -> Result<T>,
{
    op()
}

pub fn parse_js(fm: Arc<SourceFile>) -> Result<Module> {
    let mut errors = vec![];
    let res = parse_file_as_module(
        &fm,
        Syntax::Es(Default::default()),
        EsVersion::latest(),
        None,
        &mut errors,
    )
    .map_err(|err| HANDLER.with(|handler| err.into_diagnostic(handler).emit()));

    for err in errors {
        HANDLER.with(|handler| err.into_diagnostic(handler).emit());
    }

    match res {
        Ok(v) => Ok(v),
        Err(()) => bail!("failed to parse a js file as a module"),
    }
}
