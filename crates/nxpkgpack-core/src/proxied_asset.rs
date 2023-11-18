use nxpkg_tasks::Vc;
use nxpkg_tasks_fs::FileSystemPath;

use crate::{
    asset::{Asset, AssetContent},
    ident::AssetIdent,
    output::{OutputAsset, OutputAssets},
    version::VersionedContent,
};

/// An [`Asset`] with an overwritten path. This is helpful to expose an asset at
/// a different path than it was originally set up to be, e.g. to expose layout
/// CSS chunks under the server FS instead of the output FS when rendering
/// Next.js apps.
#[nxpkg_tasks::value]
pub struct ProxiedAsset {
    asset: Vc<Box<dyn OutputAsset>>,
    path: Vc<FileSystemPath>,
}

#[nxpkg_tasks::value_impl]
impl ProxiedAsset {
    /// Creates a new [`ProxiedAsset`] from an [`Asset`] and a path.
    #[nxpkg_tasks::function]
    pub fn new(asset: Vc<Box<dyn OutputAsset>>, path: Vc<FileSystemPath>) -> Vc<Self> {
        ProxiedAsset { asset, path }.cell()
    }
}

#[nxpkg_tasks::value_impl]
impl OutputAsset for ProxiedAsset {
    #[nxpkg_tasks::function]
    fn ident(&self) -> Vc<AssetIdent> {
        AssetIdent::from_path(self.path)
    }

    #[nxpkg_tasks::function]
    fn references(&self) -> Vc<OutputAssets> {
        self.asset.references()
    }
}

#[nxpkg_tasks::value_impl]
impl Asset for ProxiedAsset {
    #[nxpkg_tasks::function]
    fn content(&self) -> Vc<AssetContent> {
        self.asset.content()
    }

    #[nxpkg_tasks::function]
    fn versioned_content(&self) -> Vc<Box<dyn VersionedContent>> {
        self.asset.versioned_content()
    }
}
