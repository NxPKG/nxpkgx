use nxpkg_tasks::Vc;

use crate::{self as nxpkg_tasks};

#[nxpkg_tasks::value_trait]
pub trait ValueToString {
    fn to_string(self: Vc<Self>) -> Vc<String>;
}
