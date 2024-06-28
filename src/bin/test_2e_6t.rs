//! this test is used to test the stride.
//! block_delay(100) is about 1ms
#![no_std]
#![no_main]
// #![feature(impl_trait_in_assoc_type)]
// use the asm code.
// the rust version of uc's thread part
#[path ="../uc_thread/mod.rs"]
pub mod uc_thread;

use cortex_m_rt::entry;
use defmt::*;
use embassy_executor::Executor;
use embassy_stm32::rcc::Pll;
use embassy_time::{Instant, Timer};
use static_cell::StaticCell;
use stm32_metapac::rcc::vals;
use uc_thread::{os_init, os_start, os_task_create, systick_init, OsStk};

use {defmt_rtt as _, panic_probe as _};
static EXECUTOR_LOW1: StaticCell<Executor> = StaticCell::new();
static EXECUTOR_LOW2: StaticCell<Executor> = StaticCell::new();
const TASK1_STK_SIZE: usize = 512;
const TASK2_STK_SIZE: usize = 512;
static mut TASK1_STK: [OsStk; TASK1_STK_SIZE] = [0; TASK1_STK_SIZE];
static mut TASK2_STK: [OsStk; TASK2_STK_SIZE] = [0; TASK2_STK_SIZE];
const PRINT_TIME:usize = 20;

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
    unsafe{os_task_create(task_1, &mut TASK1_STK[TASK1_STK_SIZE-1], 43);}
    info!("task_2");
    unsafe{os_task_create(task_2, &mut TASK2_STK[TASK2_STK_SIZE-1], 3);}
    os_start()
}

fn task_1() {
    let executor = EXECUTOR_LOW1.init(Executor::new());
    executor.run(|spawner| {
        unwrap!(spawner.spawn(dead_task1()));
        unwrap!(spawner.spawn(blink1_1()));
        unwrap!(spawner.spawn(blink1_2()));
    });
}

fn task_2() {
    let executor = EXECUTOR_LOW2.init(Executor::new());
    executor.run(|spawner| {
        unwrap!(spawner.spawn(dead_task2()));
        unwrap!(spawner.spawn(blink2_1()));
        unwrap!(spawner.spawn(blink2_2()));
    });
}

#[embassy_executor::task]
async fn dead_task1() {
    let mut count_dead1:usize=0;
    let start = Instant::now();
    loop{
        count_dead1+=1;
        Timer::after_millis(1).await;
        block_delay(10);
        if count_dead1 % PRINT_TIME==0{
            let end = Instant::now();
            let ms = end.duration_since(start).as_millis();
            info!("task0 end with times:{}", count_dead1);
            info!("task0 execute time:{}ms", ms);
        }
    }
}

#[embassy_executor::task]
async fn dead_task2(){
    let mut count_dead2:usize=0;
    let start = Instant::now();
    loop{
        count_dead2+=1;
        Timer::after_millis(1).await;
        block_delay(10);
        if count_dead2 % PRINT_TIME==0{
            let end = Instant::now();
            let ms = end.duration_since(start).as_millis();
            info!("task0 end with times:{}", count_dead2);
            info!("task0 execute time:{}ms", ms);
        }
    }
}

#[embassy_executor::task]
async fn blink1_1() {
    let mut count1_times:usize = 0;
    let start = Instant::now();
    loop {
        count1_times += 1;
        info!("high1");
        // led.set_high();
        Timer::after_millis(1).await;

        info!("low1");
        block_delay(10);
        if count1_times % PRINT_TIME==0{
            let end = Instant::now();
            let ms = end.duration_since(start).as_millis();
            info!("task1 end with times:{}", count1_times);
            info!("task1 execute time:{}ms", ms);
        }
    }
}

#[embassy_executor::task]
async fn blink1_2() {
    let mut count1_times:usize = 0;
    let start = Instant::now();
    loop {
        count1_times += 1;
        info!("high1");
        // led.set_high();
        Timer::after_millis(1).await;

        info!("low1");
        block_delay(10);
        if count1_times % PRINT_TIME==0{
            let end = Instant::now();
            let ms = end.duration_since(start).as_millis();
            info!("task2 end with times:{}", count1_times);
            info!("task2 execute time:{}ms", ms);
        }
    }
}

#[embassy_executor::task]
async fn blink2_1() {
    let mut count1_times:usize = 0;
    let start = Instant::now();
    loop {
        count1_times += 1;
        info!("high1");
        // led.set_high();
        Timer::after_millis(1).await;
        info!("low1");
        block_delay(10);
        if count1_times % PRINT_TIME==0 {
            let end = Instant::now();
            let ms = end.duration_since(start).as_millis();
            info!("task1 end with times:{}", count1_times);
            info!("task1 execute time:{}ms", ms);
        }
    }
}

#[embassy_executor::task]
async fn blink2_2() {
    let mut count1_times:usize = 0;
    let start = Instant::now();
    loop {
        count1_times += 1;
        info!("high1");
        // led.set_high();
        Timer::after_millis(1).await;
        
        info!("low1");
        block_delay(10);
        if count1_times % PRINT_TIME==0 {
            let end = Instant::now();
            let ms = end.duration_since(start).as_millis();
            info!("task2 end with times:{}", count1_times);
            info!("task2 execute time:{}ms", ms);
        }
    }
}

#[allow(unused)]
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