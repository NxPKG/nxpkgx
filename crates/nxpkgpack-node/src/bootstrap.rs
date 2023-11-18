use std::fmt::Write;

use anyhow::Result;
use nxpkg_tasks::Vc;
use nxpkg_tasks_fs::{File, FileSystemPath};
use nxpkgpack_core::{
    asset::{Asset, AssetContent},
    chunk::{ChunkingContext, EvaluatableAssets},
    ident::AssetIdent,
    output::{OutputAsset, OutputAssets},
};
use nxpkgpack_ecmascript::utils::StringifyJs;

#[nxpkg_tasks::value(shared)]
pub(super) struct NodeJsBootstrapAsset {
    pub(super) path: Vc<FileSystemPath>,
    pub(super) chunking_context: Vc<Box<dyn ChunkingContext>>,
    pub(super) evaluatable_assets: Vc<EvaluatableAssets>,
}

#[nxpkg_tasks::function]
fn node_js_bootstrap_chunk_reference_description() -> Vc<String> {
    Vc::cell("node.js bootstrap chunk".to_string())
}

impl NodeJsBootstrapAsset {
    fn chunks(&self) -> Vc<OutputAssets> {
        self.chunking_context
            .evaluated_chunk_group(AssetIdent::from_path(self.path), self.evaluatable_assets)
    }
}

#[nxpkg_tasks::value_impl]
impl OutputAsset for NodeJsBootstrapAsset {
    #[nxpkg_tasks::function]
    fn ident(&self) -> Vc<AssetIdent> {
        AssetIdent::from_path(self.path)
    }

    #[nxpkg_tasks::function]
    fn references(&self) -> Vc<OutputAssets> {
        self.chunks()
    }
}

#[nxpkg_tasks::value_impl]
impl Asset for NodeJsBootstrapAsset {
    #[nxpkg_tasks::function]
    async fn content(&self) -> Result<Vc<AssetContent>> {
        let context_path = self.path.parent().await?;

        // TODO(sokra) We need to have a chunk format for node.js
        // but until then this is a simple hack to make it work for now
        let mut output = "Error.stackTraceLimit = 100;\nglobal.self = global;\n".to_string();

        for chunk in self.chunks().await?.iter() {
            let path = &*chunk.ident().path().await?;
            if let Some(p) = context_path.get_relative_path_to(path) {
                if p.ends_with(".js") {
                    writeln!(&mut output, "require({});", StringifyJs(&p))?;
                }
            }
        }

        Ok(AssetContent::file(File::from(output).into()))
    }
}
