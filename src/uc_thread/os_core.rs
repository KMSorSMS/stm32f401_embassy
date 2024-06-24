//! The core of the uC/OS-II
#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use core::{
    arch::asm,
    ptr::{addr_of, addr_of_mut},
};
use os_task::os_task_create;
use types::OsErrState;
use core::arch::global_asm;

global_asm!(include_str!("os_cpu.S"));

// import asm func
extern "C" {
    pub fn OSStartHighRdy()->!;
    pub fn OSCtxSw();
    pub fn OSIntCtxSw();
}

/// initialize the internals of uC/OS-II
#[allow(unused)]
pub fn os_init() {
    os_init_tcblist();
    os_init_event_list();
    os_mem_init();
    os_q_init();
    os_init_task_idle();
}

/// start the uC/OS-II. should be called after the os_init and the creation of taskz
#[allow(unused)]
pub fn os_start() -> !{
    // when the os just starts, there is no need to use cs
    if !unsafe { OS_IS_RUNNING } {
        // os_sched_new();
        let mut y: u8 = 0;
        unsafe {
            // for it is the first time to run the os, so schedule the task by prio
            y = OS_UNMAP_TBL[unsafe { OS_RDY_GRP } as usize];
            OS_PRIO_HIGH_RDY = (y << 3) + OS_UNMAP_TBL[*(OS_RDY_TBL.add(y as usize)) as usize];
            // (*OS_TCB_HIGH_RDY).stride+=OS_STRIDE_NUM/(*OS_TCB_HIGH_RDY).os_prio as usize;
            OS_PRIO_CUR = OS_PRIO_HIGH_RDY;
            OS_TCB_HIGH_RDY = OS_TCB_PRIO_TBL[OS_PRIO_HIGH_RDY as usize];
            OS_TCB_CUR = OS_TCB_HIGH_RDY;
            OSStartHighRdy()
        }
    }else{
        loop {
            unsafe {
                asm!("wfe");
            }
        }
    }
}

/// init the tcb
#[allow(unused)]
pub fn os_tcb_init(prio: OSTCBPrio, ptos: OsStkPtr) -> OsErrState {
    // pick a free TCB from the free list
    let ptcb = critical_section::with(|cs| {
        if unsafe { OS_TCB_FREE_LIST.as_ref().is_none() } {
            return None;
        } else {
            let ptcb = unsafe { OS_TCB_FREE_LIST };
            unsafe { OS_TCB_FREE_LIST = unsafe { (*OS_TCB_FREE_LIST).ostcb_next.unwrap() } }
            return Some(ptcb);
        }
    });
    // if there is no free TCB, return an error
    if ptcb.is_none() {
        return OsErrState::OsErrNoFreeTCB;
    }
    // init the TCB
    let ptcb = ptcb.unwrap();
    unsafe {
        (*ptcb).os_prio = prio;
        (*ptcb).sp = ptos;
        (*ptcb).ostcb_next = None;
        (*ptcb).ostcb_pre = None;
        (*ptcb).os_tcb_y = prio >> 3;
        (*ptcb).os_tcb_x = prio & 0x07;
        (*ptcb).os_tcb_bity = 1 << (*ptcb).os_tcb_y;
        (*ptcb).os_tcb_bitx = 1 << (*ptcb).os_tcb_x;
    }
    // update OS_Prio_TBL
    critical_section::with(|_cs| unsafe {
        OS_TCB_PRIO_TBL[prio as usize] = ptcb;
    });
    // add the tcb to the allocated queue which is the OSTCBList
    critical_section::with(|_cs| unsafe {
        (*ptcb).ostcb_next = Some(OS_TCB_LIST);
        (*ptcb).ostcb_pre = None;
        if (!OS_TCB_LIST.is_null()) {
            (*OS_TCB_LIST).ostcb_pre = Some(ptcb);
        }
        OS_TCB_LIST = ptcb;
        // set the RgyGrp and RdyTbl
        OS_RDY_GRP |= (*ptcb).os_tcb_bity;
        // set the bit in the ready table
        OS_RDY_TBL.offset((*ptcb).os_tcb_y as isize).write((*ptcb).os_tcb_bitx);
        OsErrState::OsErrNone   
    })

}

/// shedule the task
#[allow(unused)]
pub fn os_sched() {
    // need a cs
    critical_section::with(|cs| {
        if unsafe { OSINT_NESTING } == 0 {
            os_sched_new();
            unsafe {
                OS_TCB_HIGH_RDY = OS_TCB_PRIO_TBL[OS_PRIO_HIGH_RDY as usize];
                // the new task is no the old task, need to sw
                if OS_PRIO_CUR != OS_PRIO_HIGH_RDY {
                    (*OS_TCB_HIGH_RDY).stride+=OS_STRIDE_NUM/(*OS_TCB_HIGH_RDY).os_prio as usize;
                    OSCtxSw();
                }
            }
        }
    });
}

/// is called when enter an ISR
#[allow(unused)]
pub fn os_int_enter() {
    // need a cs
    critical_section::with(|_cs| {
        unsafe {
            // less than 255 to void overflow
            if OS_IS_RUNNING && OSINT_NESTING < 255 {
                OSINT_NESTING += 1;
            }
        }
    });
}

/// is called when exit an ISR
#[allow(unused)]
pub fn os_int_exit() {
    // need a cs
    critical_section::with(|_cs| unsafe {
        if OS_IS_RUNNING && OSINT_NESTING > 0 {
            OSINT_NESTING -= 1;
        }
        if OSINT_NESTING == 0 {
            os_sched_new();
            OS_TCB_HIGH_RDY = OS_TCB_PRIO_TBL[OS_PRIO_HIGH_RDY as usize];
            if OS_PRIO_CUR != OS_PRIO_HIGH_RDY {
                // update the stride 
                (*OS_TCB_HIGH_RDY).stride+=OS_STRIDE_NUM/(*OS_TCB_HIGH_RDY).os_prio as usize;
                OSIntCtxSw();
            }
        }
    });
}

/// maybe no need?
#[allow(unused)]
fn os_time_tick() {}

/// init the TCB list(OS_TCB_FREE_LIST&OS_TCB_LIST)
fn os_init_tcblist() {
    // can't use OSTCBPtr because it's not mut
    let mut ptcb1: OSTCBPtr;
    let mut ptcb2: OSTCBPtr;
    for i in 0..(OS_MAX_TASKS + OS_N_SYS_TASKS - 1) {
        unsafe {
            // there is no need to set the ostcb_next because it will be set when TCB's init
            ptcb1 = OSTCBTBL.add(i);
            ptcb2 = OSTCBTBL.add(i + 1);
            (*ptcb1).ostcb_next = Some(ptcb2);
        }
    }
    // set the OS_TCB_FREE_LIST.
    // there is no need to set the OSTCBList and ostcb_next of the last task for the default value.
    unsafe {
        OS_TCB_FREE_LIST = OSTCBTBL;
    }
}

/// may use in the future
fn os_init_event_list() {}

/// may use in the future
fn os_mem_init() {}

/// may use in the future
fn os_q_init() {}

/// the task of idle
fn os_task_idle() {
    // check the ready queue, if there is no task, then wfe.
    // here we need a cs
    loop {
        if critical_section::with(|_cs| {
            if unsafe { OS_RDY_GRP } != 0 {
                return false;
            } else {
                return true;
            }
        }) {
            unsafe {
                asm!("wfe");
            }
        }
    }
}

/// init the idle task
fn os_init_task_idle() {
    // for the idle task's stack is a global variable, so we need unsafe.
    // because this func is called in OS_init, so there is no need to use cs
    unsafe {
        os_task_create(os_task_idle, &mut OS_TASK_IDLE_STK[0], OS_TASK_IDLE_PRIO);
    }
}

/// find highest priority's task priority number
/// change this func to stride scheduling algorithm
fn os_sched_new() {
    #[cfg(feature="bitmap")]
    {
        // now the OS_LOWEST_PRIO will not be large than 63
        let mut y: u8 = 0;
        // need a cs
        critical_section::with(|_cs| {
            y = OS_UNMAP_TBL[unsafe { OS_RDY_GRP } as usize];
            unsafe {
                OS_PRIO_HIGH_RDY = (y << 3) + OS_UNMAP_TBL[*(OS_RDY_TBL.add(y as usize)) as usize];
            }
        });
    }
    
    #[cfg(feature = "stride")]
    {
        // need a cs
        critical_section::with(|_cs| {
            unsafe {
                // init min_stride & OS_PRIO_HIGH_RDY
                let mut min_stride=usize::MAX;
                let mut ptr:OSTCBPtr = OS_TCB_LIST;
                while !ptr.is_null() {
                    if min_stride>(*ptr).stride {
                        OS_PRIO_HIGH_RDY = (*ptr).os_prio;
                        min_stride=(*ptr).stride;
                    }
                    ptr = (*ptr).ostcb_next.unwrap();
                }
            }
        });
    }
}
