use super::OsTCB;

pub type OSTCBPtr = &'static OsTCB;
pub type OSTCBPrio = u8;
pub type OSPRIOBIT = u8;
// 32 bit machine, the register bit is 32bit
pub type OsStk = u32;
pub type OsStkPtr = *mut OsStk;
