use super::OsTCB;

pub type OSTCBPtr = *mut OsTCB;
pub type OSTCBPrio = u8;
pub type OSPRIOBIT = u8;
// 32 bit machine, the register bit is 32bit
pub type OsStk = usize;
pub type OsStkPtr = *mut OsStk;
pub type Task = fn();

/// define error state related to os
/*
*              OS_ERR_NONE                     if the function was successful.
*              OS_ERR_PRIO_EXIST               if the task priority already exist
*                                              (each task MUST have a unique priority).
*              OS_ERR_PRIO_INVALID             if the priority you specify is higher that the maximum
*                                              allowed (i.e. >= OS_LOWEST_PRIO)
*              OS_ERR_TASK_CREATE              if you tried to create a task from an ISR or the task already exist.
*              OS_ERR_ILLEGAL_CREATE_RUN_TIME  if you tried to create a task after safety critical
*                                              operation started.
*              OsErrNoFreeTCB                  if there is no free TCB
*/
#[derive(PartialEq)]
pub enum OsErrState {
    OsErrNone,
    OsErrPrioExist,
    OsErrPrioInvalid,
    OsErrTaskCreate,
    OsErrIllegalCreateRunTime,
    OsErrNoFreeTCB,
}