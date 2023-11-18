use nxpkg_tasks::Vc;
use nxpkgpack_core::chunk::ChunkingContext;

/// [`EcmascriptChunkingContext`] must be implemented by [`ChunkingContext`]
/// implementors that want to operate on [`EcmascriptChunk`]s.
#[nxpkg_tasks::value_trait]
pub trait EcmascriptChunkingContext: ChunkingContext {
    /// Whether chunk items generated by this chunking context should include
    /// the `__nxpkgpack_refresh__` argument.
    fn has_react_refresh(self: Vc<Self>) -> Vc<bool> {
        Vc::cell(false)
    }
}
