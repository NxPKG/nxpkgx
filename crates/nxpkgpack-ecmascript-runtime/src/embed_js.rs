use nxpkg_tasks::Vc;
use nxpkg_tasks_fs::{embed_directory, FileContent, FileSystem, FileSystemPath};
use nxpkgpack_core::{code_builder::Code, context::AssetContext};
use nxpkgpack_ecmascript::StaticEcmascriptCode;

#[nxpkg_tasks::function]
pub fn embed_fs() -> Vc<Box<dyn FileSystem>> {
    embed_directory!("nxpkgpack", "$CARGO_MANIFEST_DIR/js/src")
}

#[nxpkg_tasks::function]
pub fn embed_file(path: String) -> Vc<FileContent> {
    embed_fs().root().join(path).read()
}

#[nxpkg_tasks::function]
pub fn embed_file_path(path: String) -> Vc<FileSystemPath> {
    embed_fs().root().join(path)
}

#[nxpkg_tasks::function]
pub fn embed_static_code(asset_context: Vc<Box<dyn AssetContext>>, path: String) -> Vc<Code> {
    StaticEcmascriptCode::new(asset_context, embed_file_path(path)).code()
}
