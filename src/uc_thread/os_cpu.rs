use super::{types::{ OsStkPtr, Task}, CONTEXT_STACK_SIZE};

/// the context structure store in stack
#[repr(C,align(8))]
struct UcStk {
    // below are the remaining part of the task's context
    r4: u32,    
    r5: u32,
    r6: u32,
    r7: u32,
    r8: u32,
    r9: u32,
    r10: u32,
    r11: u32,
    // below are stored when the interrupt occurs
    r0: u32,
    r1: u32,
    r2: u32,
    r3: u32,
    r12: u32,
    lr: u32,
    pc: u32,
    xpsr: u32,

}

/// initialize the stack of the task :simulation push
pub fn ostask_stk_init(task: Task,ptos:OsStkPtr)-> OsStkPtr {
    // we store the data in UcStk and then push it to the stack
    let ptos = unsafe {
        ptos.offset(-(CONTEXT_STACK_SIZE as isize) as isize)
    };
    let psp = ptos as *mut UcStk;
    // initialize the stack
    unsafe {
        (*psp).r0 = 0;
        (*psp).r1 = 0;
        (*psp).r2 = 0;
        (*psp).r3 = 0;
        (*psp).r4 = 0;
        (*psp).r5 = 0;
        (*psp).r6 = 0;
        (*psp).r7 = 0;
        (*psp).r8 = 0;
        (*psp).r9 = 0;
        (*psp).r10 = 0;
        (*psp).r11 = 0;
        (*psp).lr = 0;
        (*psp).pc = task as u32;
        (*psp).xpsr = 0x01000000;
    }
    psp as OsStkPtr
}