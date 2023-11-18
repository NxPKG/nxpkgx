use nxpkg_tasks::Vc;
use nxpkgpack_core::{asset::Asset, chunk::ChunkableModule, module::Module};

use crate::references::{async_module::OptionAsyncModule, esm::EsmExports};

#[nxpkg_tasks::value_trait]
pub trait EcmascriptChunkPlaceable: ChunkableModule + Module + Asset {
    fn get_exports(self: Vc<Self>) -> Vc<EcmascriptExports>;
    fn get_async_module(self: Vc<Self>) -> Vc<OptionAsyncModule> {
        Vc::cell(None)
    }
}

#[nxpkg_tasks::value(transparent)]
pub struct EcmascriptChunkPlaceables(Vec<Vc<Box<dyn EcmascriptChunkPlaceable>>>);

#[nxpkg_tasks::value_impl]
impl EcmascriptChunkPlaceables {
    #[nxpkg_tasks::function]
    pub fn empty() -> Vc<Self> {
        Vc::cell(Vec::new())
    }
}

#[nxpkg_tasks::value(shared)]
pub enum EcmascriptExports {
    EsmExports(Vc<EsmExports>),
    DynamicNamespace,
    CommonJs,
    Value,
    None,
}
