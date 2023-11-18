use nxpkg_tasks::Vc;
use nxpkg_tasks_fs::{embed_directory, FileContent, FileSystem, FileSystemPath};

#[nxpkg_tasks::function]
fn embed_fs() -> Vc<Box<dyn FileSystem>> {
    embed_directory!("nxpkgpack-cli", "$CARGO_MANIFEST_DIR/js/src")
}

#[nxpkg_tasks::function]
pub(crate) fn embed_file(path: String) -> Vc<FileContent> {
    embed_fs().root().join(path).read()
}

#[nxpkg_tasks::function]
pub(crate) fn embed_file_path(path: String) -> Vc<FileSystemPath> {
    embed_fs().root().join(path)
}
