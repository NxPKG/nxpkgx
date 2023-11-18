use anyhow::Result;
use nxpkg_tasks::Vc;
use nxpkg_tasks_fs::FileSystemPath;

use super::{Issue, IssueSeverity, IssueSource, OptionIssueSource, StyledString};
use crate::ident::AssetIdent;

#[nxpkg_tasks::value(shared)]
pub struct AnalyzeIssue {
    pub severity: Vc<IssueSeverity>,
    pub source_ident: Vc<AssetIdent>,
    pub title: Vc<String>,
    pub message: Vc<StyledString>,
    pub category: Vc<String>,
    pub code: Option<String>,
    pub source: Option<Vc<IssueSource>>,
}

#[nxpkg_tasks::value_impl]
impl Issue for AnalyzeIssue {
    #[nxpkg_tasks::function]
    fn severity(&self) -> Vc<IssueSeverity> {
        self.severity
    }

    #[nxpkg_tasks::function]
    async fn title(&self) -> Result<Vc<String>> {
        Ok(if let Some(code) = self.code.as_ref() {
            Vc::cell(format!("{code} {}", self.title.await?))
        } else {
            self.title
        })
    }

    #[nxpkg_tasks::function]
    fn category(&self) -> Vc<String> {
        self.category
    }

    #[nxpkg_tasks::function]
    fn file_path(&self) -> Vc<FileSystemPath> {
        self.source_ident.path()
    }

    #[nxpkg_tasks::function]
    fn description(&self) -> Vc<StyledString> {
        self.message
    }

    #[nxpkg_tasks::function]
    fn source(&self) -> Vc<OptionIssueSource> {
        Vc::cell(self.source)
    }
}
