use anyhow::Result;
use nxpkg_tasks::{Value, ValueToString, Vc};
use nxpkg_tasks_fs::FileSystemPath;
use nxpkgpack_core::{
    context::AssetContext,
    file_source::FileSource,
    reference::ModuleReference,
    reference_type::{ReferenceType, TypeScriptReferenceSubType},
    resolve::{origin::ResolveOrigin, parse::Request, ModuleResolveResult},
};

use crate::typescript::{resolve::type_resolve, TsConfigModuleAsset};

#[nxpkg_tasks::value]
#[derive(Hash, Clone, Debug)]
pub struct TsConfigReference {
    pub tsconfig: Vc<FileSystemPath>,
    pub origin: Vc<Box<dyn ResolveOrigin>>,
}

#[nxpkg_tasks::value_impl]
impl TsConfigReference {
    #[nxpkg_tasks::function]
    pub fn new(origin: Vc<Box<dyn ResolveOrigin>>, tsconfig: Vc<FileSystemPath>) -> Vc<Self> {
        Self::cell(TsConfigReference { tsconfig, origin })
    }
}

#[nxpkg_tasks::value_impl]
impl ModuleReference for TsConfigReference {
    #[nxpkg_tasks::function]
    fn resolve_reference(&self) -> Vc<ModuleResolveResult> {
        ModuleResolveResult::module(Vc::upcast(TsConfigModuleAsset::new(
            self.origin,
            Vc::upcast(FileSource::new(self.tsconfig)),
        )))
        .into()
    }
}

#[nxpkg_tasks::value_impl]
impl ValueToString for TsConfigReference {
    #[nxpkg_tasks::function]
    async fn to_string(&self) -> Result<Vc<String>> {
        Ok(Vc::cell(format!(
            "tsconfig {}",
            self.tsconfig.to_string().await?,
        )))
    }
}

#[nxpkg_tasks::value]
#[derive(Hash, Debug)]
pub struct TsReferencePathAssetReference {
    pub origin: Vc<Box<dyn ResolveOrigin>>,
    pub path: String,
}

#[nxpkg_tasks::value_impl]
impl TsReferencePathAssetReference {
    #[nxpkg_tasks::function]
    pub fn new(origin: Vc<Box<dyn ResolveOrigin>>, path: String) -> Vc<Self> {
        Self::cell(TsReferencePathAssetReference { origin, path })
    }
}

#[nxpkg_tasks::value_impl]
impl ModuleReference for TsReferencePathAssetReference {
    #[nxpkg_tasks::function]
    async fn resolve_reference(&self) -> Result<Vc<ModuleResolveResult>> {
        Ok(
            if let Some(path) = &*self
                .origin
                .origin_path()
                .parent()
                .try_join(self.path.clone())
                .await?
            {
                ModuleResolveResult::module(Vc::upcast(self.origin.asset_context().process(
                    Vc::upcast(FileSource::new(*path)),
                    Value::new(ReferenceType::TypeScript(
                        TypeScriptReferenceSubType::Undefined,
                    )),
                )))
                .cell()
            } else {
                ModuleResolveResult::unresolveable().cell()
            },
        )
    }
}

#[nxpkg_tasks::value_impl]
impl ValueToString for TsReferencePathAssetReference {
    #[nxpkg_tasks::function]
    async fn to_string(&self) -> Result<Vc<String>> {
        Ok(Vc::cell(format!(
            "typescript reference path comment {}",
            self.path,
        )))
    }
}

#[nxpkg_tasks::value]
#[derive(Hash, Debug)]
pub struct TsReferenceTypeAssetReference {
    pub origin: Vc<Box<dyn ResolveOrigin>>,
    pub module: String,
}

#[nxpkg_tasks::value_impl]
impl TsReferenceTypeAssetReference {
    #[nxpkg_tasks::function]
    pub fn new(origin: Vc<Box<dyn ResolveOrigin>>, module: String) -> Vc<Self> {
        Self::cell(TsReferenceTypeAssetReference { origin, module })
    }
}

#[nxpkg_tasks::value_impl]
impl ModuleReference for TsReferenceTypeAssetReference {
    #[nxpkg_tasks::function]
    fn resolve_reference(&self) -> Vc<ModuleResolveResult> {
        type_resolve(
            self.origin,
            Request::module(
                self.module.clone(),
                Value::new("".to_string().into()),
                Vc::<String>::default(),
            ),
        )
    }
}

#[nxpkg_tasks::value_impl]
impl ValueToString for TsReferenceTypeAssetReference {
    #[nxpkg_tasks::function]
    async fn to_string(&self) -> Result<Vc<String>> {
        Ok(Vc::cell(format!(
            "typescript reference type comment {}",
            self.module,
        )))
    }
}
