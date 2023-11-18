use serde::{Deserialize, Serialize};
use nxpkg_tasks::trace::TraceRawVcs;

#[derive(
    Serialize,
    Deserialize,
    Debug,
    Clone,
    Copy,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    TraceRawVcs,
    Default,
)]
pub enum RuntimeType {
    #[default]
    /// Default, full-featured runtime.
    Default,
    #[cfg(feature = "test")]
    /// Dummy runtime for snapshot tests.
    Dummy,
}
