use anyhow::Result;
use nxpkg_tasks::Vc;

use crate::source::Source;

#[nxpkg_tasks::value_trait]
pub trait SourceTransform {
    fn transform(self: Vc<Self>, source: Vc<Box<dyn Source>>) -> Vc<Box<dyn Source>>;
}

#[nxpkg_tasks::value(transparent)]
pub struct SourceTransforms(Vec<Vc<Box<dyn SourceTransform>>>);

#[nxpkg_tasks::value_impl]
impl SourceTransforms {
    #[nxpkg_tasks::function]
    pub async fn transform(
        self: Vc<Self>,
        source: Vc<Box<dyn Source>>,
    ) -> Result<Vc<Box<dyn Source>>> {
        Ok(self
            .await?
            .iter()
            .fold(source, |source, transform| transform.transform(source)))
    }
}
