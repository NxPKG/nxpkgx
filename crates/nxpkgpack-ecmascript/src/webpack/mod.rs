use anyhow::Result;
use swc_core::ecma::ast::Lit;
use nxpkg_tasks::{Value, ValueToString, Vc};
use nxpkgpack_core::{
    asset::{Asset, AssetContent},
    file_source::FileSource,
    ident::AssetIdent,
    module::Module,
    reference::{ModuleReference, ModuleReferences},
    reference_type::{CommonJsReferenceSubType, ReferenceType},
    resolve::{
        origin::{ResolveOrigin, ResolveOriginExt},
        parse::Request,
        resolve, AffectingResolvingAssetReference, ModuleResolveResult,
    },
    source::Source,
};

use self::{parse::WebpackRuntime, references::module_references};
use super::resolve::apply_cjs_specific_options;
use crate::EcmascriptInputTransforms;

pub mod parse;
pub(crate) mod references;

#[nxpkg_tasks::function]
fn modifier() -> Vc<String> {
    Vc::cell("webpack".to_string())
}

#[nxpkg_tasks::value]
pub struct WebpackModuleAsset {
    pub source: Vc<Box<dyn Source>>,
    pub runtime: Vc<WebpackRuntime>,
    pub transforms: Vc<EcmascriptInputTransforms>,
}

#[nxpkg_tasks::value_impl]
impl WebpackModuleAsset {
    #[nxpkg_tasks::function]
    pub fn new(
        source: Vc<Box<dyn Source>>,
        runtime: Vc<WebpackRuntime>,
        transforms: Vc<EcmascriptInputTransforms>,
    ) -> Vc<Self> {
        Self::cell(WebpackModuleAsset {
            source,
            runtime,
            transforms,
        })
    }
}

#[nxpkg_tasks::value_impl]
impl Module for WebpackModuleAsset {
    #[nxpkg_tasks::function]
    fn ident(&self) -> Vc<AssetIdent> {
        self.source.ident().with_modifier(modifier())
    }

    #[nxpkg_tasks::function]
    fn references(&self) -> Vc<ModuleReferences> {
        module_references(self.source, self.runtime, self.transforms)
    }
}

#[nxpkg_tasks::value_impl]
impl Asset for WebpackModuleAsset {
    #[nxpkg_tasks::function]
    fn content(&self) -> Vc<AssetContent> {
        self.source.content()
    }
}

#[nxpkg_tasks::value(shared)]
pub struct WebpackChunkAssetReference {
    #[nxpkg_tasks(trace_ignore)]
    pub chunk_id: Lit,
    pub runtime: Vc<WebpackRuntime>,
    pub transforms: Vc<EcmascriptInputTransforms>,
}

#[nxpkg_tasks::value_impl]
impl ModuleReference for WebpackChunkAssetReference {
    #[nxpkg_tasks::function]
    async fn resolve_reference(&self) -> Result<Vc<ModuleResolveResult>> {
        let runtime = self.runtime.await?;
        Ok(match &*runtime {
            WebpackRuntime::Webpack5 {
                chunk_request_expr: _,
                context_path,
            } => {
                // TODO determine filename from chunk_request_expr
                let chunk_id = match &self.chunk_id {
                    Lit::Str(str) => str.value.to_string(),
                    Lit::Num(num) => format!("{num}"),
                    _ => todo!(),
                };
                let filename = format!("./chunks/{}.js", chunk_id);
                let source = Vc::upcast(FileSource::new(context_path.join(filename)));

                ModuleResolveResult::module(Vc::upcast(WebpackModuleAsset::new(
                    source,
                    self.runtime,
                    self.transforms,
                )))
                .into()
            }
            WebpackRuntime::None => ModuleResolveResult::unresolveable().into(),
        })
    }
}

#[nxpkg_tasks::value_impl]
impl ValueToString for WebpackChunkAssetReference {
    #[nxpkg_tasks::function]
    async fn to_string(&self) -> Result<Vc<String>> {
        let chunk_id = match &self.chunk_id {
            Lit::Str(str) => str.value.to_string(),
            Lit::Num(num) => format!("{num}"),
            _ => todo!(),
        };
        Ok(Vc::cell(format!("webpack chunk {}", chunk_id)))
    }
}

#[nxpkg_tasks::value(shared)]
pub struct WebpackEntryAssetReference {
    pub source: Vc<Box<dyn Source>>,
    pub runtime: Vc<WebpackRuntime>,
    pub transforms: Vc<EcmascriptInputTransforms>,
}

#[nxpkg_tasks::value_impl]
impl ModuleReference for WebpackEntryAssetReference {
    #[nxpkg_tasks::function]
    fn resolve_reference(&self) -> Vc<ModuleResolveResult> {
        ModuleResolveResult::module(Vc::upcast(WebpackModuleAsset::new(
            self.source,
            self.runtime,
            self.transforms,
        )))
        .into()
    }
}

#[nxpkg_tasks::value_impl]
impl ValueToString for WebpackEntryAssetReference {
    #[nxpkg_tasks::function]
    async fn to_string(&self) -> Result<Vc<String>> {
        Ok(Vc::cell("webpack entry".to_string()))
    }
}

#[nxpkg_tasks::value(shared)]
pub struct WebpackRuntimeAssetReference {
    pub origin: Vc<Box<dyn ResolveOrigin>>,
    pub request: Vc<Request>,
    pub runtime: Vc<WebpackRuntime>,
    pub transforms: Vc<EcmascriptInputTransforms>,
}

#[nxpkg_tasks::value_impl]
impl ModuleReference for WebpackRuntimeAssetReference {
    #[nxpkg_tasks::function]
    async fn resolve_reference(&self) -> Result<Vc<ModuleResolveResult>> {
        let ty = Value::new(ReferenceType::CommonJs(CommonJsReferenceSubType::Undefined));
        let options = self.origin.resolve_options(ty.clone());

        let options = apply_cjs_specific_options(options);

        let resolved = resolve(
            self.origin.origin_path().parent().resolve().await?,
            self.request,
            options,
        );

        Ok(resolved
            .await?
            .map_module(
                |source| async move {
                    Ok(Vc::upcast(WebpackModuleAsset::new(
                        source,
                        self.runtime,
                        self.transforms,
                    )))
                },
                |r| async move { Ok(Vc::upcast(AffectingResolvingAssetReference::new(r))) },
            )
            .await?
            .cell())
    }
}

#[nxpkg_tasks::value_impl]
impl ValueToString for WebpackRuntimeAssetReference {
    #[nxpkg_tasks::function]
    async fn to_string(&self) -> Result<Vc<String>> {
        Ok(Vc::cell(format!(
            "webpack {}",
            self.request.to_string().await?,
        )))
    }
}
