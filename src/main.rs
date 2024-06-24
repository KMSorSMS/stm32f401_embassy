#![no_std]
#![no_main]
// #![feature(impl_trait_in_assoc_type)]
// use the asm code.
// the rust version of uc's thread part
pub mod uc_thread;
use core::ptr::addr_of_mut;

use cortex_m_rt::entry;
use defmt::*;
use embassy_executor::{Executor, Spawner};
use embassy_stm32::{
    gpio::{Level, Output, Speed},
    rcc::Pll,
    Config,
};
use embassy_time::Timer;
use static_cell::StaticCell;
use stm32_metapac::rcc::vals;
use uc_thread::{os_init, os_start, os_task_create, OsStk};

use {defmt_rtt as _, panic_probe as _};
static EXECUTOR_LOW1: StaticCell<Executor> = StaticCell::new();
static EXECUTOR_LOW2: StaticCell<Executor> = StaticCell::new();
static LED: StaticCell<Output<'static>> = StaticCell::new();
const TASK1_STK_SIZE: usize = 128;
const TASK2_STK_SIZE: usize = 128;
static mut TASK1_STK: [OsStk; TASK1_STK_SIZE] = [0; TASK1_STK_SIZE];
static mut TASK1_STK_PTR: *mut OsStk = unsafe { addr_of_mut!(TASK1_STK[0]) };
static mut TASK2_STK: [OsStk; TASK2_STK_SIZE] = [0; TASK2_STK_SIZE];
static mut TASK2_STK_PTR: *mut OsStk = unsafe { addr_of_mut!(TASK2_STK[0]) };

#[entry]
fn main() -> ! {
    let hse = Some(embassy_stm32::rcc::Hse {
        freq: embassy_stm32::time::Hertz(8_000_000),
        mode: embassy_stm32::rcc::HseMode::Oscillator,
    });
    let pll = Some(Pll {
        prediv: vals::Pllm::DIV4,
        mul: vals::Plln::MUL84,
        divp: Some(vals::Pllp::DIV2),
        divq: Some(vals::Pllq::DIV4),
        divr: None,
    });

    let mut rcc = embassy_stm32::rcc::Config::default();
    // config the default mannually, its dull
    rcc.hsi = false;
    rcc.hse = hse;
    rcc.sys = vals::Sw::PLL1_P;
    rcc.pll_src = vals::Pllsrc::HSE;
    rcc.pll = pll;
    rcc.ahb_pre = vals::Hpre::DIV1;
    rcc.apb1_pre = vals::Ppre::DIV2;
    rcc.apb2_pre = vals::Ppre::DIV2;

    let mut config = embassy_stm32::Config::default();
    config.rcc = rcc;
    systick_init();
    info!("Hello World!");
    os_init();
    os_task_create(task_1, unsafe { TASK1_STK_PTR }, 10);
    os_task_create(task_2, unsafe { TASK2_STK_PTR }, 10);
    os_start()
}

fn task_1() {
    let executor = EXECUTOR_LOW1.init(Executor::new());
    executor.run(|spawner| {
        unwrap!(spawner.spawn(blink1()));
    });
}

fn task_2() {
    let executor = EXECUTOR_LOW2.init(Executor::new());
    executor.run(|spawner| {
        unwrap!(spawner.spawn(blink2()));
    });
}

#[embassy_executor::task]
async fn blink1() {
    loop {
        info!("high1");
        // led.set_high();
        Timer::after_millis(300).await;

        info!("low1");
        // led.set_low();
        Timer::after_millis(300).await;
    }
}

#[embassy_executor::task]
async fn blink2() {
    loop {
        info!("high2");
        // led.set_high();
        Timer::after_millis(300).await;

        info!("low2");
        // led.set_low();
        Timer::after_millis(300).await;
    }
}

fn systick_init() {}
