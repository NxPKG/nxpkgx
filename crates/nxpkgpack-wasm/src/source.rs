use anyhow::Result;
use serde::{Deserialize, Serialize};
use nxpkg_tasks::{trace::TraceRawVcs, TaskInput, Vc};
use nxpkg_tasks_fs::{File, FileContent};
use nxpkgpack_core::{
    asset::{Asset, AssetContent},
    ident::AssetIdent,
    source::Source,
};

#[derive(
    PartialOrd,
    Ord,
    Eq,
    PartialEq,
    Hash,
    Debug,
    Copy,
    Clone,
    Serialize,
    Deserialize,
    TaskInput,
    TraceRawVcs,
)]
pub enum WebAssemblySourceType {
    /// Binary WebAssembly files (.wasm).
    Binary,
    /// WebAssembly text format (.wat).
    Text,
}

/// Returns the raw binary WebAssembly source or the assembled version of a text
/// format source.
#[nxpkg_tasks::value]
#[derive(Clone)]
pub struct WebAssemblySource {
    source: Vc<Box<dyn Source>>,
    source_ty: WebAssemblySourceType,
}

#[nxpkg_tasks::value_impl]
impl WebAssemblySource {
    #[nxpkg_tasks::function]
    pub fn new(source: Vc<Box<dyn Source>>, source_ty: WebAssemblySourceType) -> Vc<Self> {
        Self::cell(WebAssemblySource { source, source_ty })
    }
}

#[nxpkg_tasks::value_impl]
impl Source for WebAssemblySource {
    #[nxpkg_tasks::function]
    fn ident(&self) -> Vc<AssetIdent> {
        match self.source_ty {
            WebAssemblySourceType::Binary => self.source.ident(),
            WebAssemblySourceType::Text => self
                .source
                .ident()
                .with_path(self.source.ident().path().append("_.wasm".to_string())),
        }
    }
}

#[nxpkg_tasks::value_impl]
impl Asset for WebAssemblySource {
    #[nxpkg_tasks::function]
    async fn content(&self) -> Result<Vc<AssetContent>> {
        let content = match self.source_ty {
            WebAssemblySourceType::Binary => return Ok(self.source.content()),
            WebAssemblySourceType::Text => self.source.content(),
        };

        let content = content.file_content().await?;

        let FileContent::Content(file) = &*content else {
            return Ok(AssetContent::file(FileContent::NotFound.cell()));
        };

        let bytes = file.content().to_bytes()?;
        let parsed = wat::parse_bytes(&bytes)?;

        Ok(AssetContent::file(File::from(&*parsed).into()))
    }
}
