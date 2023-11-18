use anyhow::Result;
use nxpkg_tasks::Vc;
use nxpkg_tasks_env::ProcessEnv;
use nxpkg_tasks_fs::FileSystemPath;
use nxpkgpack_core::chunk::ChunkingContext;

#[nxpkg_tasks::value]
pub struct ExecutionContext {
    pub project_path: Vc<FileSystemPath>,
    pub chunking_context: Vc<Box<dyn ChunkingContext>>,
    pub env: Vc<Box<dyn ProcessEnv>>,
}

#[nxpkg_tasks::value_impl]
impl ExecutionContext {
    #[nxpkg_tasks::function]
    pub fn new(
        project_path: Vc<FileSystemPath>,
        chunking_context: Vc<Box<dyn ChunkingContext>>,
        env: Vc<Box<dyn ProcessEnv>>,
    ) -> Vc<Self> {
        ExecutionContext {
            project_path,
            chunking_context,
            env,
        }
        .cell()
    }

    #[nxpkg_tasks::function]
    pub async fn project_path(self: Vc<Self>) -> Result<Vc<FileSystemPath>> {
        Ok(self.await?.project_path)
    }

    #[nxpkg_tasks::function]
    pub async fn chunking_context(self: Vc<Self>) -> Result<Vc<Box<dyn ChunkingContext>>> {
        Ok(self.await?.chunking_context)
    }

    #[nxpkg_tasks::function]
    pub async fn env(self: Vc<Self>) -> Result<Vc<Box<dyn ProcessEnv>>> {
        Ok(self.await?.env)
    }
}
