use anyhow::Result;
use nxpkg_tasks::{ValueToString, Vc};

use super::{
    utils::{children_from_module_references, content_to_details},
    Introspectable, IntrospectableChildren,
};
use crate::{asset::Asset, module::Module};

#[nxpkg_tasks::value]
pub struct IntrospectableModule(Vc<Box<dyn Module>>);

#[nxpkg_tasks::value_impl]
impl IntrospectableModule {
    #[nxpkg_tasks::function]
    pub async fn new(asset: Vc<Box<dyn Module>>) -> Result<Vc<Box<dyn Introspectable>>> {
        Ok(Vc::try_resolve_sidecast::<Box<dyn Introspectable>>(asset)
            .await?
            .unwrap_or_else(|| Vc::upcast(IntrospectableModule(asset).cell())))
    }
}

#[nxpkg_tasks::function]
fn ty() -> Vc<String> {
    Vc::cell("asset".to_string())
}

#[nxpkg_tasks::value_impl]
impl Introspectable for IntrospectableModule {
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
        children_from_module_references(self.0.references())
    }
}
