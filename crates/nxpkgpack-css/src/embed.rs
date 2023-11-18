use nxpkg_tasks::Vc;
use nxpkgpack_core::{chunk::ChunkItem, output::OutputAsset};

#[nxpkg_tasks::value_trait]
pub trait CssEmbed: ChunkItem {
    fn embedded_asset(self: Vc<Self>) -> Vc<Box<dyn OutputAsset>>;
}
