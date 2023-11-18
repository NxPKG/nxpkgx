mod change_detector;
mod filter;
mod simple_glob;
mod target_selector;

use std::collections::HashSet;

use filter::{FilterResolver, PackageInference};
use nxpkgpath::AbsoluteSystemPath;
use nxpkgrepo_repository::package_graph::{PackageGraph, WorkspaceName};
use nxpkgrepo_scm::SCM;

use crate::opts::ScopeOpts;
pub use crate::run::scope::filter::ResolutionError;

#[tracing::instrument(skip(opts, pkg_graph, scm))]
pub fn resolve_packages(
    opts: &ScopeOpts,
    nxpkg_root: &AbsoluteSystemPath,
    pkg_graph: &PackageGraph,
    scm: &SCM,
) -> Result<HashSet<WorkspaceName>, ResolutionError> {
    let pkg_inference = opts.pkg_inference_root.as_ref().map(|pkg_inference_path| {
        PackageInference::calculate(nxpkg_root, pkg_inference_path, pkg_graph)
    });

    let filtered_packages = FilterResolver::new(opts, pkg_graph, nxpkg_root, pkg_inference, scm)
        .resolve(&opts.get_filters())?;

    Ok(filtered_packages)
}
