use std::io::Write;

use anyhow::Result;
use indoc::writedoc;
use nxpkg_tasks::{ReadRef, TryJoinIterExt, Vc};
use nxpkg_tasks_fs::File;
use nxpkgpack_core::{
    asset::AssetContent,
    chunk::{ChunkItemExt, ChunkingContext, ModuleId},
    code_builder::{Code, CodeBuilder},
    output::OutputAsset,
    source_map::{GenerateSourceMap, OptionSourceMap},
    version::{Version, VersionedContent},
};
use nxpkgpack_ecmascript::{
    chunk::{EcmascriptChunkContent, EcmascriptChunkItemExt},
    utils::StringifyJs,
};

use super::{chunk::EcmascriptBuildNodeChunk, version::EcmascriptBuildNodeChunkVersion};
use crate::{chunking_context::MinifyType, ecmascript::minify::minify, BuildChunkingContext};

#[nxpkg_tasks::value]
pub(super) struct EcmascriptBuildNodeChunkContent {
    pub(super) content: Vc<EcmascriptChunkContent>,
    pub(super) chunking_context: Vc<BuildChunkingContext>,
    pub(super) chunk: Vc<EcmascriptBuildNodeChunk>,
}

#[nxpkg_tasks::value_impl]
impl EcmascriptBuildNodeChunkContent {
    #[nxpkg_tasks::function]
    pub(crate) async fn new(
        chunking_context: Vc<BuildChunkingContext>,
        chunk: Vc<EcmascriptBuildNodeChunk>,
        content: Vc<EcmascriptChunkContent>,
    ) -> Result<Vc<Self>> {
        Ok(EcmascriptBuildNodeChunkContent {
            content,
            chunking_context,
            chunk,
        }
        .cell())
    }
}

pub(super) async fn chunk_items(
    content: Vc<EcmascriptChunkContent>,
) -> Result<Vec<(ReadRef<ModuleId>, ReadRef<Code>)>> {
    content
        .await?
        .chunk_items
        .iter()
        .map(|&(chunk_item, async_module_info)| async move {
            Ok((
                chunk_item.id().await?,
                chunk_item.code(async_module_info).await?,
            ))
        })
        .try_join()
        .await
}

#[nxpkg_tasks::value_impl]
impl EcmascriptBuildNodeChunkContent {
    #[nxpkg_tasks::function]
    async fn code(self: Vc<Self>) -> Result<Vc<Code>> {
        let this = self.await?;
        let chunk_path_vc = this.chunk.ident().path();
        let chunk_path = chunk_path_vc.await?;

        let mut code = CodeBuilder::default();

        writedoc!(
            code,
            r#"
                module.exports = {{

            "#,
        )?;

        for (id, item_code) in chunk_items(this.content).await? {
            write!(code, "{}: ", StringifyJs(&id))?;
            code.push_code(&item_code);
            writeln!(code, ",")?;
        }

        write!(code, "\n}};")?;

        if code.has_source_map() {
            let filename = chunk_path.file_name();
            write!(code, "\n\n//# sourceMappingURL={}.map", filename)?;
        }

        let code = code.build().cell();
        if matches!(
            this.chunking_context.await?.minify_type(),
            MinifyType::Minify
        ) {
            return Ok(minify(chunk_path_vc, code));
        }

        Ok(code)
    }

    #[nxpkg_tasks::function]
    pub(crate) async fn own_version(self: Vc<Self>) -> Result<Vc<EcmascriptBuildNodeChunkVersion>> {
        let this = self.await?;
        Ok(EcmascriptBuildNodeChunkVersion::new(
            this.chunking_context.output_root(),
            this.chunk.ident().path(),
            this.content,
            this.chunking_context.await?.minify_type(),
        ))
    }
}

#[nxpkg_tasks::value_impl]
impl GenerateSourceMap for EcmascriptBuildNodeChunkContent {
    #[nxpkg_tasks::function]
    fn generate_source_map(self: Vc<Self>) -> Vc<OptionSourceMap> {
        self.code().generate_source_map()
    }
}

#[nxpkg_tasks::value_impl]
impl VersionedContent for EcmascriptBuildNodeChunkContent {
    #[nxpkg_tasks::function]
    async fn content(self: Vc<Self>) -> Result<Vc<AssetContent>> {
        let code = self.code().await?;
        Ok(AssetContent::file(
            File::from(code.source_code().clone()).into(),
        ))
    }

    #[nxpkg_tasks::function]
    fn version(self: Vc<Self>) -> Vc<Box<dyn Version>> {
        Vc::upcast(self.own_version())
    }
}
