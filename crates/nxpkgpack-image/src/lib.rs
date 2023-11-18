#![feature(arbitrary_self_types)]

pub mod process;

pub fn register() {
    nxpkg_tasks::register();
    nxpkg_tasks_fs::register();
    nxpkgpack_core::register();
    include!(concat!(env!("OUT_DIR"), "/register.rs"));
}
