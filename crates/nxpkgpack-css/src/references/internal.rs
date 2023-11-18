use anyhow::Result;
use nxpkg_tasks::{ValueToString, Vc};
use nxpkgpack_core::{
    chunk::ChunkableModuleReference, module::Module, reference::ModuleReference,
    resolve::ModuleResolveResult,
};

/// A reference to an internal CSS asset.
#[nxpkg_tasks::value]
#[derive(Hash, Debug)]
pub struct InternalCssAssetReference {
    module: Vc<Box<dyn Module>>,
}

#[nxpkg_tasks::value_impl]
impl InternalCssAssetReference {
    /// Creates a new [`Vc<InternalCssAssetReference>`].
    #[nxpkg_tasks::function]
    pub fn new(module: Vc<Box<dyn Module>>) -> Vc<Self> {
        Self::cell(InternalCssAssetReference { module })
    }
}

#[nxpkg_tasks::value_impl]
impl ModuleReference for InternalCssAssetReference {
    #[nxpkg_tasks::function]
    fn resolve_reference(&self) -> Vc<ModuleResolveResult> {
        ModuleResolveResult::module(self.module).cell()
    }
}

#[nxpkg_tasks::value_impl]
impl ValueToString for InternalCssAssetReference {
    #[nxpkg_tasks::function]
    async fn to_string(&self) -> Result<Vc<String>> {
        Ok(Vc::cell(format!(
            "internal css {}",
            self.module.ident().to_string().await?
        )))
    }
}

#[nxpkg_tasks::value_impl]
impl ChunkableModuleReference for InternalCssAssetReference {}
