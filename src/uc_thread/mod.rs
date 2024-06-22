//! the basic structure for thread,like TCB and stack
mod os_core;
mod os_task;
mod os_cpu;
mod types;
use types::{OSTCBPrio, OSTCBPtr, OsStkPtr, OSPRIOBIT};

#[repr(C)]
#[allow(unused)]
#[derive(Clone, Copy)]
struct OsTCB<'a> {
    /// stack pointer contains the basic register info
    sp: OsStkPtr,
    /// to make it a double linked list,need OSTCBNext and OSTCBPre
    ostcb_next: Option<OSTCBPtr<'a>>,
    ostcb_pre: Option<OSTCBPtr<'a>>,
    /// the priority of the thread
    os_prio: OSTCBPrio,
    /// to accomplish the real-time requirement, adding bit map
    os_tcb_bitx: OSPRIOBIT,
    os_tcb_bity: OSPRIOBIT,
}

/********************************************************************************
                                   Globle Settings
*********************************************************************************/
/// the lowest priority
const OS_LOWEST_PRIO: usize= 63;
/// the prio of idle
const OS_TASK_IDLE_PRIO:usize=OS_LOWEST_PRIO;
/// the size of the context stack(unit is 32bit)
const CONTEXT_STACK_SIZE: usize = 16;
/// the number of user's task
const OS_MAX_TASKS:usize = 20;
/// the number of system's task(now just idle)
const OS_N_SYS_TASKS:usize = 1;


/********************************************************************************
                                    TCBs' List
*********************************************************************************/
/// create a static OSTCB linked list we wrap it in a upsafecell to avoid unsafe access of global variable
#[allow(unused)]
static mut OSTCBTBL: [OsTCB;OS_MAX_TASKS+OS_N_SYS_TASKS+1] = [OsTCB {
    sp: 0 as OsStkPtr,
    ostcb_next: None,
    ostcb_pre: None,
    os_prio: 0,
    os_tcb_bitx: 0,
    os_tcb_bity: 0,
}; OS_MAX_TASKS+OS_N_SYS_TASKS+1];

/// the linked table of the free TCBs
static mut OS_TCB_FREE_LIST:Option<OSTCBPtr> = None;

/// the linked table of the created TCBs
static mut OS_TCB_LIST:Option<OSTCBPtr> = None;

/// Get the tcb in the array by its prio.
#[allow(unused)]
static mut OS_TCB_PRIO_TBL: [Option<OSTCBPtr>; OS_LOWEST_PRIO+1] = [None;OS_LOWEST_PRIO+1];

/*********************************************************************************
                                   Bit Map       
**********************************************************************************/
/// the size of the bit map
const OS_BITMAP_TBL_SIZE:usize = (OS_LOWEST_PRIO/8 + 1) as usize;

/// OSRdyGrp, used by the bit map of the ready task
#[allow(unused)]
static mut OS_RDY_GRP:OSTCBPrio=0;

/// the bit map table of the ready task
#[allow(unused)]
static mut OS_RDY_TBL:[OSTCBPrio;OS_BITMAP_TBL_SIZE]=[0;OS_BITMAP_TBL_SIZE];

/*********************************************************************************
                                  Check Var       
**********************************************************************************/
/// The number of layers of the interrupt nesting
#[allow(unused)]
static mut OSINT_NESTING: u8 = 0;

/*********************************************************************************
                                Schedule Var        
**********************************************************************************/
/// The piority of the current task
#[allow(unused)]
static mut OS_PRIO_CUR:u8 = 0;

/// The piority of the new task
#[allow(unused)]
static mut OS_PRIO_HIGH_RDY:u8 = 0;

/// the TCP ptr of the current task
#[allow(unused)]
static mut OS_TCB_CUR:Option<OSTCBPtr>=None;

/// the TCP ptr of the new task
#[allow(unused)]
static mut OS_TCB_HIGH_RDY:Option<OSTCBPtr> = None;
 
/// the state of the OS Core
#[allow(unused)]
static mut OS_RUNNING:bool=false;


// static mut OSTaskCtr:u32=0;
// static mut OSCtxSwCtr:u32=0;
// static mut OSIdleCtr:u32=0;
// static mut OSTaskRegNextAvailID:u8=0;
/********************************************************************************
                                    Time 
*********************************************************************************/
#[allow(unused)]
static mut OS_TIME:u32 = 0;