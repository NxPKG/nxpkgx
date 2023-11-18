//! JSON asset support for nxpkgpack.
//!
//! JSON assets are parsed to ensure they contain valid JSON.
//!
//! When imported from ES modules, they produce a module that exports the
//! JSON value as an object.

#![feature(min_specialization)]
#![feature(arbitrary_self_types)]

use std::fmt::Write;

use anyhow::{bail, Context, Error, Result};
use nxpkg_tasks::{ValueToString, Vc};
use nxpkg_tasks_fs::{FileContent, FileJsonContent};
use nxpkgpack_core::{
    asset::{Asset, AssetContent},
    chunk::{ChunkItem, ChunkType, ChunkableModule, ChunkingContext},
    ident::AssetIdent,
    module::Module,
    reference::ModuleReferences,
    source::Source,
};
use nxpkgpack_ecmascript::chunk::{
    EcmascriptChunkItem, EcmascriptChunkItemContent, EcmascriptChunkPlaceable, EcmascriptChunkType,
    EcmascriptChunkingContext, EcmascriptExports,
};

#[nxpkg_tasks::function]
fn modifier() -> Vc<String> {
    Vc::cell("json".to_string())
}

#[nxpkg_tasks::value]
pub struct JsonModuleAsset {
    source: Vc<Box<dyn Source>>,
}

#[nxpkg_tasks::value_impl]
impl JsonModuleAsset {
    #[nxpkg_tasks::function]
    pub fn new(source: Vc<Box<dyn Source>>) -> Vc<Self> {
        Self::cell(JsonModuleAsset { source })
    }
}

#[nxpkg_tasks::value_impl]
impl Module for JsonModuleAsset {
    #[nxpkg_tasks::function]
    fn ident(&self) -> Vc<AssetIdent> {
        self.source.ident().with_modifier(modifier())
    }
}

#[nxpkg_tasks::value_impl]
impl Asset for JsonModuleAsset {
    #[nxpkg_tasks::function]
    fn content(&self) -> Vc<AssetContent> {
        self.source.content()
    }
}

#[nxpkg_tasks::value_impl]
impl ChunkableModule for JsonModuleAsset {
    #[nxpkg_tasks::function]
    async fn as_chunk_item(
        self: Vc<Self>,
        chunking_context: Vc<Box<dyn ChunkingContext>>,
    ) -> Result<Vc<Box<dyn nxpkgpack_core::chunk::ChunkItem>>> {
        let chunking_context =
            Vc::try_resolve_downcast::<Box<dyn EcmascriptChunkingContext>>(chunking_context)
                .await?
                .context(
                    "chunking context must impl EcmascriptChunkingContext to use JsonModuleAsset",
                )?;
        Ok(Vc::upcast(JsonChunkItem::cell(JsonChunkItem {
            module: self,
            chunking_context,
        })))
    }
}

#[nxpkg_tasks::value_impl]
impl EcmascriptChunkPlaceable for JsonModuleAsset {
    #[nxpkg_tasks::function]
    fn get_exports(&self) -> Vc<EcmascriptExports> {
        EcmascriptExports::Value.cell()
    }
}

#[nxpkg_tasks::value]
struct JsonChunkItem {
    module: Vc<JsonModuleAsset>,
    chunking_context: Vc<Box<dyn EcmascriptChunkingContext>>,
}

#[nxpkg_tasks::value_impl]
impl ChunkItem for JsonChunkItem {
    #[nxpkg_tasks::function]
    fn asset_ident(&self) -> Vc<AssetIdent> {
        self.module.ident()
    }

    #[nxpkg_tasks::function]
    fn references(&self) -> Vc<ModuleReferences> {
        self.module.references()
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
        Vc::upcast(self.module)
    }
}

#[nxpkg_tasks::value_impl]
impl EcmascriptChunkItem for JsonChunkItem {
    #[nxpkg_tasks::function]
    fn chunking_context(&self) -> Vc<Box<dyn EcmascriptChunkingContext>> {
        self.chunking_context
    }

    #[nxpkg_tasks::function]
    async fn content(&self) -> Result<Vc<EcmascriptChunkItemContent>> {
        // We parse to JSON and then stringify again to ensure that the
        // JSON is valid.
        let content = self.module.content().file_content();
        let data = content.parse_json().await?;
        match &*data {
            FileJsonContent::Content(data) => {
                let js_str_content = serde_json::to_string(&data.to_string())?;
                let inner_code =
                    format!("__nxpkgpack_export_value__(JSON.parse({js_str_content}));");

                Ok(EcmascriptChunkItemContent {
                    inner_code: inner_code.into(),
                    ..Default::default()
                }
                .into())
            }
            FileJsonContent::Unparseable(e) => {
                let mut message = "Unable to make a module from invalid JSON: ".to_string();
                if let FileContent::Content(content) = &*content.await? {
                    let text = content.content().to_str()?;
                    e.write_with_content(&mut message, text.as_ref())?;
                } else {
                    write!(message, "{}", e)?;
                }

                Err(Error::msg(message))
            }
            FileJsonContent::NotFound => {
                bail!(
                    "JSON file not found: {}",
                    self.module.ident().to_string().await?
                );
            }
        }
    }
}

pub fn register() {
    nxpkg_tasks::register();
    nxpkg_tasks_fs::register();
    nxpkgpack_core::register();
    nxpkgpack_ecmascript::register();
    include!(concat!(env!("OUT_DIR"), "/register.rs"));
}
