use anyhow::Result;
use nxpkg_tasks::{Completion, State, Value, Vc};
use nxpkgpack_core::introspect::{Introspectable, IntrospectableChildren};

use super::{
    route_tree::{MapGetContentSourceContent, RouteTree, RouteTrees},
    ContentSource, ContentSourceData, ContentSourceDataVary, ContentSourceSideEffect,
    GetContentSourceContent,
};
use crate::source::{ContentSourceContent, ContentSources};

/// Combines two [ContentSource]s like the [CombinedContentSource], but only
/// allows to serve from the second source when the first source has
/// successfully served something once.
/// This is a laziness optimization when the content of the second source can
/// only be reached via references from the first source.
///
/// For example, we use that in the content source that handles SSR rendering of
/// pages. Here HTML and "other assets" are in different content sources. So we
/// use this source to only serve (and process) "other assets" when the HTML was
/// served once.
#[nxpkg_tasks::value(serialization = "none", eq = "manual", cell = "new")]
pub struct ConditionalContentSource {
    activator: Vc<Box<dyn ContentSource>>,
    action: Vc<Box<dyn ContentSource>>,
    activated: State<bool>,
}

#[nxpkg_tasks::value_impl]
impl ConditionalContentSource {
    #[nxpkg_tasks::function]
    pub fn new(
        activator: Vc<Box<dyn ContentSource>>,
        action: Vc<Box<dyn ContentSource>>,
    ) -> Vc<Self> {
        ConditionalContentSource {
            activator,
            action,
            activated: State::new(false),
        }
        .cell()
    }
}

#[nxpkg_tasks::value_impl]
impl ContentSource for ConditionalContentSource {
    #[nxpkg_tasks::function]
    async fn get_routes(self: Vc<Self>) -> Result<Vc<RouteTree>> {
        let this = self.await?;
        Ok(if !*this.activated.get() {
            this.activator.get_routes().map_routes(Vc::upcast(
                ConditionalContentSourceMapper { source: self }.cell(),
            ))
        } else {
            Vc::<RouteTrees>::cell(vec![this.activator.get_routes(), this.action.get_routes()])
                .merge()
        })
    }

    #[nxpkg_tasks::function]
    fn get_children(&self) -> Vc<ContentSources> {
        Vc::cell(vec![self.activator, self.action])
    }
}

#[nxpkg_tasks::value]
struct ConditionalContentSourceMapper {
    source: Vc<ConditionalContentSource>,
}

#[nxpkg_tasks::value_impl]
impl MapGetContentSourceContent for ConditionalContentSourceMapper {
    #[nxpkg_tasks::function]
    fn map_get_content(
        &self,
        get_content: Vc<Box<dyn GetContentSourceContent>>,
    ) -> Vc<Box<dyn GetContentSourceContent>> {
        Vc::upcast(
            ActivateOnGetContentSource {
                source: self.source,
                get_content,
            }
            .cell(),
        )
    }
}

#[nxpkg_tasks::function]
fn introspectable_type() -> Vc<String> {
    Vc::cell("conditional content source".to_string())
}

#[nxpkg_tasks::function]
fn activator_key() -> Vc<String> {
    Vc::cell("activator".to_string())
}

#[nxpkg_tasks::function]
fn action_key() -> Vc<String> {
    Vc::cell("action".to_string())
}

#[nxpkg_tasks::value_impl]
impl Introspectable for ConditionalContentSource {
    #[nxpkg_tasks::function]
    fn ty(&self) -> Vc<String> {
        introspectable_type()
    }

    #[nxpkg_tasks::function]
    async fn details(&self) -> Result<Vc<String>> {
        Ok(Vc::cell(
            if *self.activated.get() {
                "activated"
            } else {
                "not activated"
            }
            .to_string(),
        ))
    }

    #[nxpkg_tasks::function]
    async fn title(&self) -> Result<Vc<String>> {
        if let Some(activator) =
            Vc::try_resolve_sidecast::<Box<dyn Introspectable>>(self.activator).await?
        {
            Ok(activator.title())
        } else {
            Ok(Vc::<String>::default())
        }
    }

    #[nxpkg_tasks::function]
    async fn children(&self) -> Result<Vc<IntrospectableChildren>> {
        Ok(Vc::cell(
            [
                Vc::try_resolve_sidecast::<Box<dyn Introspectable>>(self.activator)
                    .await?
                    .map(|i| (activator_key(), i)),
                Vc::try_resolve_sidecast::<Box<dyn Introspectable>>(self.action)
                    .await?
                    .map(|i| (action_key(), i)),
            ]
            .into_iter()
            .flatten()
            .collect(),
        ))
    }
}

#[nxpkg_tasks::value(serialization = "none", eq = "manual", cell = "new")]
struct ActivateOnGetContentSource {
    source: Vc<ConditionalContentSource>,
    get_content: Vc<Box<dyn GetContentSourceContent>>,
}

#[nxpkg_tasks::value_impl]
impl GetContentSourceContent for ActivateOnGetContentSource {
    #[nxpkg_tasks::function]
    fn vary(&self) -> Vc<ContentSourceDataVary> {
        self.get_content.vary()
    }

    #[nxpkg_tasks::function]
    async fn get(
        self: Vc<Self>,
        path: String,
        data: Value<ContentSourceData>,
    ) -> Result<Vc<ContentSourceContent>> {
        nxpkg_tasks::emit(Vc::upcast::<Box<dyn ContentSourceSideEffect>>(self));
        Ok(self.await?.get_content.get(path, data))
    }
}

#[nxpkg_tasks::value_impl]
impl ContentSourceSideEffect for ActivateOnGetContentSource {
    #[nxpkg_tasks::function]
    async fn apply(&self) -> Result<Vc<Completion>> {
        self.source.await?.activated.set(true);
        Ok(Completion::new())
    }
}
