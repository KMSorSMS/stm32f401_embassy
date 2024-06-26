//! the basic structure for thread,like TCB and stack
mod os_core;
mod os_cpu;
mod os_task;
mod types;

use core::ptr::addr_of_mut;

pub use types::{OSTCBPrio, OSTCBPtr, OsStk, OsStkPtr, OSPRIOBIT};
pub use os_core::{os_init, os_start};
pub use os_task::os_task_create;
pub use os_cpu::systick_init;

#[repr(C)]
#[allow(unused)]
#[derive(Clone, Copy)]
pub struct OsTCB {
    /// stack pointer contains the basic register info
    sp: OsStkPtr,
    /// to make it a double linked list,need OSTCBNext and OSTCBPre
    ostcb_next: Option<OSTCBPtr>,
    ostcb_pre: Option<OSTCBPtr>,
    /// the priority of the thread
    os_prio: OSTCBPrio,
    /// the stride of the task
    stride:usize,
    /// to accomplish the real-time requirement, adding bit map
    os_tcb_bitx: OSPRIOBIT,
    os_tcb_bity: OSPRIOBIT,
    os_tcb_x: OSPRIOBIT,
    os_tcb_y: OSPRIOBIT,
}
unsafe impl Sync for OsTCB {}
// unsafe impl Send for OsTCB {}
/********************************************************************************
                                   Globle Settings
*********************************************************************************/
/// the lowest priority. For using in the array declare, OS_LOWEST_PRIO must be uszie
const OS_LOWEST_PRIO: usize = 63;
/// the prio of idle, the prio of the task must be u8
const OS_TASK_IDLE_PRIO: u8 = OS_LOWEST_PRIO as u8;
/// the size of the context stack(unit is 32bit)
const CONTEXT_STACK_SIZE: usize = 16;
/// the number of user's task
const OS_MAX_TASKS: usize = 20;
/// the number of system's task(now just idle)
const OS_N_SYS_TASKS: usize = 1;
/// the stride num of scheduling algorithm
const OS_STRIDE_NUM: usize = 0x8000;


/********************************************************************************
                                    TCBs' List
*********************************************************************************/
/// create a static OSTCB linked list we wrap it in a upsafecell to avoid unsafe access of global variable
#[allow(unused)]
static mut OSTCBTBL_ORIGIN: [OsTCB; OS_MAX_TASKS + OS_N_SYS_TASKS + 1] = [OsTCB {
    sp: 0 as OsStkPtr,
    ostcb_next: None,
    ostcb_pre: None,
    os_prio: 0,
    stride:0,
    os_tcb_bitx: 0,
    os_tcb_bity: 0,
    os_tcb_x: 0,
    os_tcb_y: 0,
}; OS_MAX_TASKS + OS_N_SYS_TASKS + 1];

#[allow(unused)]
static mut OSTCBTBL: OSTCBPtr = unsafe { addr_of_mut!(OSTCBTBL_ORIGIN[0]) };

/// the linked table of the free TCBs
static mut OS_TCB_FREE_LIST: OSTCBPtr = core::ptr::null_mut();

/// the linked table of the created TCBs
static mut OS_TCB_LIST: OSTCBPtr = core::ptr::null_mut();

/// Get the tcb in the array by its prio.
#[allow(unused)]
static mut OS_TCB_PRIO_TBL: [OSTCBPtr; OS_LOWEST_PRIO + 1] = [core::ptr::null_mut(); OS_LOWEST_PRIO + 1];
/// the pointer of the OS_TCB_PRIO_TBL
// #[allow(unused)]
// static mut OS_TCB_PRIO_TBL: *mut OSTCBPtr = unsafe { addr_of_mut!(OS_TCB_PRIO_TBL_ORIGIN[0])};

/*********************************************************************************
                                   Bit Map
**********************************************************************************/
/// the size of the bit map
const OS_BITMAP_TBL_SIZE: usize = (OS_LOWEST_PRIO / 8 + 1) as usize;

/// OSRdyGrp, used by the bit map of the ready task
#[allow(unused)]
static mut OS_RDY_GRP: OSTCBPrio = 0;
// static mut OS_RDY_GRP: *mut OSTCBPrio = unsafe { addr_of_mut!(OS_RDY_GRP_ORIGIN) };

/// the bit map table of the ready task
#[allow(unused)]
static mut OS_RDY_TBL_ORIGIN: [OSTCBPrio; OS_BITMAP_TBL_SIZE] = [0; OS_BITMAP_TBL_SIZE];
#[allow(unused)]
static mut OS_RDY_TBL: *mut OSTCBPrio = unsafe { addr_of_mut!(OS_RDY_TBL_ORIGIN[0]) };

/// Index into table is bit pattern to resolve highest priority Indexed value corresponds to highest priority bit position (i.e. 0..7)
const OS_UNMAP_TBL: [u8; 256] = [
    0, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0, /* 0x00 to 0x0F                   */
    4, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0, /* 0x10 to 0x1F                   */
    5, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0, /* 0x20 to 0x2F                   */
    4, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0, /* 0x30 to 0x3F                   */
    6, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0, /* 0x40 to 0x4F                   */
    4, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0, /* 0x50 to 0x5F                   */
    5, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0, /* 0x60 to 0x6F                   */
    4, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0, /* 0x70 to 0x7F                   */
    7, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0, /* 0x80 to 0x8F                   */
    4, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0, /* 0x90 to 0x9F                   */
    5, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0, /* 0xA0 to 0xAF                   */
    4, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0, /* 0xB0 to 0xBF                   */
    6, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0, /* 0xC0 to 0xCF                   */
    4, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0, /* 0xD0 to 0xDF                   */
    5, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0, /* 0xE0 to 0xEF                   */
    4, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0, /* 0xF0 to 0xFF                   */
];

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
#[no_mangle]
static mut OS_PRIO_CUR: u8 = 0;

// #[allow(unused)]
// static mut OS_PRIO_CUR: *mut u8 = unsafe { addr_of_mut!(OS_PRIO_CUR_ORIGIN) };

/// The piority of the new task
#[allow(unused)]
#[no_mangle]
static mut OS_PRIO_HIGH_RDY: u8 = 0;

// #[allow(unused)]
// static mut OS_PRIO_HIGH_RDY: *mut u8 = unsafe { addr_of_mut!(OS_PRIO_HIGH_RDY_ORIGIN) };

/// the TCP ptr of the current task
#[allow(unused)]
#[no_mangle]
static mut OS_TCB_CUR: OSTCBPtr = core::ptr::null_mut();

/// the TCP ptr of the new task
#[allow(unused)]
#[no_mangle]
static mut OS_TCB_HIGH_RDY: OSTCBPtr = core::ptr::null_mut();

/// the state of the OS Core
#[allow(unused)]
#[no_mangle]
static mut OS_IS_RUNNING: bool = false;

// static mut OSTaskCtr:u32=0;
// static mut OSCtxSwCtr:u32=0;
// static mut OSIdleCtr:u32=0;
// static mut OSTaskRegNextAvailID:u8=0;
/********************************************************************************
                                    Time
*********************************************************************************/
// #[allow(unused)]
// static mut OS_TIME: u32 = 0;
// the tick num of a time slice
#[allow(unused)]
const OS_TICKS_PER_SEC:usize=10;

/********************************************************************************
                            Stack Size & Stack
*********************************************************************************/
/// the size of idle task's stack
const OS_TASK_IDLE_STK_SIZE: usize = 128;
/// the stack of the idle task
static mut OS_TASK_IDLE_STK: [OsStk; OS_TASK_IDLE_STK_SIZE] = [0; OS_TASK_IDLE_STK_SIZE];

/// OS_CPU_EXCEPT_STK_BASE size
const OS_CPU_EXCEPT_STK_BASE_SIZE: usize = 256;

#[allow(unused)]
#[no_mangle]
static mut OS_CPU_EXCEPT_STK_BASE_ARRAY: [OsStk; OS_CPU_EXCEPT_STK_BASE_SIZE] = [0; OS_CPU_EXCEPT_STK_BASE_SIZE];
#[allow(unused)]
#[no_mangle]
static mut OS_CPU_EXCEPT_STK_BASE: OsStkPtr = unsafe { addr_of_mut!(OS_CPU_EXCEPT_STK_BASE_ARRAY[OS_CPU_EXCEPT_STK_BASE_SIZE-1]) };

// /// the size of task1's stack
// const OS_TASK1_STK_SIZE:usize = 128;
// /// the stack of task1
// static mut OS_TASK1_STK:[OsStk; OS_TASK1_STK_SIZE] = [0;OS_TASK1_STK_SIZE];

// /// the size of task2's stack
// const OS_TASK2_STK_SIZE:usize = 128;
// /// the stack of task2
// static mut OS_TASK2_STK:[OsStk; OS_TASK2_STK_SIZE] = [0;OS_TASK2_STK_SIZE];

// seem to not be used
// /// the size of exception stack
// const OS_CPU_EXCEPT_STK_SIZE:usize=256;
// /// the stack of exception
// static mut OS_CPU_EXCEPT_STK:[OsStk; OS_CPU_EXCEPT_STK_SIZE] = [0;OS_CPU_EXCEPT_STK_SIZE];
