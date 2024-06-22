//! The core of the uC/OS-II
use types::OsErrState;

#[allow(unused_imports)]
use super::*;
/// initialize the internals of uC/OS-II
#[allow(unused)]
pub fn os_init(){
    os_init_hook_begin();
    os_init_tcblist();
    os_init_event_list();
    os_mem_init();
    os_q_init();
}

/// start the uC/OS-II. should be called after the os_init and the creation of taskz
#[allow(unused)]
 pub fn os_start(){

}

/// init the tcb
#[allow(unused)]
pub fn os_tcb_init(prio: OSTCBPrio, ptos: OsStkPtr) -> OsErrState {
    
    OsErrState::OsErrNone
    
}

/// To be done
fn os_init_hook_begin(){

}

/// init the TCB list(OS_TCB_FREE_LIST&OS_TCB_LIST)
fn os_init_tcblist(){
    // can't use OSTCBPtr because it's not mut
    let mut ptcb1:&mut OsTCB;
    let mut ptcb2:&mut OsTCB;
    for i in 0..(OS_MAX_TASKS+OS_N_SYS_TASKS-1){
        unsafe{
            // there is no need to set the ostcb_next because it will be set when TCB's init
            ptcb1=&mut OSTCBTBL[i];
            ptcb2=&mut OSTCBTBL[i+1];
        }
        ptcb1.ostcb_next=Some(ptcb2); 
    }
    // set the OS_TCB_FREE_LIST.
    // there is no need to set the OSTCBList and ostcb_next of the last task for the default value.
    unsafe{
        OS_TCB_FREE_LIST=Some(&OSTCBTBL[0]);
    }
}

/// may use in the future
fn os_init_event_list(){

}

/// may use in the future
fn os_mem_init(){

}

/// may use in the future
fn os_q_init(){

}

/// the task of idle
fn OS_TaskIdle(){
    // for (;;){
    //     OS_ENTER_CRITICAL();
    //     OSIdleCtr++;
    //     OS_EXIT_CRITICAL();
    //     OSTaskIdleHook(); /* Call user definable HOOK                           */
    // }
}

/// init the idle task
fn os_init_task_idle(){
    // os_task_create(,,OS_TASK_IDLE_PRIO);
    
}

