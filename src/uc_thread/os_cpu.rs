use core::arch::asm;

use super::{
    os_core::{os_int_enter, os_int_exit},
    types::{OsStkPtr, Task},
    CONTEXT_STACK_SIZE, OS_TICKS_PER_SEC,
};
use cortex_m::Peripherals;
use cortex_m_rt::exception;
// use defmt::info;
extern crate cortex_m_rt;
// import asm func
extern "C" {
    pub fn PendSV_Handler();
}

/// the context structure store in stack
#[repr(C, align(8))]
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
    r14: u32,
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
pub fn ostask_stk_init(task: Task, ptos: OsStkPtr) -> OsStkPtr {
    // we store the data in UcStk and then push it to the stack
    let ptos = unsafe { ptos.offset(-(CONTEXT_STACK_SIZE as isize) as isize) };
    let psp = ptos as *mut UcStk;
    // initialize the stack
    unsafe {
        (*psp).r0 = 0;
        (*psp).r1 = 0x01010101;
        (*psp).r2 = 0x02020202;
        (*psp).r3 = 0x03030303;
        (*psp).r4 = 0x04040404;
        (*psp).r5 = 0x05050505;
        (*psp).r6 = 0x06060606;
        (*psp).r7 = 0x07070707;
        (*psp).r8 = 0x08080808;
        (*psp).r9 = 0x09090909;
        (*psp).r10 = 0x10101010;
        (*psp).r11 = 0x11111111;
        (*psp).r12 = 0x12121212;
        (*psp).r14 = 0xFFFFFFFD;
        (*psp).lr = 0;
        (*psp).pc = task as u32;
        (*psp).xpsr = 0x01000000;
    }
    psp as OsStkPtr
}

pub fn systick_init(cpu_freq: usize) {
    let cnts: u32 = (cpu_freq / OS_TICKS_PER_SEC) as u32;
    let mut p = Peripherals::take().unwrap();
    // get the register block of systick
    let mut stk = p.SYST;
    // set the reload val
    stk.set_reload(cnts - 1);
    // set the systick handler prio this need to use the register:SCB_SHPRI3
    unsafe { p.SCB.set_priority(cortex_m::peripheral::scb::SystemHandler::SysTick, 2) };

    // clear the current value
    stk.clear_current();
    // set the source of the systick
    // SYST::set_clock_source(SystClkSource::External);
    // enable the timer interrupt
    stk.enable_interrupt();
    // enable the systick
    stk.enable_counter();
}

/// the systick handler
/// the func is not pub for the requirement of the exception
#[exception]
fn SysTick() {
    // info!("systick");
    os_int_enter();
    ostime_tick();
    os_int_exit();
}

/// the pendsv hadler
#[exception]
fn PendSV() {
    // info!("PendSV");
    unsafe {
        PendSV_Handler();
    }
    // info!("end of PendSV");
}

/// the tick func of the os. In this func the
/// If the func is called, there MUST be a scheduling point.
#[inline]
fn ostime_tick() {
    // wake up the MCU(maybe the MCU is not in low power mode anyway)
    // In blinky, though sev instruction is called, the ISR will be executed continue.
    // Maybe it is because the cs?
    // there are two situation that this func is called:
    // 1. there is no thread to run (now the MCU should be in low power mode)
    // 2. there is still some threads can be sheduled, but the MCU is in low power mode. at this time, we should wake up the MCU.
    // But we consider that the wfe time is also the time of the thread.
    critical_section::with(|_cs| {
        unsafe { asm!("sev") };
    });
    // there is nothing to do. I will chage the scheduling algorithm in os_sched
}
