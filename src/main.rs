#![no_std]
#![no_main]
// #![feature(impl_trait_in_assoc_type)]
// use the asm code.
// the rust version of uc's thread part
pub mod uc_thread;

use cortex_m_rt::entry;
use defmt::*;
use embassy_executor::Executor;
use embassy_stm32::rcc::Pll;
// use embassy_time::Timer;
use static_cell::StaticCell;
use stm32_metapac::rcc::vals;
use uc_thread::{os_init, os_start, os_task_create, systick_init, OsStk};

use {defmt_rtt as _, panic_probe as _};
static EXECUTOR_LOW1: StaticCell<Executor> = StaticCell::new();
static EXECUTOR_LOW2: StaticCell<Executor> = StaticCell::new();
const TASK1_STK_SIZE: usize = 1024;
const TASK2_STK_SIZE: usize = 1024;
static mut TASK1_STK: [OsStk; TASK1_STK_SIZE] = [0; TASK1_STK_SIZE];
static mut TASK2_STK: [OsStk; TASK2_STK_SIZE] = [0; TASK2_STK_SIZE];


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
    let _p = embassy_stm32::init(config);
    systick_init(84000000);
    info!("Hello World!");
    os_init();
    info!("task_1");
    unsafe{os_task_create(task_1, &mut TASK1_STK[TASK1_STK_SIZE-1], 12);}
    info!("task_2");
    unsafe{os_task_create(task_2, &mut TASK2_STK[TASK2_STK_SIZE-1], 11);}
    os_start()
}

fn task_1() {
    let executor = EXECUTOR_LOW1.init(Executor::new());
    executor.run(|spawner| {
        // unwrap!(spawner.spawn(blink2()));
        unwrap!(spawner.spawn(blink1()));
    });
}

fn task_2() {
    let executor = EXECUTOR_LOW2.init(Executor::new());
    executor.run(|spawner| {
        unwrap!(spawner.spawn(blink3()));
    });
}

#[embassy_executor::task]
async fn blink1() {
    loop {
        info!("high1");
        // led.set_high();
        // Timer::after_millis(300).await;
        block_delay(1000);

        info!("low1");
        // led.set_low();
        // Timer::after_millis(300).await;
        block_delay(1000);

    }
}

#[embassy_executor::task]
async fn blink2() {
    loop {
        info!("high2");
        // led.set_high();
        // Timer::after_millis(300).await;
        block_delay(100);

        info!("low2");
        // led.set_low();
        block_delay(100);
        // Timer::after_millis(300).await;
    }
}

#[embassy_executor::task]
async fn blink3() {
    loop {
        info!("high3");
        // led.set_high();
        // Timer::after_millis(300).await;
        block_delay(1000);
        info!("low3");
        // led.set_low();
        // Timer::after_millis(300).await;
        block_delay(1000);
    }
}

fn block_delay(tick: u32){
    let mut i = 0;
    let mut j = 0;
    while i < tick {
        i += 1;
        while j < tick*tick {
            j += 1;
        }
    }
} 