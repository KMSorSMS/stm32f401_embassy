//! OS task/thread abstraction
use super::{OSINT_NESTING, OS_TCB_PRIO_TBL};
use super::os_cpu::ostask_stk_init;
use super::types::{OSTCBPrio, OsErrState, OsStkPtr, Task};

/// create a task/thread
#[allow(unused)]
pub fn os_task_create(task: Task, ptos: OsStkPtr, prio: OSTCBPrio) -> OsErrState {
    // judge if the tcb already allocated
    if critical_section::with(|cs| {
        let mut ref_prio_tcb = &unsafe { OS_TCB_PRIO_TBL[prio as usize] };
        if let Some(_) = ref_prio_tcb {
            return false;
        }
        // make sure we are not in the interrupt
        else if unsafe { OSINT_NESTING } > 0 {
            return false;
        }
        true
    }) {
        // we call the OSTaskStkInit function to initialize the task's stack
        let psp = ostask_stk_init(task, ptos);
        // with this psp we will init our tcb

    }
    OsErrState::OsErrNone
}
