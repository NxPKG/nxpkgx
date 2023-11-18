use nxpkg_tasks::Vc;
use nxpkg_tasks_fs::FileSystemPath;

use crate::{
    asset::{Asset, AssetContent},
    ident::AssetIdent,
    output::OutputAsset,
};

/// An [OutputAsset] that is created from some passed source code.
#[nxpkg_tasks::value]
pub struct VirtualOutputAsset {
    pub path: Vc<FileSystemPath>,
    pub content: Vc<AssetContent>,
}

#[nxpkg_tasks::value_impl]
impl VirtualOutputAsset {
    #[nxpkg_tasks::function]
    pub fn new(path: Vc<FileSystemPath>, content: Vc<AssetContent>) -> Vc<Self> {
        VirtualOutputAsset { path, content }.cell()
    }
}

#[nxpkg_tasks::value_impl]
impl OutputAsset for VirtualOutputAsset {
    #[nxpkg_tasks::function]
    fn ident(&self) -> Vc<AssetIdent> {
        AssetIdent::from_path(self.path)
    }
}

#[nxpkg_tasks::value_impl]
impl Asset for VirtualOutputAsset {
    #[nxpkg_tasks::function]
    fn content(&self) -> Vc<AssetContent> {
        self.content
    }
}
