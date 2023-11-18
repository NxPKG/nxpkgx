//! TODO(WEB-741) Remove this file once Sass is supported.

use anyhow::Result;
use nxpkg_tasks::Vc;
use nxpkg_tasks_fs::{glob::Glob, FileSystemPath};
use nxpkgpack_core::{
    issue::{Issue, IssueExt, IssueSeverity, StyledString},
    resolve::{
        parse::Request,
        plugin::{ResolvePlugin, ResolvePluginCondition},
        ResolveResultOption,
    },
};

/// Resolve plugins that warns when importing a sass file.
#[nxpkg_tasks::value]
pub(crate) struct UnsupportedSassResolvePlugin {
    root: Vc<FileSystemPath>,
}

#[nxpkg_tasks::value_impl]
impl UnsupportedSassResolvePlugin {
    #[nxpkg_tasks::function]
    pub fn new(root: Vc<FileSystemPath>) -> Vc<Self> {
        UnsupportedSassResolvePlugin { root }.cell()
    }
}

#[nxpkg_tasks::value_impl]
impl ResolvePlugin for UnsupportedSassResolvePlugin {
    #[nxpkg_tasks::function]
    fn after_resolve_condition(&self) -> Vc<ResolvePluginCondition> {
        ResolvePluginCondition::new(self.root.root(), Glob::new("**/*.{sass,scss}".to_string()))
    }

    #[nxpkg_tasks::function]
    async fn after_resolve(
        &self,
        fs_path: Vc<FileSystemPath>,
        lookup_path: Vc<FileSystemPath>,
        request: Vc<Request>,
    ) -> Result<Vc<ResolveResultOption>> {
        let extension = fs_path.extension().await?;
        if ["sass", "scss"].iter().any(|ext| ext == &*extension) {
            UnsupportedSassModuleIssue {
                file_path: lookup_path,
                request,
            }
            .cell()
            .emit();
        }

        Ok(ResolveResultOption::none())
    }
}

#[nxpkg_tasks::value(shared)]
struct UnsupportedSassModuleIssue {
    file_path: Vc<FileSystemPath>,
    request: Vc<Request>,
}

#[nxpkg_tasks::value_impl]
impl Issue for UnsupportedSassModuleIssue {
    #[nxpkg_tasks::function]
    fn severity(&self) -> Vc<IssueSeverity> {
        IssueSeverity::Warning.into()
    }

    #[nxpkg_tasks::function]
    fn category(&self) -> Vc<String> {
        Vc::cell("resolve".to_string())
    }

    #[nxpkg_tasks::function]
    async fn title(&self) -> Result<Vc<String>> {
        Ok(Vc::cell(format!(
            "Unsupported Sass request: {}",
            self.request.await?.request().as_deref().unwrap_or("N/A")
        )))
    }

    #[nxpkg_tasks::function]
    fn file_path(&self) -> Vc<FileSystemPath> {
        self.file_path
    }

    #[nxpkg_tasks::function]
    fn description(&self) -> Vc<StyledString> {
        StyledString::Text("Nxpkgpack does not yet support importing Sass modules.".to_string())
            .cell()
    }
}
