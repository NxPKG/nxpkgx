use anyhow::Result;
use swc_core::{
    ecma::ast::{Expr, ExprOrSpread, NewExpr},
    quote,
};
use nxpkg_tasks::{Value, ValueToString, Vc};
use nxpkgpack_core::{
    chunk::{
        ChunkItemExt, ChunkableModule, ChunkableModuleReference, ChunkingType, ChunkingTypeOption,
    },
    environment::Rendering,
    issue::IssueSource,
    reference::ModuleReference,
    reference_type::UrlReferenceSubType,
    resolve::{origin::ResolveOrigin, parse::Request, ModuleResolveResult},
};

use super::base::ReferencedAsset;
use crate::{
    chunk::EcmascriptChunkingContext,
    code_gen::{CodeGenerateable, CodeGeneration},
    create_visitor,
    references::AstPath,
    resolve::{try_to_severity, url_resolve},
    utils::module_id_to_lit,
};

/// Determines how to treat `new URL(...)` rewrites.
/// This allows to construct url depends on the different building context,
/// e.g. SSR, CSR, or Node.js.
#[nxpkg_tasks::value(shared)]
#[derive(Debug, Copy, Clone, PartialOrd, Ord, Hash)]
pub enum UrlRewriteBehavior {
    /// Omits base, resulting in a relative URL.
    Relative,
    /// Uses the full URL, including the base.
    Full,
    /// Do not attempt to rewrite the URL.
    None,
}

/// URL Asset References are injected during code analysis when we find a
/// (staticly analyzable) `new URL("path", import.meta.url)`.
///
/// It's responsible rewriting the `URL` constructor's arguments to allow the
/// referenced file to be imported/fetched/etc.
#[nxpkg_tasks::value]
pub struct UrlAssetReference {
    origin: Vc<Box<dyn ResolveOrigin>>,
    request: Vc<Request>,
    rendering: Vc<Rendering>,
    ast_path: Vc<AstPath>,
    issue_source: Vc<IssueSource>,
    in_try: bool,
    url_rewrite_behavior: Vc<UrlRewriteBehavior>,
}

#[nxpkg_tasks::value_impl]
impl UrlAssetReference {
    #[nxpkg_tasks::function]
    pub fn new(
        origin: Vc<Box<dyn ResolveOrigin>>,
        request: Vc<Request>,
        rendering: Vc<Rendering>,
        ast_path: Vc<AstPath>,
        issue_source: Vc<IssueSource>,
        in_try: bool,
        url_rewrite_behavior: Vc<UrlRewriteBehavior>,
    ) -> Vc<Self> {
        UrlAssetReference {
            origin,
            request,
            rendering,
            ast_path,
            issue_source,
            in_try,
            url_rewrite_behavior,
        }
        .cell()
    }

    #[nxpkg_tasks::function]
    pub(super) async fn get_referenced_asset(self: Vc<Self>) -> Result<Vc<ReferencedAsset>> {
        let this = self.await?;
        Ok(ReferencedAsset::from_resolve_result(
            self.resolve_reference(),
            this.request,
        ))
    }
}

#[nxpkg_tasks::value_impl]
impl ModuleReference for UrlAssetReference {
    #[nxpkg_tasks::function]
    async fn resolve_reference(&self) -> Vc<ModuleResolveResult> {
        url_resolve(
            self.origin,
            self.request,
            Value::new(UrlReferenceSubType::EcmaScriptNewUrl),
            self.issue_source,
            try_to_severity(self.in_try),
        )
    }
}

#[nxpkg_tasks::value_impl]
impl ValueToString for UrlAssetReference {
    #[nxpkg_tasks::function]
    async fn to_string(&self) -> Result<Vc<String>> {
        Ok(Vc::cell(format!(
            "new URL({})",
            self.request.to_string().await?,
        )))
    }
}

#[nxpkg_tasks::value_impl]
impl ChunkableModuleReference for UrlAssetReference {
    #[nxpkg_tasks::function]
    fn chunking_type(&self) -> Vc<ChunkingTypeOption> {
        Vc::cell(Some(ChunkingType::Parallel))
    }
}

#[nxpkg_tasks::value_impl]
impl CodeGenerateable for UrlAssetReference {
    /// Rewrites call to the `new URL()` ctor depends on the current
    /// conditions. Generated code will point to the output path of the asset,
    /// as similar to the webpack's behavior. This is based on the
    /// configuration (UrlRewriteBehavior), and the current context
    /// (rendering), lastly the asset's condition (if it's referenced /
    /// external). The following table shows the behavior:
    /*
     * original call: `new URL(url, base);`
    ┌───────────────────────────────┬─────────────────────────────────────────────────────────────────────────┬────────────────────────────────────────────────┬───────────────────────┐
    │  UrlRewriteBehavior\RefAsset  │                         ReferencedAsset::Some()                         │           ReferencedAsset::External            │ ReferencedAsset::None │
    ├───────────────────────────────┼─────────────────────────────────────────────────────────────────────────┼────────────────────────────────────────────────┼───────────────────────┤
    │ Relative                      │ __nxpkgpack_relative_url__(__nxpkgpack_require__(urlId))                │ __nxpkgpack_relative_url__(url)                │ new URL(url, base)    │
    │ Full(RenderingClient::Client) │ new URL(__nxpkgpack_require__(urlId), location.origin)                  │ new URL(url, location.origin)                  │ new URL(url, base)    │
    │ Full(RenderingClient::..)     │ new URL(__nxpkgpack_resolve_module_id_path__(urlId))                    │ new URL(url, base)                             │ new URL(url, base)    │
    │ None                          │ new URL(url, base)                                                      │ new URL(url, base)                             │ new URL(url, base)    │
    └───────────────────────────────┴─────────────────────────────────────────────────────────────────────────┴────────────────────────────────────────────────┴───────────────────────┘
    */
    #[nxpkg_tasks::function]
    async fn code_generation(
        self: Vc<Self>,
        chunking_context: Vc<Box<dyn EcmascriptChunkingContext>>,
    ) -> Result<Vc<CodeGeneration>> {
        let this = self.await?;
        let mut visitors = vec![];
        let rewrite_behavior = &*this.url_rewrite_behavior.await?;

        match rewrite_behavior {
            UrlRewriteBehavior::Relative => {
                let referenced_asset = self.get_referenced_asset().await?;
                let ast_path = this.ast_path.await?;

                // if the referenced url is in the module graph of nxpkgpack, replace it into
                // the chunk item will be emitted into output path to point the
                // static asset path. for the `new URL()` call, replace it into
                // pseudo url object `__nxpkgpack_relative_url__`
                // which is injected by nxpkgpack's runtime to resolve into the relative path
                // omitting the base.
                match &*referenced_asset {
                    ReferencedAsset::Some(asset) => {
                        // We rewrite the first `new URL()` arguments to be a require() of the chunk
                        // item, which exports the static asset path to the linked file.
                        let id = asset
                            .as_chunk_item(Vc::upcast(chunking_context))
                            .id()
                            .await?;

                        visitors.push(create_visitor!(ast_path, visit_mut_expr(new_expr: &mut Expr) {
                            if let Expr::New(NewExpr { args: Some(args), callee, .. }) = new_expr {
                                *callee = Box::new(quote!("__nxpkgpack_relative_url__" as Expr));
                                if let Some(ExprOrSpread { box expr, spread: None }) = args.get_mut(0) {
                                    *expr = quote!(
                                        "__nxpkgpack_require__($id)" as Expr,
                                        id: Expr = module_id_to_lit(&id),
                                    );
                                }
                            }
                        }));
                    }
                    ReferencedAsset::OriginalReferenceTypeExternal(request) => {
                        let request = request.to_string();
                        visitors.push(create_visitor!(ast_path, visit_mut_expr(new_expr: &mut Expr) {
                            if let Expr::New(NewExpr { args: Some(args), callee, .. }) = new_expr {
                                *callee = Box::new(quote!("__nxpkgpack_relative_url__" as Expr));
                                if let Some(ExprOrSpread { box expr, spread: None }) = args.get_mut(0) {
                                    *expr = request.as_str().into()
                                }
                            }
                        }));
                    }
                    ReferencedAsset::None => {}
                }
            }
            UrlRewriteBehavior::Full => {
                let referenced_asset = self.get_referenced_asset().await?;
                let ast_path = this.ast_path.await?;

                // For rendering environments (CSR), we rewrite the `import.meta.url` to
                // be a location.origin because it allows us to access files from the root of
                // the dev server.
                //
                // By default for the remaining environments, nxpkgpack's runtime have overriden
                // `import.meta.url`.
                let rewrite_url_base = match &*this.rendering.await? {
                    Rendering::Client => Some(quote!("location.origin" as Expr)),
                    Rendering::None | Rendering::Server(..) => None,
                };

                match &*referenced_asset {
                    ReferencedAsset::Some(asset) => {
                        // We rewrite the first `new URL()` arguments to be a require() of the
                        // chunk item, which returns the asset path as its exports.
                        let id = asset
                            .as_chunk_item(Vc::upcast(chunking_context))
                            .id()
                            .await?;

                        // If there's a rewrite to the base url, then the current rendering
                        // environment should able to resolve the asset path
                        // (asset_url) from the base. Wrap the module id
                        // with __nxpkgpack_require_ which returns the asset_url.
                        //
                        // Otherwise, the envioronment should provide an absolute path to the actual
                        // output asset; delegate those calculation to the
                        // runtime fn __nxpkgpack_resolve_module_id_path__.
                        let url_segment_resolver = if rewrite_url_base.is_some() {
                            quote!(
                                "__nxpkgpack_require__($id)" as Expr,
                                id: Expr = module_id_to_lit(&id),
                            )
                        } else {
                            quote!(
                                "__nxpkgpack_resolve_module_id_path__($id)" as Expr,
                                id: Expr = module_id_to_lit(&id),
                            )
                        };

                        visitors.push(create_visitor!(ast_path, visit_mut_expr(new_expr: &mut Expr) {
                            if let Expr::New(NewExpr { args: Some(args), .. }) = new_expr {
                                if let Some(ExprOrSpread { box expr, spread: None }) = args.get_mut(0) {
                                    *expr = url_segment_resolver.clone();
                                }

                                if let Some(ExprOrSpread { box expr, spread: None }) = args.get_mut(1) {
                                    if let Some(rewrite) = &rewrite_url_base {
                                        *expr = rewrite.clone();
                                    } else {
                                        // If rewrite for the base doesn't exists, means __nxpkgpack_resolve_module_id_path__
                                        // should resolve the full path correctly and there shouldn't be a base.
                                        args.remove(1);
                                    }
                                }
                            }
                        }));
                    }
                    ReferencedAsset::OriginalReferenceTypeExternal(request) => {
                        let request = request.to_string();
                        visitors.push(create_visitor!(ast_path, visit_mut_expr(new_expr: &mut Expr) {
                            if let Expr::New(NewExpr { args: Some(args), .. }) = new_expr {
                                if let Some(ExprOrSpread { box expr, spread: None }) = args.get_mut(0) {
                                    *expr = request.as_str().into()
                                }

                                if let Some(rewrite) = &rewrite_url_base {
                                    if let Some(ExprOrSpread { box expr, spread: None }) = args.get_mut(1) {
                                        *expr = rewrite.clone();
                                    }
                                }
                            }
                        }));
                    }
                    ReferencedAsset::None => {}
                }
            }
            UrlRewriteBehavior::None => {
                // Asked to not rewrite the URL, so we don't do anything.
            }
        };

        Ok(CodeGeneration { visitors }.into())
    }
}
