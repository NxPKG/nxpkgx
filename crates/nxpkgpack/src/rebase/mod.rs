use std::hash::Hash;

use anyhow::Result;
use nxpkg_tasks::Vc;
use nxpkg_tasks_fs::FileSystemPath;
use nxpkgpack_core::{
    asset::{Asset, AssetContent},
    ident::AssetIdent,
    module::Module,
    output::{OutputAsset, OutputAssets},
    reference::all_referenced_modules,
};

/// Converts a [Module] graph into an [OutputAsset] graph by placing it into a
/// different directory.
#[nxpkg_tasks::value]
#[derive(Hash)]
pub struct RebasedAsset {
    source: Vc<Box<dyn Module>>,
    input_dir: Vc<FileSystemPath>,
    output_dir: Vc<FileSystemPath>,
}

#[nxpkg_tasks::value_impl]
impl RebasedAsset {
    #[nxpkg_tasks::function]
    pub fn new(
        source: Vc<Box<dyn Module>>,
        input_dir: Vc<FileSystemPath>,
        output_dir: Vc<FileSystemPath>,
    ) -> Vc<Self> {
        Self::cell(RebasedAsset {
            source,
            input_dir,
            output_dir,
        })
    }
}

#[nxpkg_tasks::value_impl]
impl OutputAsset for RebasedAsset {
    #[nxpkg_tasks::function]
    fn ident(&self) -> Vc<AssetIdent> {
        AssetIdent::from_path(FileSystemPath::rebase(
            self.source.ident().path(),
            self.input_dir,
            self.output_dir,
        ))
    }

    #[nxpkg_tasks::function]
    async fn references(&self) -> Result<Vc<OutputAssets>> {
        let mut references = Vec::new();
        for &module in all_referenced_modules(self.source).await?.iter() {
            references.push(Vc::upcast(RebasedAsset::new(
                module,
                self.input_dir,
                self.output_dir,
            )));
        }
        Ok(Vc::cell(references))
    }
}

#[nxpkg_tasks::value_impl]
impl Asset for RebasedAsset {
    #[nxpkg_tasks::function]
    fn content(&self) -> Vc<AssetContent> {
        self.source.content()
    }
}
