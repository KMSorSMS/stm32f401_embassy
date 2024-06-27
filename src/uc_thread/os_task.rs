//! OS task/thread abstraction
use defmt::info;

use super::os_core::{os_sched, os_tcb_init};
use super::os_cpu::ostask_stk_init;
use super::types::{OSTCBPrio, OsErrState, OsStkPtr, Task};
use super::{OSINT_NESTING, OS_IS_RUNNING, OS_TCB_PRIO_TBL};

/// create a task/thread
#[allow(unused)]
pub fn os_task_create(task: Task, ptos: OsStkPtr, prio: OSTCBPrio) -> OsErrState {
    // info!("I'm in task_1");
    let _is_running = unsafe { OS_IS_RUNNING };
    // info!("if os is running in os_task_create {}", unsafe { OS_IS_RUNNING });
    // judge if the tcb already allocated
    if critical_section::with(|cs| {
        let _is_running = unsafe { OS_IS_RUNNING };
        let mut ref_prio_tcb = unsafe { OS_TCB_PRIO_TBL[prio as usize] };
        // also make sure we are not in the interrupt
        if !ref_prio_tcb.is_null() || unsafe { OSINT_NESTING } > 0 {
            return false;
        }
        true
    }) {
        // we call the OSTaskStkInit function to initialize the task's stack
        let psp = ostask_stk_init(task, ptos);
        // the allocate adress of psp
        info!("psp in os_task_create is {:#?}", psp);
        // with this psp we will init our tcb
        let err = os_tcb_init(prio, psp);
        if err == OsErrState::OsErrNone {
            if unsafe { OS_IS_RUNNING } {
                os_sched();
                return OsErrState::OsErrNone;
            }
        } else {
            return err;
        }
    }
    OsErrState::OsErrTaskCreate
}