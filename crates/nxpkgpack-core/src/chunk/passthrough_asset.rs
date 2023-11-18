use crate::{asset::Asset, module::Module};

/// A [Module] that should never be placed into a chunk, but whose references
/// should still be followed.
#[nxpkg_tasks::value_trait]
pub trait PassthroughModule: Module + Asset {}
