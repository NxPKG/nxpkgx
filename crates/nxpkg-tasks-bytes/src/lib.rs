#![feature(arbitrary_self_types)]

pub mod bytes;
pub mod stream;

pub use crate::{
    bytes::Bytes,
    stream::{Stream, StreamRead},
};

pub fn register() {
    nxpkg_tasks::register();
    include!(concat!(env!("OUT_DIR"), "/register.rs"));
}
