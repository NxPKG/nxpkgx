use indexmap::IndexMap;
use nxpkg_tasks::Vc;

#[nxpkg_tasks::value(transparent)]
#[derive(Debug, Clone)]
#[serde(untagged)]
pub enum Param {
    Single(String),
    Multi(Vec<String>),
}

#[nxpkg_tasks::value(transparent)]
#[derive(Debug, Clone)]
pub struct Params(pub Option<IndexMap<String, Param>>);

/// Extracts parameters from a URL path.
pub trait RouteMatcherRef {
    /// Returns whether the given path is a match for the route.
    fn matches(&self, path: &str) -> bool;

    /// Returns the parameters extracted from the given path.
    fn params(&self, path: &str) -> Params;
}

/// Extracts parameters from a URL path (Vc version)
#[nxpkg_tasks::value_trait]
pub trait RouteMatcher {
    /// Returns whether the given path is a match for the route.
    fn matches(self: Vc<Self>, path: String) -> Vc<bool>;

    /// Returns the parameters extracted from the given path.
    fn params(self: Vc<Self>, path: String) -> Vc<Params>;
}
