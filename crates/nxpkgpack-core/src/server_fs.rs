use anyhow::{bail, Result};
use nxpkg_tasks::{Completion, ValueToString, Vc};
use nxpkg_tasks_fs::{
    DirectoryContent, FileContent, FileMeta, FileSystem, FileSystemPath, LinkContent,
};

#[nxpkg_tasks::value]
pub struct ServerFileSystem {}

#[nxpkg_tasks::value_impl]
impl ServerFileSystem {
    #[nxpkg_tasks::function]
    pub fn new() -> Vc<Self> {
        Self::cell(ServerFileSystem {})
    }
}

#[nxpkg_tasks::value_impl]
impl FileSystem for ServerFileSystem {
    #[nxpkg_tasks::function]
    fn read(&self, _fs_path: Vc<FileSystemPath>) -> Result<Vc<FileContent>> {
        bail!("Reading is not possible from the marker filesystem for the server")
    }

    #[nxpkg_tasks::function]
    fn read_link(&self, _fs_path: Vc<FileSystemPath>) -> Result<Vc<LinkContent>> {
        bail!("Reading is not possible from the marker filesystem for the server")
    }

    #[nxpkg_tasks::function]
    fn read_dir(&self, _fs_path: Vc<FileSystemPath>) -> Result<Vc<DirectoryContent>> {
        bail!("Reading is not possible from the marker filesystem for the server")
    }

    #[nxpkg_tasks::function]
    fn track(&self, _fs_path: Vc<FileSystemPath>) -> Result<Vc<Completion>> {
        bail!("Tracking is not possible to the marker filesystem for the server")
    }

    #[nxpkg_tasks::function]
    fn write(
        &self,
        _fs_path: Vc<FileSystemPath>,
        _content: Vc<FileContent>,
    ) -> Result<Vc<Completion>> {
        bail!("Writing is not possible to the marker filesystem for the server")
    }

    #[nxpkg_tasks::function]
    fn write_link(
        &self,
        _fs_path: Vc<FileSystemPath>,
        _target: Vc<LinkContent>,
    ) -> Result<Vc<Completion>> {
        bail!("Writing is not possible to the marker filesystem for the  server")
    }

    #[nxpkg_tasks::function]
    fn metadata(&self, _fs_path: Vc<FileSystemPath>) -> Result<Vc<FileMeta>> {
        bail!("Reading is not possible from the marker filesystem for the  server")
    }
}

#[nxpkg_tasks::value_impl]
impl ValueToString for ServerFileSystem {
    #[nxpkg_tasks::function]
    fn to_string(&self) -> Vc<String> {
        Vc::cell("root of the server".to_string())
    }
}
