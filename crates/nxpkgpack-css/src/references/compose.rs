use anyhow::Result;
use nxpkg_tasks::{Value, ValueToString, Vc};
use nxpkgpack_core::{
    chunk::ChunkableModuleReference,
    reference::ModuleReference,
    reference_type::CssReferenceSubType,
    resolve::{origin::ResolveOrigin, parse::Request, ModuleResolveResult},
};

use crate::references::css_resolve;

/// A `composes: ... from ...` CSS module reference.
#[nxpkg_tasks::value]
#[derive(Hash, Debug)]
pub struct CssModuleComposeReference {
    pub origin: Vc<Box<dyn ResolveOrigin>>,
    pub request: Vc<Request>,
}

#[nxpkg_tasks::value_impl]
impl CssModuleComposeReference {
    /// Creates a new [`CssModuleComposeReference`].
    #[nxpkg_tasks::function]
    pub fn new(origin: Vc<Box<dyn ResolveOrigin>>, request: Vc<Request>) -> Vc<Self> {
        Self::cell(CssModuleComposeReference { origin, request })
    }
}

#[nxpkg_tasks::value_impl]
impl ModuleReference for CssModuleComposeReference {
    #[nxpkg_tasks::function]
    fn resolve_reference(&self) -> Vc<ModuleResolveResult> {
        css_resolve(
            self.origin,
            self.request,
            Value::new(CssReferenceSubType::Compose),
            // TODO: add real issue source, currently impossible because `CssClassName` doesn't
            // contain the source span
            // https://docs.rs/swc_css_modules/0.21.16/swc_css_modules/enum.CssClassName.html
            None,
        )
    }
}

#[nxpkg_tasks::value_impl]
impl ValueToString for CssModuleComposeReference {
    #[nxpkg_tasks::function]
    async fn to_string(&self) -> Result<Vc<String>> {
        Ok(Vc::cell(format!(
            "compose(url) {}",
            self.request.to_string().await?,
        )))
    }
}

#[nxpkg_tasks::value_impl]
impl ChunkableModuleReference for CssModuleComposeReference {}
