//! OS task/thread abstraction

use super::types::OsErrState;

/// create a task/thread
#[allow(unused)]
pub fn os_task_create() -> OsErrState{
    critical_section::with(|cs| {

    });
    OsErrState::OsErrNone
}