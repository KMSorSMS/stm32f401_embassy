//! OS task/thread abstraction
use super::OSTCBPRIOTBL;
use super::types::{OsErrState, OsStkPtr};

/// create a task/thread
#[allow(unused)]
pub fn os_task_create(task: impl FnOnce(),ptos:OsStkPtr,prio:OSTCBPrio) -> OsErrState{
    critical_section::with(|cs| {
        if let None = OSTCBPRIOTBL[prio] {
            
        }
    });
    OsErrState::OsErrNone
}

