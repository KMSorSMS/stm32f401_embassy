use super::{OsTCB, CONTEXT_STACK_SIZE};

pub type OSTCBPtr<'a> = &'a OsTCB<'a>;
pub type OSTCBPrio = u8;
pub type OSPRIOBIT = u8;
// 32 bit machine, the register bit is 32bit
pub type OsStk = u32;
pub type OsStkPtr = *mut OsStk;
pub type Task = fn();

/// define error state related to os
/*
*              OS_ERR_NONE                     if the function was successful.
*              OS_ERR_PRIO_EXIST               if the task priority already exist
*                                              (each task MUST have a unique priority).
*              OS_ERR_PRIO_INVALID             if the priority you specify is higher that the maximum
*                                              allowed (i.e. >= OS_LOWEST_PRIO)
*              OS_ERR_TASK_CREATE_ISR          if you tried to create a task from an ISR.
*              OS_ERR_ILLEGAL_CREATE_RUN_TIME  if you tried to create a task after safety critical
*                                              operation started.
*/
pub enum OsErrState {
    OsErrNone,
    OsErrPrioExist,
    OsErrPrioInvalid,
    OsErrTaskCreateIsr,
    OsErrIllegalCreateRunTime,
}
