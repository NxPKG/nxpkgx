use anyhow::{anyhow, Result};
use nxpkg_tasks::Vc;
use nxpkg_tasks_fs::FileContent;
use nxpkgpack_core::{
    asset::{Asset, AssetContent},
    chunk::ChunkingContext,
    ident::AssetIdent,
    output::OutputAsset,
    source::Source,
};
#[nxpkg_tasks::value]
pub struct StaticAsset {
    chunking_context: Vc<Box<dyn ChunkingContext>>,
    source: Vc<Box<dyn Source>>,
}

#[nxpkg_tasks::value_impl]
impl StaticAsset {
    #[nxpkg_tasks::function]
    pub fn new(
        chunking_context: Vc<Box<dyn ChunkingContext>>,
        source: Vc<Box<dyn Source>>,
    ) -> Vc<Self> {
        Self::cell(StaticAsset {
            chunking_context,
            source,
        })
    }
}

#[nxpkg_tasks::value_impl]
impl OutputAsset for StaticAsset {
    #[nxpkg_tasks::function]
    async fn ident(&self) -> Result<Vc<AssetIdent>> {
        let content = self.source.content();
        let content_hash = if let AssetContent::File(file) = &*content.await? {
            if let FileContent::Content(file) = &*file.await? {
                nxpkg_tasks_hash::hash_xxh3_hash64(file.content())
            } else {
                return Err(anyhow!("StaticAsset::path: not found"));
            }
        } else {
            return Err(anyhow!("StaticAsset::path: unsupported file content"));
        };
        let content_hash_b16 = nxpkg_tasks_hash::encode_hex(content_hash);
        let asset_path = self
            .chunking_context
            .asset_path(content_hash_b16, self.source.ident());
        Ok(AssetIdent::from_path(asset_path))
    }
}

#[nxpkg_tasks::value_impl]
impl Asset for StaticAsset {
    #[nxpkg_tasks::function]
    fn content(&self) -> Vc<AssetContent> {
        self.source.content()
    }
}
