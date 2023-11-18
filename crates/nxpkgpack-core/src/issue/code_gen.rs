use nxpkg_tasks::Vc;
use nxpkg_tasks_fs::FileSystemPath;

use super::{Issue, IssueSeverity, StyledString};

#[nxpkg_tasks::value(shared)]
pub struct CodeGenerationIssue {
    pub severity: Vc<IssueSeverity>,
    pub path: Vc<FileSystemPath>,
    pub title: Vc<String>,
    pub message: Vc<StyledString>,
}

#[nxpkg_tasks::value_impl]
impl Issue for CodeGenerationIssue {
    #[nxpkg_tasks::function]
    fn severity(&self) -> Vc<IssueSeverity> {
        self.severity
    }

    #[nxpkg_tasks::function]
    fn title(&self) -> Vc<String> {
        self.title
    }

    #[nxpkg_tasks::function]
    fn category(&self) -> Vc<String> {
        Vc::cell("code generation".to_string())
    }

    #[nxpkg_tasks::function]
    fn file_path(&self) -> Vc<FileSystemPath> {
        self.path
    }

    #[nxpkg_tasks::function]
    fn description(&self) -> Vc<StyledString> {
        self.message
    }
}
