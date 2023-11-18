use anyhow::{bail, Result};
use nxpkg_tasks::{TryJoinIterExt, Vc};
use nxpkgpack_core::{
    asset::AssetContent,
    version::{Update, Version, VersionedContent},
};

use super::{
    super::content::EcmascriptDevChunkContent, update::update_ecmascript_merged_chunk,
    version::EcmascriptDevMergedChunkVersion,
};

/// Composite [`EcmascriptChunkContent`] that is the result of merging multiple
/// EcmaScript chunk's contents together through the
/// [`EcmascriptChunkContentMerger`].
///
/// [`EcmascriptChunkContentMerger`]: super::merger::EcmascriptChunkContentMerger
#[nxpkg_tasks::value(serialization = "none", shared)]
pub(super) struct EcmascriptDevMergedChunkContent {
    pub contents: Vec<Vc<EcmascriptDevChunkContent>>,
}

#[nxpkg_tasks::value_impl]
impl EcmascriptDevMergedChunkContent {
    #[nxpkg_tasks::function]
    pub async fn version(self: Vc<Self>) -> Result<Vc<EcmascriptDevMergedChunkVersion>> {
        Ok(EcmascriptDevMergedChunkVersion {
            versions: self
                .await?
                .contents
                .iter()
                .map(|content| async move { content.own_version().await })
                .try_join()
                .await?,
        }
        .cell())
    }
}

#[nxpkg_tasks::value_impl]
impl VersionedContent for EcmascriptDevMergedChunkContent {
    #[nxpkg_tasks::function]
    fn content(self: Vc<Self>) -> Result<Vc<AssetContent>> {
        bail!("EcmascriptDevMergedChunkContent does not have content")
    }

    #[nxpkg_tasks::function]
    fn version(self: Vc<Self>) -> Vc<Box<dyn Version>> {
        Vc::upcast(self.version())
    }

    #[nxpkg_tasks::function]
    async fn update(self: Vc<Self>, from_version: Vc<Box<dyn Version>>) -> Result<Vc<Update>> {
        Ok(update_ecmascript_merged_chunk(self, from_version)
            .await?
            .cell())
    }
}
