use std::{borrow::Cow, collections::HashSet, fmt::Display};

use anyhow::Result;
use nxpkg_tasks::{ReadRef, TryJoinIterExt, Vc};
use nxpkg_tasks_fs::{json::parse_json_with_source_context, File};
use nxpkgpack_core::{
    asset::AssetContent,
    introspect::{Introspectable, IntrospectableChildren},
    version::VersionedContentExt,
};
use nxpkgpack_ecmascript::utils::FormatIter;

use crate::source::{
    route_tree::{RouteTree, RouteTrees, RouteType},
    ContentSource, ContentSourceContent, ContentSourceData, GetContentSourceContent,
};

#[nxpkg_tasks::value(shared)]
pub struct IntrospectionSource {
    pub roots: HashSet<Vc<Box<dyn Introspectable>>>,
}

#[nxpkg_tasks::value_impl]
impl Introspectable for IntrospectionSource {
    #[nxpkg_tasks::function]
    fn ty(&self) -> Vc<String> {
        Vc::cell("introspection-source".to_string())
    }

    #[nxpkg_tasks::function]
    fn title(&self) -> Vc<String> {
        Vc::cell("introspection-source".to_string())
    }

    #[nxpkg_tasks::function]
    fn children(&self) -> Vc<IntrospectableChildren> {
        let name = Vc::cell("root".to_string());
        Vc::cell(self.roots.iter().map(|root| (name, *root)).collect())
    }
}

struct HtmlEscaped<T>(T);

impl<T: Display> Display for HtmlEscaped<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            &self
                .0
                .to_string()
                // TODO this is pretty inefficient
                .replace('&', "&amp;")
                .replace('>', "&gt;")
                .replace('<', "&lt;"),
        )
    }
}

struct HtmlStringEscaped<T>(T);

impl<T: Display> Display for HtmlStringEscaped<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            &self
                .0
                .to_string()
                // TODO this is pretty inefficient
                .replace('&', "&amp;")
                .replace('"', "&quot;")
                .replace('>', "&gt;")
                .replace('<', "&lt;"),
        )
    }
}

#[nxpkg_tasks::value_impl]
impl ContentSource for IntrospectionSource {
    #[nxpkg_tasks::function]
    fn get_routes(self: Vc<Self>) -> Vc<RouteTree> {
        Vc::<RouteTrees>::cell(vec![
            RouteTree::new_route(Vec::new(), RouteType::Exact, Vc::upcast(self)),
            RouteTree::new_route(Vec::new(), RouteType::CatchAll, Vc::upcast(self)),
        ])
        .merge()
    }
}

#[nxpkg_tasks::value_impl]
impl GetContentSourceContent for IntrospectionSource {
    #[nxpkg_tasks::function]
    async fn get(
        self: Vc<Self>,
        path: String,
        _data: nxpkg_tasks::Value<ContentSourceData>,
    ) -> Result<Vc<ContentSourceContent>> {
        // get last segment
        let path = &path[path.rfind('/').unwrap_or(0) + 1..];
        let introspectable = if path.is_empty() {
            let roots = &self.await?.roots;
            if roots.len() == 1 {
                *roots.iter().next().unwrap()
            } else {
                Vc::upcast(self)
            }
        } else {
            parse_json_with_source_context(path)?
        };
        let internal_ty = Vc::debug_identifier(introspectable).await?;
        fn str_or_err(s: &Result<ReadRef<String>>) -> Cow<'_, str> {
            s.as_ref().map_or_else(
                |e| Cow::<'_, str>::Owned(format!("ERROR: {:?}", e)),
                |d| Cow::Borrowed(&**d),
            )
        }
        let ty = introspectable.ty().await;
        let ty = str_or_err(&ty);
        let title = introspectable.title().await;
        let title = str_or_err(&title);
        let details = introspectable.details().await;
        let details = str_or_err(&details);
        let children = introspectable.children().await?;
        let has_children = !children.is_empty();
        let children = children
            .iter()
            .map(|&(name, child)| async move {
                let name = name.await;
                let name = str_or_err(&name);
                let ty = child.ty().await;
                let ty = str_or_err(&ty);
                let title = child.title().await;
                let title = str_or_err(&title);
                let path = serde_json::to_string(&child)?;
                Ok(format!(
                    "<li>{name} <!-- {title} --><a href=\"./{path}\">[{ty}] {title}</a></li>",
                    name = HtmlEscaped(name),
                    title = HtmlEscaped(title),
                    path = HtmlStringEscaped(urlencoding::encode(&path)),
                    ty = HtmlEscaped(ty),
                ))
            })
            .try_join()
            .await?;
        let details = if details.is_empty() {
            String::new()
        } else if has_children {
            format!(
                "<details><summary><h3 style=\"display: \
                 inline;\">Details</h3></summary><pre>{details}</pre></details>",
                details = HtmlEscaped(details)
            )
        } else {
            format!(
                "<h3>Details</h3><pre>{details}</pre>",
                details = HtmlEscaped(details)
            )
        };
        let html = format!(
            "<!DOCTYPE html>
<html><head><title>{title}</title></head>
<body>
  <h3>{internal_ty}</h3>
  <h2>{ty}</h2>
  <h1>{title}</h1>
  {details}
  <ul>{children}</ul>
</body>
</html>",
            title = HtmlEscaped(title),
            ty = HtmlEscaped(ty),
            children = FormatIter(|| children.iter())
        );
        Ok(ContentSourceContent::static_content(
            AssetContent::file(
                File::from(html)
                    .with_content_type(mime::TEXT_HTML_UTF_8)
                    .into(),
            )
            .versioned(),
        ))
    }
}
