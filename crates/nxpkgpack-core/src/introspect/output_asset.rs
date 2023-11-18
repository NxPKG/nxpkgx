use anyhow::Result;
use nxpkg_tasks::{ValueToString, Vc};

use super::{
    utils::{children_from_output_assets, content_to_details},
    Introspectable, IntrospectableChildren,
};
use crate::{asset::Asset, output::OutputAsset};

#[nxpkg_tasks::value]
pub struct IntrospectableOutputAsset(Vc<Box<dyn OutputAsset>>);

#[nxpkg_tasks::value_impl]
impl IntrospectableOutputAsset {
    #[nxpkg_tasks::function]
    pub async fn new(asset: Vc<Box<dyn OutputAsset>>) -> Result<Vc<Box<dyn Introspectable>>> {
        Ok(Vc::try_resolve_sidecast::<Box<dyn Introspectable>>(asset)
            .await?
            .unwrap_or_else(|| Vc::upcast(IntrospectableOutputAsset(asset).cell())))
    }
}

#[nxpkg_tasks::function]
fn ty() -> Vc<String> {
    Vc::cell("output asset".to_string())
}

#[nxpkg_tasks::value_impl]
impl Introspectable for IntrospectableOutputAsset {
    #[nxpkg_tasks::function]
    fn ty(&self) -> Vc<String> {
        ty()
    }

    #[nxpkg_tasks::function]
    fn title(&self) -> Vc<String> {
        self.0.ident().to_string()
    }

    #[nxpkg_tasks::function]
    fn details(&self) -> Vc<String> {
        content_to_details(self.0.content())
    }

    #[nxpkg_tasks::function]
    fn children(&self) -> Vc<IntrospectableChildren> {
        children_from_output_assets(self.0.references())
    }
}
