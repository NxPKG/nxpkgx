use nxpkg_tasks::{Value, Vc};
use nxpkg_tasks_fs::FileSystemPath;
use nxpkgpack_core::chunk::{ChunkingContext, EvaluatableAsset, EvaluatableAssets};
use nxpkgpack_dev_server::source::ContentSourceData;

#[nxpkg_tasks::value(shared)]
pub struct NodeRenderingEntry {
    pub runtime_entries: Vc<EvaluatableAssets>,
    pub module: Vc<Box<dyn EvaluatableAsset>>,
    pub chunking_context: Vc<Box<dyn ChunkingContext>>,
    pub intermediate_output_path: Vc<FileSystemPath>,
    pub output_root: Vc<FileSystemPath>,
    pub project_dir: Vc<FileSystemPath>,
}

#[nxpkg_tasks::value(transparent)]
pub struct NodeRenderingEntries(Vec<Vc<NodeRenderingEntry>>);

/// Trait that allows to get the entry module for rendering something in Node.js
#[nxpkg_tasks::value_trait]
pub trait NodeEntry {
    fn entry(self: Vc<Self>, data: Value<ContentSourceData>) -> Vc<NodeRenderingEntry>;
    fn entries(self: Vc<Self>) -> Vc<NodeRenderingEntries> {
        Vc::cell(vec![self.entry(Value::new(Default::default()))])
    }
}
