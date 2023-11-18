use thiserror::Error;
use nxpkgpath::AbsoluteSystemPathBuf;
use nxpkgrepo_repository::package_graph;

use crate::{
    config, daemon, engine, opts,
    run::{global_hash, scope},
    task_graph, task_hash,
};

#[derive(Debug, Error)]
pub enum Error {
    #[error("failed to open graph file {0}")]
    OpenGraphFile(#[source] std::io::Error, AbsoluteSystemPathBuf),
    #[error("failed to produce graph output")]
    GraphOutput(#[source] std::io::Error),
    #[error("error preparing engine: Invalid persistent task configuration:\n{0}")]
    EngineValidation(String),
    #[error(transparent)]
    Builder(#[from] engine::BuilderError),
    #[error(transparent)]
    Env(#[from] nxpkgrepo_env::Error),
    #[error(transparent)]
    Opts(#[from] opts::Error),
    #[error(transparent)]
    PackageJson(#[from] nxpkgrepo_repository::package_json::Error),
    #[error(transparent)]
    PackageManager(#[from] nxpkgrepo_repository::package_manager::Error),
    #[error(transparent)]
    Config(#[from] config::Error),
    #[error(transparent)]
    PackageGraphBuilder(#[from] package_graph::builder::Error),
    #[error(transparent)]
    DaemonConnector(#[from] daemon::DaemonConnectorError),
    #[error(transparent)]
    Cache(#[from] nxpkgrepo_cache::CacheError),
    #[error(transparent)]
    Path(#[from] nxpkgpath::PathError),
    #[error(transparent)]
    Scope(#[from] scope::ResolutionError),
    #[error(transparent)]
    GlobalHash(#[from] global_hash::Error),
    #[error(transparent)]
    TaskHash(#[from] task_hash::Error),
    #[error(transparent)]
    Visitor(#[from] task_graph::VisitorError),
}
