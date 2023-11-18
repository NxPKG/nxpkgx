#![feature(lint_reasons)]
#![feature(iter_intersperse)]
#![feature(int_roundings)]
#![feature(arbitrary_self_types)]

pub(crate) mod chunking_context;
pub(crate) mod ecmascript;
pub mod react_refresh;

pub use chunking_context::{DevChunkingContext, DevChunkingContextBuilder};

pub fn register() {
    nxpkg_tasks::register();
    nxpkg_tasks_fs::register();
    nxpkgpack_core::register();
    nxpkgpack_ecmascript::register();
    nxpkgpack_ecmascript_runtime::register();
    include!(concat!(env!("OUT_DIR"), "/register.rs"));
}
