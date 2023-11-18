use anyhow::Result;
use nxpkg_tasks::Vc;
use nxpkg_tasks_fs::File;
use nxpkgpack_core::{
    asset::{Asset, AssetContent},
    chunk::Chunk,
    ident::AssetIdent,
    output::OutputAsset,
    source_map::{GenerateSourceMap, SourceMap},
};

use super::chunk::SingleItemCssChunk;

/// Represents the source map of a single item CSS chunk.
#[nxpkg_tasks::value]
pub struct SingleItemCssChunkSourceMapAsset {
    chunk: Vc<SingleItemCssChunk>,
}

#[nxpkg_tasks::value_impl]
impl SingleItemCssChunkSourceMapAsset {
    #[nxpkg_tasks::function]
    pub fn new(chunk: Vc<SingleItemCssChunk>) -> Vc<Self> {
        SingleItemCssChunkSourceMapAsset { chunk }.cell()
    }
}

#[nxpkg_tasks::value_impl]
impl OutputAsset for SingleItemCssChunkSourceMapAsset {
    #[nxpkg_tasks::function]
    async fn ident(&self) -> Result<Vc<AssetIdent>> {
        Ok(AssetIdent::from_path(
            self.chunk.path().append(".map".to_string()),
        ))
    }
}

#[nxpkg_tasks::value_impl]
impl Asset for SingleItemCssChunkSourceMapAsset {
    #[nxpkg_tasks::function]
    async fn content(&self) -> Result<Vc<AssetContent>> {
        let sm = if let Some(sm) = *self.chunk.generate_source_map().await? {
            sm
        } else {
            SourceMap::empty()
        };
        let sm = sm.to_rope().await?;
        Ok(AssetContent::file(File::from(sm).into()))
    }
}
