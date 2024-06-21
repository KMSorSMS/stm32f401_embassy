//! the basic structure for thread,like TCB and stack
mod types;
use types::{OSTCBPrio, OSTCBPtr, OsStkPtr, OSPRIOBIT};

#[repr(C)]
struct OsTCB {
    /// stack pointer contains the basic register info
    sp: OsStkPtr,
    /// to make it a double linked list,need OSTCBNext and OSTCBPre
    ostcb_next: Option<OSTCBPtr>,
    ostcb_pre: Option<OSTCBPtr>,
    /// the priority of the thread
    os_prio: OSTCBPrio,
    /// to accomplish the real-time requirement, adding bit map
    os_tcb_bitx: OSPRIOBIT,
    os_tcb_bity: OSPRIOBIT,
}