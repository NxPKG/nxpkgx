use nxpkg_tasks::Vc;
use nxpkg_tasks_fs::FileSystemPath;
use nxpkgpack_core::issue::{Issue, StyledString};

/// An issue that occurred while resolving the parsing or evaluating the .env.
#[nxpkg_tasks::value(shared)]
pub struct ProcessEnvIssue {
    pub path: Vc<FileSystemPath>,
    pub description: Vc<StyledString>,
}

#[nxpkg_tasks::value_impl]
impl Issue for ProcessEnvIssue {
    #[nxpkg_tasks::function]
    fn title(&self) -> Vc<String> {
        Vc::cell("Error loading dotenv file".to_string())
    }

    #[nxpkg_tasks::function]
    fn category(&self) -> Vc<String> {
        Vc::cell("parse".to_string())
    }

    #[nxpkg_tasks::function]
    fn file_path(&self) -> Vc<FileSystemPath> {
        self.path
    }

    #[nxpkg_tasks::function]
    fn description(&self) -> Vc<StyledString> {
        self.description
    }
}
