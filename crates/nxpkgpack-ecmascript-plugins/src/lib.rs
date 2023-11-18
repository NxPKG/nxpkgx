#![feature(arbitrary_self_types)]

pub mod transform;

pub fn register() {
    nxpkg_tasks::register();
    nxpkgpack_ecmascript::register();
    include!(concat!(env!("OUT_DIR"), "/register.rs"));
}
