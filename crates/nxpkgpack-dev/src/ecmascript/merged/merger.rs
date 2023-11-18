use anyhow::{bail, Result};
use nxpkg_tasks::{TryJoinIterExt, Vc};
use nxpkgpack_core::version::{VersionedContent, VersionedContentMerger, VersionedContents};

use super::{super::content::EcmascriptDevChunkContent, content::EcmascriptDevMergedChunkContent};

/// Merges multiple [`EcmascriptChunkContent`] into a single
/// [`EcmascriptDevMergedChunkContent`]. This is useful for generating a single
/// update for multiple ES chunks updating all at the same time.
#[nxpkg_tasks::value]
pub(crate) struct EcmascriptDevChunkContentMerger;

#[nxpkg_tasks::value_impl]
impl EcmascriptDevChunkContentMerger {
    /// Creates a new [`EcmascriptDevChunkContentMerger`].
    #[nxpkg_tasks::function]
    pub fn new() -> Vc<Self> {
        Self::cell(EcmascriptDevChunkContentMerger)
    }
}

#[nxpkg_tasks::value_impl]
impl VersionedContentMerger for EcmascriptDevChunkContentMerger {
    #[nxpkg_tasks::function]
    async fn merge(
        &self,
        contents: Vc<VersionedContents>,
    ) -> Result<Vc<Box<dyn VersionedContent>>> {
        let contents = contents
            .await?
            .iter()
            .map(|content| async move {
                if let Some(content) =
                    Vc::try_resolve_downcast_type::<EcmascriptDevChunkContent>(*content).await?
                {
                    Ok(content)
                } else {
                    bail!("expected Vc<EcmascriptDevChunkContent>")
                }
            })
            .try_join()
            .await?;

        Ok(Vc::upcast(
            EcmascriptDevMergedChunkContent { contents }.cell(),
        ))
    }
}
