#![no_std]
#![no_main]
// #![feature(impl_trait_in_assoc_type)]
// use the asm code.
// the rust version of uc's thread part
#[path = "../uc_thread/mod.rs"]
pub mod uc_thread;

use cortex_m_rt::entry;
use defmt::*;
use embassy_executor::Executor;
use embassy_stm32::rcc::Pll;
use embassy_time::Timer;
use static_cell::StaticCell;
use stm32_metapac::rcc::vals;
use uc_thread::{os_init, os_start, os_task_create, systick_init, OsStk};

use {defmt_rtt as _, panic_probe as _};
static EXECUTOR_LOW1: StaticCell<Executor> = StaticCell::new();
static EXECUTOR_LOW2: StaticCell<Executor> = StaticCell::new();
static EXECUTOR_LOW3: StaticCell<Executor> = StaticCell::new();
const TASK1_STK_SIZE: usize = 512;
const TASK2_STK_SIZE: usize = 512;
const TASK3_STK_SIZE: usize = 512;
static mut TASK1_STK: [OsStk; TASK1_STK_SIZE] = [0; TASK1_STK_SIZE];
static mut TASK2_STK: [OsStk; TASK2_STK_SIZE] = [0; TASK2_STK_SIZE];
static mut TASK3_STK: [OsStk; TASK3_STK_SIZE] = [0; TASK3_STK_SIZE];
// const LIMIT_TIME:usize = 25;
const PRINT_TIME: usize = 10;
// unit is ms
const LONG_SYSCALL_TIME: u64 = 45;
const MID_SYSCALL_TIME: u64 = 15;
const SHORT_SYSCALL_TIME: u64 = 5;
// unit is ms
const LONG_CALCULATE_TIME: u64 = 200;
const MID_CALCULATE_TIME: u64 = 100;
const SHORT_CALCULATE_TIME: u64 = 80;
const VERY_SHORT_CALCULATE_TIME: u64 = 70;

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
    unsafe {
        os_task_create(task_1, &mut TASK1_STK[TASK1_STK_SIZE - 1], 43);
    }
    info!("task_2");
    unsafe {
        os_task_create(task_2, &mut TASK2_STK[TASK2_STK_SIZE - 1], 3);
    }
    info!("task_3");
    unsafe {
        os_task_create(task_3, &mut TASK3_STK[TASK3_STK_SIZE - 1], 23);
    }
    os_start()
}

fn task_1() {
    let executor = EXECUTOR_LOW1.init(Executor::new());
    executor.run(|spawner| {
        unwrap!(spawner.spawn(mock_task2()));
        unwrap!(spawner.spawn(mock_task1()));
        unwrap!(spawner.spawn(mock_task3()));
        unwrap!(spawner.spawn(mock_task4()));
        unwrap!(spawner.spawn(mock_task5()));
        unwrap!(spawner.spawn(mock_task6()));
        // unwrap!(spawner.spawn(dead_task1()));
    });
}

fn task_2() {
    let executor = EXECUTOR_LOW2.init(Executor::new());
    executor.run(|spawner| {
        unwrap!(spawner.spawn(mock_task7()));
        unwrap!(spawner.spawn(mock_task8()));
        unwrap!(spawner.spawn(mock_task9()));
        unwrap!(spawner.spawn(mock_task10()));
        unwrap!(spawner.spawn(mock_task11()));
        unwrap!(spawner.spawn(mock_task12()));
        // unwrap!(spawner.spawn(dead_task2()));
    });
}

fn task_3(){
    let executor = EXECUTOR_LOW3.init(Executor::new());
    executor.run(|spawner| {
        unwrap!(spawner.spawn(mock_task13()));
        unwrap!(spawner.spawn(mock_task14()));
        unwrap!(spawner.spawn(mock_task15()));
        unwrap!(spawner.spawn(mock_task16()));
        unwrap!(spawner.spawn(mock_task17()));
        unwrap!(spawner.spawn(mock_task18()));
        // unwrap!(spawner.spawn(dead_task2()));
    });

}

#[embassy_executor::task]
async fn mock_task1() {
    let mut count1_times: usize = 0;
    loop {
        count1_times += 1;
        Timer::after_millis(MID_SYSCALL_TIME).await;
        block_delay(VERY_SHORT_CALCULATE_TIME);
        Timer::after_millis(SHORT_SYSCALL_TIME).await;
        block_delay(VERY_SHORT_CALCULATE_TIME);
        // 进行模10，每10次打印一次执行次数
        if count1_times % PRINT_TIME == 0 {
            info!("task_1_1 counted execute times:{}", count1_times);
        }
    }
}

#[embassy_executor::task]
async fn mock_task2() {
    let mut count2_times: usize = 0;
    loop {
        count2_times += 1;
        Timer::after_millis(LONG_SYSCALL_TIME).await;
        block_delay(LONG_CALCULATE_TIME);
        Timer::after_millis(LONG_SYSCALL_TIME).await;
        block_delay(LONG_CALCULATE_TIME);
        if count2_times % PRINT_TIME == 0 {
            info!("task_1_2 counted execute times:{}", count2_times);
        }
    }
}

#[embassy_executor::task]
async fn mock_task3() {
    // record it's execute time
    let mut count3_times: usize = 0;
    loop {
        count3_times += 1;
        Timer::after_millis(SHORT_SYSCALL_TIME).await;
        block_delay(SHORT_CALCULATE_TIME);
        Timer::after_millis(SHORT_SYSCALL_TIME).await;
        block_delay(SHORT_CALCULATE_TIME);
        if count3_times % PRINT_TIME == 0 {
            info!("task_1_3 counted execute times:{}", count3_times);
        }
    }
}

#[embassy_executor::task]
async fn mock_task4() {
    let mut count4_times: usize = 0;
    loop {
        count4_times += 1;
        Timer::after_millis(MID_SYSCALL_TIME).await;
        block_delay(MID_CALCULATE_TIME);
        Timer::after_millis(MID_SYSCALL_TIME).await;
        block_delay(MID_CALCULATE_TIME);
        if count4_times % PRINT_TIME == 0 {
            info!("task_1_4 counted execute times:{}", count4_times);
        }
    }
}

#[embassy_executor::task]
async fn mock_task5() {
    let mut count_times: usize = 0;
    loop {
        count_times += 1;
        Timer::after_millis(MID_SYSCALL_TIME).await;
        block_delay(LONG_CALCULATE_TIME);
        Timer::after_millis(MID_SYSCALL_TIME).await;
        block_delay(LONG_CALCULATE_TIME);
        if count_times % PRINT_TIME == 0 {
            info!("task_1_5 counted execute times:{}", count_times);
        }
    }
}


#[embassy_executor::task]
async fn mock_task6() {
    let mut count_times: usize = 0;
    loop {
        count_times += 1;
        Timer::after_millis(LONG_SYSCALL_TIME).await;
        block_delay(VERY_SHORT_CALCULATE_TIME);
        Timer::after_millis(LONG_SYSCALL_TIME).await;
        block_delay(VERY_SHORT_CALCULATE_TIME);
        if count_times % PRINT_TIME == 0 {
            info!("task_1_6 counted execute times:{}", count_times);
        }
    }
}


#[embassy_executor::task]
async fn mock_task7() {
    // record it's execute time
    let mut count5_times: usize = 0;
    loop {
        count5_times += 1;
        Timer::after_millis(MID_SYSCALL_TIME).await;
        block_delay(VERY_SHORT_CALCULATE_TIME);
        Timer::after_millis(SHORT_SYSCALL_TIME).await;
        block_delay(VERY_SHORT_CALCULATE_TIME);
        if count5_times % PRINT_TIME == 0 {
            info!("task_2_1 counted execute times:{}", count5_times);
        }
    }
}

#[embassy_executor::task]
async fn mock_task8() {
    // record it's execute time
    let mut count6_times: usize = 0;
    loop {
        count6_times += 1;
        Timer::after_millis(LONG_SYSCALL_TIME).await;
        block_delay(LONG_CALCULATE_TIME);
        Timer::after_millis(LONG_SYSCALL_TIME).await;
        block_delay(LONG_CALCULATE_TIME);
        if count6_times % PRINT_TIME == 0 {
            info!("task_2_2 counted execute times:{}", count6_times);
        }
    }
}

#[embassy_executor::task]
async fn mock_task9() {
    // record it's execute time
    let mut count7_times: usize = 0;
    loop {
        count7_times += 1;
        Timer::after_millis(SHORT_SYSCALL_TIME).await;
        block_delay(SHORT_CALCULATE_TIME);
        Timer::after_millis(SHORT_SYSCALL_TIME).await;
        block_delay(SHORT_CALCULATE_TIME);
        if count7_times % PRINT_TIME == 0 {
            info!("task_2_3 counted execute times:{}", count7_times);
        }
    }
}

#[embassy_executor::task]
async fn mock_task10() {
    // record it's execute time
    let mut count8_times: usize = 0;
    loop {
        count8_times += 1;
        Timer::after_millis(MID_SYSCALL_TIME).await;
        block_delay(MID_CALCULATE_TIME);
        Timer::after_millis(MID_SYSCALL_TIME).await;
        block_delay(MID_CALCULATE_TIME);
        if count8_times % PRINT_TIME == 0 {
            info!("task_2_4 counted execute times:{}", count8_times);
        }
    }
}

#[embassy_executor::task]
async fn mock_task11() {
    let mut count_times: usize = 0;
    loop {
        count_times += 1;
        Timer::after_millis(MID_SYSCALL_TIME).await;
        block_delay(LONG_CALCULATE_TIME);
        Timer::after_millis(MID_SYSCALL_TIME).await;
        block_delay(LONG_CALCULATE_TIME);
        if count_times % PRINT_TIME == 0 {
            info!("task_2_5 counted execute times:{}", count_times);
        }
    }
}


#[embassy_executor::task]
async fn mock_task12() {
    let mut count_times: usize = 0;
    loop {
        count_times += 1;
        Timer::after_millis(LONG_SYSCALL_TIME).await;
        block_delay(VERY_SHORT_CALCULATE_TIME);
        Timer::after_millis(LONG_SYSCALL_TIME).await;
        block_delay(VERY_SHORT_CALCULATE_TIME);
        if count_times % PRINT_TIME == 0 {
            info!("task_2_6 counted execute times:{}", count_times);
        }
    }
}


#[embassy_executor::task]
async fn mock_task13() {
    // record it's execute time
    let mut count5_times: usize = 0;
    loop {
        count5_times += 1;
        Timer::after_millis(MID_SYSCALL_TIME).await;
        block_delay(VERY_SHORT_CALCULATE_TIME);
        Timer::after_millis(SHORT_SYSCALL_TIME).await;
        block_delay(VERY_SHORT_CALCULATE_TIME);
        if count5_times % PRINT_TIME == 0 {
            info!("task_3_1 counted execute times:{}", count5_times);
        }
    }
}

#[embassy_executor::task]
async fn mock_task14() {
    // record it's execute time
    let mut count6_times: usize = 0;
    loop {
        count6_times += 1;
        Timer::after_millis(LONG_SYSCALL_TIME).await;
        block_delay(LONG_CALCULATE_TIME);
        Timer::after_millis(LONG_SYSCALL_TIME).await;
        block_delay(LONG_CALCULATE_TIME);
        if count6_times % PRINT_TIME == 0 {
            info!("task_3_2 counted execute times:{}", count6_times);
        }
    }
}

#[embassy_executor::task]
async fn mock_task15() {
    // record it's execute time
    let mut count7_times: usize = 0;
    loop {
        count7_times += 1;
        Timer::after_millis(SHORT_SYSCALL_TIME).await;
        block_delay(SHORT_CALCULATE_TIME);
        Timer::after_millis(SHORT_SYSCALL_TIME).await;
        block_delay(SHORT_CALCULATE_TIME);
        if count7_times % PRINT_TIME == 0 {
            info!("task_3_3 counted execute times:{}", count7_times);
        }
    }
}

#[embassy_executor::task]
async fn mock_task16() {
    // record it's execute time
    let mut count8_times: usize = 0;
    loop {
        count8_times += 1;
        Timer::after_millis(MID_SYSCALL_TIME).await;
        block_delay(MID_CALCULATE_TIME);
        Timer::after_millis(MID_SYSCALL_TIME).await;
        block_delay(MID_CALCULATE_TIME);
        if count8_times % PRINT_TIME == 0 {
            info!("task_3_4 counted execute times:{}", count8_times);
        }
    }
}

#[embassy_executor::task]
async fn mock_task17() {
    let mut count_times: usize = 0;
    loop {
        count_times += 1;
        Timer::after_millis(MID_SYSCALL_TIME).await;
        block_delay(LONG_CALCULATE_TIME);
        Timer::after_millis(MID_SYSCALL_TIME).await;
        block_delay(LONG_CALCULATE_TIME);
        if count_times % PRINT_TIME == 0 {
            info!("task_3_5 counted execute times:{}", count_times);
        }
    }
}


#[embassy_executor::task]
async fn mock_task18() {
    let mut count_times: usize = 0;
    loop {
        count_times += 1;
        Timer::after_millis(LONG_SYSCALL_TIME).await;
        block_delay(VERY_SHORT_CALCULATE_TIME);
        Timer::after_millis(LONG_SYSCALL_TIME).await;
        block_delay(VERY_SHORT_CALCULATE_TIME);
        if count_times % PRINT_TIME == 0 {
            info!("task_3_6 counted execute times:{}", count_times);
        }
    }
}

#[allow(unused)]
fn block_delay(tick: u64) {
    let mut i = 0;
    let mut j = 0;
    while i < tick {
        i += 1;
        while j < tick * tick {
            j += 1;
        }
    }
}
