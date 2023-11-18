use anyhow::Result;
use nxpkg_tasks::{Value, Vc};
use nxpkg_tasks_fs::FileSystemPath;
use nxpkgpack_core::{
    introspect::{Introspectable, IntrospectableChildren},
    issue::IssueDescriptionExt,
};

use super::{
    route_tree::{MapGetContentSourceContent, RouteTree},
    ContentSource, ContentSourceContent, ContentSourceData, ContentSourceDataVary, ContentSources,
    GetContentSourceContent,
};

#[nxpkg_tasks::value]
pub struct IssueFilePathContentSource {
    file_path: Option<Vc<FileSystemPath>>,
    description: String,
    source: Vc<Box<dyn ContentSource>>,
}

#[nxpkg_tasks::value_impl]
impl IssueFilePathContentSource {
    #[nxpkg_tasks::function]
    pub fn new_file_path(
        file_path: Vc<FileSystemPath>,
        description: String,
        source: Vc<Box<dyn ContentSource>>,
    ) -> Vc<Self> {
        IssueFilePathContentSource {
            file_path: Some(file_path),
            description,
            source,
        }
        .cell()
    }

    #[nxpkg_tasks::function]
    pub fn new_description(description: String, source: Vc<Box<dyn ContentSource>>) -> Vc<Self> {
        IssueFilePathContentSource {
            file_path: None,
            description,
            source,
        }
        .cell()
    }
}

#[nxpkg_tasks::value_impl]
impl ContentSource for IssueFilePathContentSource {
    #[nxpkg_tasks::function]
    async fn get_routes(self: Vc<Self>) -> Result<Vc<RouteTree>> {
        let this = self.await?;
        let routes = this
            .source
            .get_routes()
            .issue_file_path(this.file_path, &this.description)
            .await?;
        Ok(routes.map_routes(Vc::upcast(
            IssueContextContentSourceMapper { source: self }.cell(),
        )))
    }

    #[nxpkg_tasks::function]
    fn get_children(&self) -> Vc<ContentSources> {
        Vc::cell(vec![self.source])
    }
}

#[nxpkg_tasks::value]
struct IssueContextContentSourceMapper {
    source: Vc<IssueFilePathContentSource>,
}

#[nxpkg_tasks::value_impl]
impl MapGetContentSourceContent for IssueContextContentSourceMapper {
    #[nxpkg_tasks::function]
    fn map_get_content(
        &self,
        get_content: Vc<Box<dyn GetContentSourceContent>>,
    ) -> Vc<Box<dyn GetContentSourceContent>> {
        Vc::upcast(
            IssueContextGetContentSourceContent {
                get_content,
                source: self.source,
            }
            .cell(),
        )
    }
}

#[nxpkg_tasks::value]
struct IssueContextGetContentSourceContent {
    get_content: Vc<Box<dyn GetContentSourceContent>>,
    source: Vc<IssueFilePathContentSource>,
}

#[nxpkg_tasks::value_impl]
impl GetContentSourceContent for IssueContextGetContentSourceContent {
    #[nxpkg_tasks::function]
    async fn vary(&self) -> Result<Vc<ContentSourceDataVary>> {
        let source = self.source.await?;
        let result = self
            .get_content
            .vary()
            .issue_file_path(source.file_path, &source.description)
            .await?;
        Ok(result)
    }

    #[nxpkg_tasks::function]
    async fn get(
        &self,
        path: String,
        data: Value<ContentSourceData>,
    ) -> Result<Vc<ContentSourceContent>> {
        let source = self.source.await?;
        let result = self
            .get_content
            .get(path, data)
            .issue_file_path(source.file_path, &source.description)
            .await?;
        Ok(result)
    }
}

#[nxpkg_tasks::value_impl]
impl Introspectable for IssueFilePathContentSource {
    #[nxpkg_tasks::function]
    async fn ty(&self) -> Result<Vc<String>> {
        Ok(
            if let Some(source) =
                Vc::try_resolve_sidecast::<Box<dyn Introspectable>>(self.source).await?
            {
                source.ty()
            } else {
                Vc::cell("IssueContextContentSource".to_string())
            },
        )
    }

    #[nxpkg_tasks::function]
    async fn title(&self) -> Result<Vc<String>> {
        Ok(
            if let Some(source) =
                Vc::try_resolve_sidecast::<Box<dyn Introspectable>>(self.source).await?
            {
                let title = source.title().await?;
                Vc::cell(format!("{}: {}", self.description, title))
            } else {
                Vc::cell(self.description.clone())
            },
        )
    }

    #[nxpkg_tasks::function]
    async fn details(&self) -> Result<Vc<String>> {
        Ok(
            if let Some(source) =
                Vc::try_resolve_sidecast::<Box<dyn Introspectable>>(self.source).await?
            {
                source.details()
            } else {
                Vc::cell(String::new())
            },
        )
    }

    #[nxpkg_tasks::function]
    async fn children(&self) -> Result<Vc<IntrospectableChildren>> {
        Ok(
            if let Some(source) =
                Vc::try_resolve_sidecast::<Box<dyn Introspectable>>(self.source).await?
            {
                source.children()
            } else {
                Vc::cell(Default::default())
            },
        )
    }
}
