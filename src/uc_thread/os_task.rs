//! OS task/thread abstraction

use super::types::{OsErrState, OsStkPtr};

/// create a task/thread
#[allow(unused)]
pub fn os_task_create(task: impl FnOnce(),ptos:OsStkPtr) -> OsErrState{
    critical_section::with(|cs| {

    });
    OsErrState::OsErrNone
}