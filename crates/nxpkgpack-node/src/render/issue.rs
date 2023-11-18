use anyhow::Result;
use nxpkg_tasks::Vc;
use nxpkg_tasks_fs::FileSystemPath;
use nxpkgpack_core::issue::{Issue, StyledString};

#[nxpkg_tasks::value(shared)]
#[derive(Copy, Clone)]
pub struct RenderingIssue {
    pub file_path: Vc<FileSystemPath>,
    pub message: Vc<StyledString>,
    pub status: Option<i32>,
}

#[nxpkg_tasks::value_impl]
impl Issue for RenderingIssue {
    #[nxpkg_tasks::function]
    fn title(&self) -> Vc<String> {
        Vc::cell("Error during SSR Rendering".to_string())
    }

    #[nxpkg_tasks::function]
    fn category(&self) -> Vc<String> {
        Vc::cell("rendering".to_string())
    }

    #[nxpkg_tasks::function]
    fn file_path(&self) -> Vc<FileSystemPath> {
        self.file_path
    }

    #[nxpkg_tasks::function]
    fn description(&self) -> Vc<StyledString> {
        self.message
    }

    #[nxpkg_tasks::function]
    async fn detail(&self) -> Result<Vc<String>> {
        let mut details = vec![];

        if let Some(status) = self.status {
            if status != 0 {
                details.push(format!("Node.js exit code: {status}"));
            }
        }

        Ok(Vc::cell(details.join("\n")))
    }

    // TODO parse stack trace into source location
}
