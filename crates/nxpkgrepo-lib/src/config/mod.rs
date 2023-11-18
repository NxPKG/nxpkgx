mod nxpkg;
mod nxpkg_config;

use std::io;

use thiserror::Error;
pub use nxpkg::{
    validate_extends, validate_no_package_task_syntax, RawNxpkgJSON, SpacesJson, NxpkgJson,
};
pub use nxpkg_config::{ConfigurationOptions, NxpkgrepoConfigBuilder};
use nxpkgpath::AbsoluteSystemPathBuf;

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Error)]
pub enum Error {
    #[error("Global config path not found")]
    NoGlobalConfigPath,
    #[error(transparent)]
    PackageJson(#[from] nxpkgrepo_repository::package_json::Error),
    #[error(
        "Could not find nxpkg.json. Follow directions at https://nxpkg.build/repo/docs to create \
         one"
    )]
    NoNxpkgJSON,
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Camino(#[from] camino::FromPathBufError),
    #[error("Encountered an IO error while attempting to read {config_path}: {error}")]
    FailedToReadConfig {
        config_path: AbsoluteSystemPathBuf,
        error: io::Error,
    },
    #[error("Encountered an IO error while attempting to set {config_path}: {error}")]
    FailedToSetConfig {
        config_path: AbsoluteSystemPathBuf,
        error: io::Error,
    },
    #[error(
        "Package tasks (<package>#<task>) are not allowed in single-package repositories: found \
         {task_id}"
    )]
    PackageTaskInSinglePackageMode { task_id: String },
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error(
        "You specified \"{value}\" in the \"{key}\" key. You should not prefix your environment \
         variables with \"{env_pipeline_delimiter}\""
    )]
    InvalidEnvPrefix {
        value: String,
        key: String,
        env_pipeline_delimiter: &'static str,
    },
    #[error(transparent)]
    PathError(#[from] nxpkgpath::PathError),
    #[error("\"{actual}\". Use \"{wanted}\" instead")]
    UnnecessaryPackageTaskSyntax { actual: String, wanted: String },
    #[error("You can only extend from the root workspace")]
    ExtendFromNonRoot,
    #[error("No \"extends\" key found")]
    NoExtends,
    #[error("Failed to create APIClient: {0}")]
    ApiClient(#[source] nxpkgrepo_api_client::Error),
    #[error("{0} is not UTF8.")]
    Encoding(String),
    #[error("NXPKG_SIGNATURE should be either 1 or 0.")]
    InvalidSignature,
    #[error("NXPKG_REMOTE_CACHE_ENABLED should be either 1 or 0.")]
    InvalidRemoteCacheEnabled,
    #[error("NXPKG_REMOTE_CACHE_TIMEOUT: error parsing timeout.")]
    InvalidRemoteCacheTimeout(#[source] std::num::ParseIntError),
    #[error("NXPKG_PREFLIGHT should be either 1 or 0.")]
    InvalidPreflight,
}
