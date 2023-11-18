use anyhow::Result;
use nxpkg_tasks::Vc;
use nxpkg_tasks_fs::FileSystemPath;

use super::{Issue, IssueSeverity, StyledString};

#[nxpkg_tasks::value(shared)]
pub struct UnsupportedModuleIssue {
    pub file_path: Vc<FileSystemPath>,
    pub package: String,
    pub package_path: Option<String>,
}

#[nxpkg_tasks::value_impl]
impl Issue for UnsupportedModuleIssue {
    #[nxpkg_tasks::function]
    fn severity(&self) -> Vc<IssueSeverity> {
        IssueSeverity::Warning.into()
    }

    #[nxpkg_tasks::function]
    fn category(&self) -> Vc<String> {
        Vc::cell("resolve".to_string())
    }

    #[nxpkg_tasks::function]
    fn title(&self) -> Vc<String> {
        Vc::cell("Unsupported module".into())
    }

    #[nxpkg_tasks::function]
    fn file_path(&self) -> Vc<FileSystemPath> {
        self.file_path
    }

    #[nxpkg_tasks::function]
    async fn description(&self) -> Result<Vc<StyledString>> {
        Ok(StyledString::Text(match &self.package_path {
            Some(path) => format!("The module {}{} is not yet supported", self.package, path),
            None => format!("The package {} is not yet supported", self.package),
        })
        .cell())
    }
}
