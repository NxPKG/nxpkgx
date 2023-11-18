use nxpkg_tasks::Vc;
use nxpkgpack_core::{module::Module, resolve::ModulePart, source::Source};

use crate::ModuleAssetContext;

#[nxpkg_tasks::value_trait]
pub trait CustomModuleType {
    fn create_module(
        self: Vc<Self>,
        source: Vc<Box<dyn Source>>,
        module_asset_context: Vc<ModuleAssetContext>,
        part: Option<Vc<ModulePart>>,
    ) -> Vc<Box<dyn Module>>;
}
