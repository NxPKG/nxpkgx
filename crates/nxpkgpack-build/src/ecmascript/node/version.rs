use anyhow::{bail, Result};
use nxpkg_tasks::{ReadRef, Vc};
use nxpkg_tasks_fs::FileSystemPath;
use nxpkg_tasks_hash::{encode_hex, Xxh3Hash64Hasher};
use nxpkgpack_core::{chunk::ModuleId, code_builder::Code, version::Version};
use nxpkgpack_ecmascript::chunk::EcmascriptChunkContent;

use super::content::chunk_items;
use crate::MinifyType;

#[nxpkg_tasks::value(serialization = "none")]
pub(super) struct EcmascriptBuildNodeChunkVersion {
    chunk_path: String,
    chunk_items: Vec<(ReadRef<ModuleId>, ReadRef<Code>)>,
    minify_type: MinifyType,
}

#[nxpkg_tasks::value_impl]
impl EcmascriptBuildNodeChunkVersion {
    #[nxpkg_tasks::function]
    pub async fn new(
        output_root: Vc<FileSystemPath>,
        chunk_path: Vc<FileSystemPath>,
        content: Vc<EcmascriptChunkContent>,
        minify_type: MinifyType,
    ) -> Result<Vc<Self>> {
        let output_root = output_root.await?;
        let chunk_path = chunk_path.await?;
        let chunk_path = if let Some(path) = output_root.get_path_to(&chunk_path) {
            path
        } else {
            bail!(
                "chunk path {} is not in client root {}",
                chunk_path.to_string(),
                output_root.to_string()
            );
        };
        let chunk_items = chunk_items(content).await?;
        Ok(EcmascriptBuildNodeChunkVersion {
            chunk_path: chunk_path.to_string(),
            chunk_items,
            minify_type,
        }
        .cell())
    }
}

#[nxpkg_tasks::value_impl]
impl Version for EcmascriptBuildNodeChunkVersion {
    #[nxpkg_tasks::function]
    fn id(&self) -> Vc<String> {
        let mut hasher = Xxh3Hash64Hasher::new();
        hasher.write_ref(&self.chunk_path);
        hasher.write_ref(&self.minify_type);
        hasher.write_value(self.chunk_items.len());
        for (module_id, code) in &self.chunk_items {
            hasher.write_value((module_id, code.source_code()));
        }
        let hash = hasher.finish();
        let hex_hash = encode_hex(hash);
        Vc::cell(hex_hash)
    }
}
