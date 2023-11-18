use anyhow::Result;
use nxpkg_tasks::Vc;
use nxpkg_tasks_fs::FileSystemPath;
use nxpkgpack_core::{
    asset::{Asset, AssetContent},
    ident::AssetIdent,
    output::OutputAsset,
    source::Source,
};

/// A static asset that is served at a fixed output path. It won't use
/// content hashing to generate a long term cacheable URL.
#[nxpkg_tasks::value]
pub struct FixedStaticAsset {
    output_path: Vc<FileSystemPath>,
    source: Vc<Box<dyn Source>>,
}

#[nxpkg_tasks::value_impl]
impl FixedStaticAsset {
    #[nxpkg_tasks::function]
    pub fn new(output_path: Vc<FileSystemPath>, source: Vc<Box<dyn Source>>) -> Vc<Self> {
        FixedStaticAsset {
            output_path,
            source,
        }
        .cell()
    }
}

#[nxpkg_tasks::value_impl]
impl OutputAsset for FixedStaticAsset {
    #[nxpkg_tasks::function]
    async fn ident(&self) -> Result<Vc<AssetIdent>> {
        Ok(AssetIdent::from_path(self.output_path))
    }
}

#[nxpkg_tasks::value_impl]
impl Asset for FixedStaticAsset {
    #[nxpkg_tasks::function]
    fn content(&self) -> Vc<AssetContent> {
        self.source.content()
    }
}
