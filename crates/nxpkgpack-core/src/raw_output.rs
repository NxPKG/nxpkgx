use nxpkg_tasks::Vc;

use crate::{
    asset::{Asset, AssetContent},
    ident::AssetIdent,
    output::OutputAsset,
    source::Source,
};

/// A module where source code doesn't need to be parsed but can be used as is.
/// This module has no references to other modules.
#[nxpkg_tasks::value]
pub struct RawOutput {
    source: Vc<Box<dyn Source>>,
}

#[nxpkg_tasks::value_impl]
impl OutputAsset for RawOutput {
    #[nxpkg_tasks::function]
    fn ident(&self) -> Vc<AssetIdent> {
        self.source.ident()
    }
}

#[nxpkg_tasks::value_impl]
impl Asset for RawOutput {
    #[nxpkg_tasks::function]
    fn content(&self) -> Vc<AssetContent> {
        self.source.content()
    }
}

#[nxpkg_tasks::value_impl]
impl RawOutput {
    #[nxpkg_tasks::function]
    pub fn new(source: Vc<Box<dyn Source>>) -> Vc<RawOutput> {
        RawOutput { source }.cell()
    }
}
