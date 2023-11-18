use std::backtrace;

use thiserror::Error;
use nxpkgrepo_repository::package_graph;

use crate::{
    commands::{bin, generate, prune},
    daemon::DaemonError,
    rewrite_json::RewriteError,
    run,
};

#[derive(Debug, Error)]
pub enum Error {
    #[error("No command specified")]
    NoCommand(#[backtrace] backtrace::Backtrace),
    #[error("{0}")]
    Bin(#[from] bin::Error, #[backtrace] backtrace::Backtrace),
    #[error(transparent)]
    Path(#[from] nxpkgpath::PathError),
    #[error("at least one task must be specified")]
    NoTasks(#[backtrace] backtrace::Backtrace),
    #[error(transparent)]
    Config(#[from] crate::config::Error),
    #[error(transparent)]
    ChromeTracing(#[from] crate::tracing::Error),
    #[error(transparent)]
    BuildPackageGraph(#[from] package_graph::builder::Error),
    #[error(transparent)]
    Rewrite(#[from] RewriteError),
    #[error(transparent)]
    Auth(#[from] nxpkgrepo_auth::Error),
    #[error(transparent)]
    Daemon(#[from] DaemonError),
    #[error(transparent)]
    Generate(#[from] generate::Error),
    #[error(transparent)]
    Prune(#[from] prune::Error),
    #[error(transparent)]
    PackageJson(#[from] nxpkgrepo_repository::package_json::Error),
    #[error(transparent)]
    PackageManager(#[from] nxpkgrepo_repository::package_manager::Error),
    #[error(transparent)]
    Run(#[from] run::Error),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
}
