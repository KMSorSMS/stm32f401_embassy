//! the basic structure for thread,like TCB and stack
mod types;
mod os_task;
use types::{OSTCBPrio, OSTCBPtr, OsStkPtr, OSPRIOBIT};

#[repr(C)]
#[allow(unused)]
struct OsTCB<'a> {
    /// stack pointer contains the basic register info
    sp: OsStkPtr<'a>,
    /// to make it a double linked list,need OSTCBNext and OSTCBPre
    ostcb_next: Option<OSTCBPtr<'a>>,
    ostcb_pre: Option<OSTCBPtr<'a>>,
    /// the priority of the thread
    os_prio: OSTCBPrio,
    /// to accomplish the real-time requirement, adding bit map
    os_tcb_bitx: OSPRIOBIT,
    os_tcb_bity: OSPRIOBIT,
}