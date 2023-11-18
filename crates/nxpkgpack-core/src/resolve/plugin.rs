use anyhow::Result;
use nxpkg_tasks::Vc;
use nxpkg_tasks_fs::{glob::Glob, FileSystemPath};

use crate::resolve::{parse::Request, ResolveResultOption};

/// A condition which determines if the hooks of a resolve plugin gets called.
#[nxpkg_tasks::value]
pub struct ResolvePluginCondition {
    root: Vc<FileSystemPath>,
    glob: Vc<Glob>,
}

#[nxpkg_tasks::value_impl]
impl ResolvePluginCondition {
    #[nxpkg_tasks::function]
    pub fn new(root: Vc<FileSystemPath>, glob: Vc<Glob>) -> Vc<Self> {
        ResolvePluginCondition { root, glob }.cell()
    }

    #[nxpkg_tasks::function]
    pub async fn matches(self: Vc<Self>, fs_path: Vc<FileSystemPath>) -> Result<Vc<bool>> {
        let this = self.await?;
        let root = this.root.await?;
        let glob = this.glob.await?;

        let path = fs_path.await?;

        if let Some(path) = root.get_path_to(&path) {
            if glob.execute(path) {
                return Ok(Vc::cell(true));
            }
        }

        Ok(Vc::cell(false))
    }
}

#[nxpkg_tasks::value_trait]
pub trait ResolvePlugin {
    /// A condition which determines if the hooks gets called.
    fn after_resolve_condition(self: Vc<Self>) -> Vc<ResolvePluginCondition>;

    /// This hook gets called when a full filepath has been resolved and the
    /// condition matches. If a value is returned it replaces the resolve
    /// result.
    fn after_resolve(
        self: Vc<Self>,
        fs_path: Vc<FileSystemPath>,
        lookup_path: Vc<FileSystemPath>,
        request: Vc<Request>,
    ) -> Vc<ResolveResultOption>;
}
