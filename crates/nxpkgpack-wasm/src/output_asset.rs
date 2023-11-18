use anyhow::Result;
use nxpkg_tasks::Vc;
use nxpkgpack_core::{
    asset::{Asset, AssetContent},
    chunk::ChunkingContext,
    ident::AssetIdent,
    output::OutputAsset,
    source::Source,
};

use crate::source::WebAssemblySource;

#[nxpkg_tasks::function]
fn modifier() -> Vc<String> {
    Vc::cell("wasm".to_string())
}

/// Emits the [WebAssemblySource] at a chunk path determined by the
/// [ChunkingContext].
#[nxpkg_tasks::value]
pub(crate) struct WebAssemblyAsset {
    source: Vc<WebAssemblySource>,
    chunking_context: Vc<Box<dyn ChunkingContext>>,
}

#[nxpkg_tasks::value_impl]
impl WebAssemblyAsset {
    #[nxpkg_tasks::function]
    pub(crate) fn new(
        source: Vc<WebAssemblySource>,
        chunking_context: Vc<Box<dyn ChunkingContext>>,
    ) -> Vc<Self> {
        Self::cell(WebAssemblyAsset {
            source,
            chunking_context,
        })
    }
}

#[nxpkg_tasks::value_impl]
impl OutputAsset for WebAssemblyAsset {
    #[nxpkg_tasks::function]
    async fn ident(&self) -> Result<Vc<AssetIdent>> {
        let ident = self.source.ident().with_modifier(modifier());

        let asset_path = self.chunking_context.chunk_path(ident, ".wasm".to_string());

        Ok(AssetIdent::from_path(asset_path))
    }
}

#[nxpkg_tasks::value_impl]
impl Asset for WebAssemblyAsset {
    #[nxpkg_tasks::function]
    fn content(&self) -> Vc<AssetContent> {
        self.source.content()
    }
}
