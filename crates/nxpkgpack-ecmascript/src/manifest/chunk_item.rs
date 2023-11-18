use anyhow::Result;
use indoc::formatdoc;
use nxpkg_tasks::{TryJoinIterExt, Vc};
use nxpkgpack_core::{
    chunk::{ChunkData, ChunkItem, ChunkType, ChunkingContext, ChunksData},
    ident::AssetIdent,
    module::Module,
    reference::{ModuleReferences, SingleOutputAssetReference},
};

use super::chunk_asset::ManifestAsyncModule;
use crate::{
    chunk::{
        data::EcmascriptChunkData, EcmascriptChunkItem, EcmascriptChunkItemContent,
        EcmascriptChunkType, EcmascriptChunkingContext,
    },
    utils::StringifyJs,
};

/// The ManifestChunkItem generates a __nxpkgpack_load__ call for every chunk
/// necessary to load the real asset. Once all the loads resolve, it is safe to
/// __nxpkgpack_import__ the actual module that was dynamically imported.
#[nxpkg_tasks::value(shared)]
pub(super) struct ManifestChunkItem {
    pub chunking_context: Vc<Box<dyn EcmascriptChunkingContext>>,
    pub manifest: Vc<ManifestAsyncModule>,
}

#[nxpkg_tasks::value_impl]
impl ManifestChunkItem {
    #[nxpkg_tasks::function]
    async fn chunks_data(self: Vc<Self>) -> Result<Vc<ChunksData>> {
        let this = self.await?;
        Ok(ChunkData::from_assets(
            this.chunking_context.output_root(),
            this.manifest.chunks(),
        ))
    }
}

#[nxpkg_tasks::value_impl]
impl EcmascriptChunkItem for ManifestChunkItem {
    #[nxpkg_tasks::function]
    fn chunking_context(&self) -> Vc<Box<dyn EcmascriptChunkingContext>> {
        self.chunking_context
    }

    #[nxpkg_tasks::function]
    async fn content(self: Vc<Self>) -> Result<Vc<EcmascriptChunkItemContent>> {
        let chunks_data = self.chunks_data().await?;
        let chunks_data = chunks_data.iter().try_join().await?;
        let chunks_data: Vec<_> = chunks_data
            .iter()
            .map(|chunk_data| EcmascriptChunkData::new(chunk_data))
            .collect();

        let code = formatdoc! {
            r#"
                __nxpkgpack_export_value__({:#});
            "#,
            StringifyJs(&chunks_data)
        };

        Ok(EcmascriptChunkItemContent {
            inner_code: code.into(),
            ..Default::default()
        }
        .into())
    }
}

#[nxpkg_tasks::value_impl]
impl ChunkItem for ManifestChunkItem {
    #[nxpkg_tasks::function]
    fn asset_ident(&self) -> Vc<AssetIdent> {
        self.manifest.ident()
    }

    #[nxpkg_tasks::function]
    fn content_ident(&self) -> Vc<AssetIdent> {
        self.manifest.content_ident()
    }

    #[nxpkg_tasks::function]
    async fn references(self: Vc<Self>) -> Result<Vc<ModuleReferences>> {
        let this = self.await?;
        let mut references = this.manifest.references().await?.clone_value();

        let key = Vc::cell("chunk data reference".to_string());

        for chunk_data in &*self.chunks_data().await? {
            references.extend(chunk_data.references().await?.iter().map(|&output_asset| {
                Vc::upcast(SingleOutputAssetReference::new(output_asset, key))
            }));
        }

        Ok(Vc::cell(references))
    }

    #[nxpkg_tasks::function]
    async fn chunking_context(&self) -> Vc<Box<dyn ChunkingContext>> {
        Vc::upcast(self.chunking_context)
    }

    #[nxpkg_tasks::function]
    async fn ty(&self) -> Result<Vc<Box<dyn ChunkType>>> {
        Ok(Vc::upcast(
            Vc::<EcmascriptChunkType>::default().resolve().await?,
        ))
    }

    #[nxpkg_tasks::function]
    fn module(&self) -> Vc<Box<dyn Module>> {
        Vc::upcast(self.manifest)
    }
}
