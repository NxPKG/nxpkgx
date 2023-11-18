use anyhow::{Context, Result};
use nxpkg_tasks::{Value, Vc};
use nxpkgpack_core::{
    asset::{Asset, AssetContent},
    chunk::{availability_info::AvailabilityInfo, ChunkableModule, ChunkingContext},
    ident::AssetIdent,
    module::Module,
    output::OutputAssets,
    reference::{ModuleReferences, SingleOutputAssetReference},
};

use super::chunk_item::ManifestChunkItem;
use crate::chunk::{EcmascriptChunkPlaceable, EcmascriptChunkingContext, EcmascriptExports};

#[nxpkg_tasks::function]
fn modifier() -> Vc<String> {
    Vc::cell("manifest chunk".to_string())
}

/// The manifest module is deferred until requested by the manifest loader
/// item when the dynamic `import()` expression is reached. Its responsibility
/// is to generate a Promise that will resolve only after all the necessary
/// chunks needed by the dynamic import are loaded by the client.
///
/// Splitting the dynamic import into a quickly generate-able manifest loader
/// item and a slow-to-generate manifest chunk allows for faster incremental
/// compilation. The traversal won't be performed until the dynamic import is
/// actually reached, instead of eagerly as part of the chunk that the dynamic
/// import appears in.
#[nxpkg_tasks::value(shared)]
pub struct ManifestAsyncModule {
    pub inner: Vc<Box<dyn ChunkableModule>>,
    pub chunking_context: Vc<Box<dyn EcmascriptChunkingContext>>,
    pub availability_info: AvailabilityInfo,
}

#[nxpkg_tasks::value_impl]
impl ManifestAsyncModule {
    #[nxpkg_tasks::function]
    pub fn new(
        module: Vc<Box<dyn ChunkableModule>>,
        chunking_context: Vc<Box<dyn EcmascriptChunkingContext>>,
        availability_info: Value<AvailabilityInfo>,
    ) -> Vc<Self> {
        Self::cell(ManifestAsyncModule {
            inner: module,
            chunking_context,
            availability_info: availability_info.into_value(),
        })
    }

    #[nxpkg_tasks::function]
    pub(super) async fn chunks(self: Vc<Self>) -> Result<Vc<OutputAssets>> {
        let this = self.await?;
        Ok(this
            .chunking_context
            .chunk_group(Vc::upcast(this.inner), Value::new(this.availability_info)))
    }

    #[nxpkg_tasks::function]
    pub async fn manifest_chunks(self: Vc<Self>) -> Result<Vc<OutputAssets>> {
        let this = self.await?;
        Ok(this
            .chunking_context
            .chunk_group(Vc::upcast(self), Value::new(this.availability_info)))
    }

    #[nxpkg_tasks::function]
    pub fn module_ident(&self) -> Vc<AssetIdent> {
        self.inner.ident()
    }

    #[nxpkg_tasks::function]
    pub async fn content_ident(&self) -> Result<Vc<AssetIdent>> {
        let mut ident = self.inner.ident();
        if let Some(available_modules) = self.availability_info.available_chunk_items() {
            ident = ident.with_modifier(Vc::cell(available_modules.hash().await?.to_string()));
        }
        Ok(ident)
    }
}

#[nxpkg_tasks::function]
fn manifest_chunk_reference_description() -> Vc<String> {
    Vc::cell("manifest chunk".to_string())
}

#[nxpkg_tasks::value_impl]
impl Module for ManifestAsyncModule {
    #[nxpkg_tasks::function]
    fn ident(&self) -> Vc<AssetIdent> {
        self.inner.ident().with_modifier(modifier())
    }

    #[nxpkg_tasks::function]
    async fn references(self: Vc<Self>) -> Result<Vc<ModuleReferences>> {
        let chunks = self.chunks();

        Ok(Vc::cell(
            chunks
                .await?
                .iter()
                .copied()
                .map(|chunk| {
                    Vc::upcast(SingleOutputAssetReference::new(
                        chunk,
                        manifest_chunk_reference_description(),
                    ))
                })
                .collect(),
        ))
    }
}

#[nxpkg_tasks::value_impl]
impl Asset for ManifestAsyncModule {
    #[nxpkg_tasks::function]
    fn content(&self) -> Vc<AssetContent> {
        todo!()
    }
}

#[nxpkg_tasks::value_impl]
impl ChunkableModule for ManifestAsyncModule {
    #[nxpkg_tasks::function]
    async fn as_chunk_item(
        self: Vc<Self>,
        chunking_context: Vc<Box<dyn ChunkingContext>>,
    ) -> Result<Vc<Box<dyn nxpkgpack_core::chunk::ChunkItem>>> {
        let chunking_context =
            Vc::try_resolve_downcast::<Box<dyn EcmascriptChunkingContext>>(chunking_context)
                .await?
                .context(
                    "chunking context must impl EcmascriptChunkingContext to use \
                     ManifestChunkAsset",
                )?;
        Ok(Vc::upcast(
            ManifestChunkItem {
                chunking_context,
                manifest: self,
            }
            .cell(),
        ))
    }
}

#[nxpkg_tasks::value_impl]
impl EcmascriptChunkPlaceable for ManifestAsyncModule {
    #[nxpkg_tasks::function]
    fn get_exports(&self) -> Vc<EcmascriptExports> {
        EcmascriptExports::Value.cell()
    }
}
