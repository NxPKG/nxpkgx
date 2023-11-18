use std::marker::PhantomData;

use auto_hash_map::AutoSet;
// This specific macro identifier is detected by nxpkg-tasks-build.
use nxpkg_tasks_macros::primitive as __nxpkg_tasks_internal_primitive;

use crate as nxpkg_tasks;
use crate::{RawVc, TaskId, Vc};

__nxpkg_tasks_internal_primitive!(AutoSet<RawVc>);

impl Vc<AutoSet<RawVc>> {
    /// Casts a `TaskId` to a `Vc<AutoSet<RawVc>>`.
    ///
    /// # Safety
    ///
    /// The `TaskId` must be point to a valid `AutoSet<RawVc>`.
    pub unsafe fn from_task_id(task_id: TaskId) -> Self {
        Vc {
            node: RawVc::TaskOutput(task_id),
            _t: PhantomData,
        }
    }
}
