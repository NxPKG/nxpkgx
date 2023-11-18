use nxpkg_tasks::Vc;

use crate::{self as nxpkg_tasks};

/// `Vc<T>` analog to the `Default` trait.
///
/// Implementing this trait on `T` will make `Vc::default()` produce
/// `T::value_default()`.
///
/// There are two ways to implement this trait:
/// 1. Annotating with `#[nxpkg_tasks::value_impl]`: this will make
///    `Vc::default()` always return the same underlying value (i.e. a
///    singleton).
/// 2. No annotations: this will make `Vc::default()` always return a different
///    value.
#[nxpkg_tasks::value_trait]
pub trait ValueDefault {
    fn value_default() -> Vc<Self>;
}
