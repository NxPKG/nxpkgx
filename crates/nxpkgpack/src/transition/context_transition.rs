use anyhow::Result;
use nxpkg_tasks::Vc;
use nxpkgpack_core::compile_time_info::CompileTimeInfo;

use crate::{
    module_options::ModuleOptionsContext, resolve_options_context::ResolveOptionsContext,
    transition::Transition,
};

/// A transition that only affects the asset context.
#[nxpkg_tasks::value(shared)]
pub struct ContextTransition {
    compile_time_info: Vc<CompileTimeInfo>,
    module_options_context: Vc<ModuleOptionsContext>,
    resolve_options_context: Vc<ResolveOptionsContext>,
    layer: Vc<String>,
}

#[nxpkg_tasks::value_impl]
impl ContextTransition {
    #[nxpkg_tasks::function]
    pub async fn new(
        compile_time_info: Vc<CompileTimeInfo>,
        module_options_context: Vc<ModuleOptionsContext>,
        resolve_options_context: Vc<ResolveOptionsContext>,
        layer: Vc<String>,
    ) -> Result<Vc<ContextTransition>> {
        Ok(ContextTransition {
            module_options_context,
            resolve_options_context,
            compile_time_info,
            layer,
        }
        .cell())
    }
}

#[nxpkg_tasks::value_impl]
impl Transition for ContextTransition {
    #[nxpkg_tasks::function]
    fn process_compile_time_info(
        &self,
        _compile_time_info: Vc<CompileTimeInfo>,
    ) -> Vc<CompileTimeInfo> {
        self.compile_time_info
    }

    #[nxpkg_tasks::function]
    fn process_layer(&self, _layer: Vc<String>) -> Vc<String> {
        self.layer
    }

    #[nxpkg_tasks::function]
    fn process_module_options_context(
        &self,
        _context: Vc<ModuleOptionsContext>,
    ) -> Vc<ModuleOptionsContext> {
        self.module_options_context
    }

    #[nxpkg_tasks::function]
    fn process_resolve_options_context(
        &self,
        _context: Vc<ResolveOptionsContext>,
    ) -> Vc<ResolveOptionsContext> {
        self.resolve_options_context
    }
}
