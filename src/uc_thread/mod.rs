//! the basic structure for thread,like TCB and stack
mod os_task;
mod os_core;
mod types;
use types::{OSTCBPrio, OSTCBPtr, OsStkPtr, OSPRIOBIT};
use lazy_static::*;




#[repr(C)]
#[allow(unused)]
#[derive(Clone, Copy)]
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

/// create a static OSTCB linked list we wrap it in a upsafecell to avoid unsafe access of global variable
#[allow(unused)]
lazy_static! {
    static ref mut OSTCBTBL: [OsTCB; 64] = [OsTCB {
        sp: &0,
        ostcb_next: None,
        ostcb_pre: None,
        os_prio: 0,
        os_tcb_bitx: 0,
        os_tcb_bity: 0,
    }; 64];
}
/// create a Table of pointers of Created TCBs 
#[allow(unused)]
static mut OSTCBPRIOTBL: [Option<OSTCBPtr>; 64] = [None; 64];


/// OSRdyGrp, used by the bit map
static mut OS_RDY_GRP:OSTCBPrio = 0;
