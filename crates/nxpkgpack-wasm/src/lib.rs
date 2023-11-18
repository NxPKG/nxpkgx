//! WebAssembly support for nxpkgpack.
//!
//! WASM assets are copied directly to the output folder.
//!
//! When imported from ES modules, they produce a thin module that loads and
//! instantiates the WebAssembly module.

#![feature(min_specialization)]
#![feature(arbitrary_self_types)]

pub(crate) mod analysis;
pub(crate) mod loader;
pub mod module_asset;
pub(crate) mod output_asset;
pub mod raw;
pub mod source;

pub fn register() {
    nxpkg_tasks::register();
    nxpkg_tasks_fs::register();
    nxpkgpack_core::register();
    nxpkgpack_ecmascript::register();
    include!(concat!(env!("OUT_DIR"), "/register.rs"));
}
