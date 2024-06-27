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
const LIMIT_TIME:usize = 25;

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
        unwrap!(spawner.spawn(blink1_1()));
        unwrap!(spawner.spawn(blink1_2()));
        unwrap!(spawner.spawn(blink1_3()));
        unwrap!(spawner.spawn(blink1_4()));
        unwrap!(spawner.spawn(blink1_5()));
        unwrap!(spawner.spawn(blink1_6()));
        unwrap!(spawner.spawn(blink1_7()));
        unwrap!(spawner.spawn(blink1_8()));
        unwrap!(spawner.spawn(blink1_9()));
        unwrap!(spawner.spawn(blink1_10()));
        unwrap!(spawner.spawn(dead_task1()));
    });
}

fn task_2() {
    let executor = EXECUTOR_LOW2.init(Executor::new());
    executor.run(|spawner| {
        unwrap!(spawner.spawn(blink2_1()));
        unwrap!(spawner.spawn(blink2_2()));
        unwrap!(spawner.spawn(blink2_3()));
        unwrap!(spawner.spawn(blink2_4()));
        unwrap!(spawner.spawn(blink2_5()));
        unwrap!(spawner.spawn(blink2_6()));
        unwrap!(spawner.spawn(blink2_7()));
        unwrap!(spawner.spawn(blink2_8()));
        unwrap!(spawner.spawn(blink2_9()));
        unwrap!(spawner.spawn(blink2_10()));
        unwrap!(spawner.spawn(dead_task2()));
    });
}

#[embassy_executor::task]
async fn dead_task1() {
    loop{}
}

#[embassy_executor::task]
async fn dead_task2() {
    loop{}
}

#[embassy_executor::task]
async fn blink1_1() {
    let mut count1_times:usize = 0;
    let start = Instant::now();
    loop {
        count1_times += 1;
        info!("high1");
        // led.set_high();
        Timer::after_millis(100).await;
        // block_delay(1000);

        info!("low1");
        // led.set_low();
        Timer::after_millis(100).await;
        // block_delay(1000);
        if count1_times >= LIMIT_TIME {
            break;
        }
    }
    let end = Instant::now();
    let ms = end.duration_since(start).as_millis();
    info!("task_1_1 end with times:{}", count1_times);
    info!("task_1_1 execute time:{}ms", ms);
}

#[embassy_executor::task]
async fn blink1_2() {
    let mut count2_times:usize = 0;
    let start = Instant::now();
    loop {
        count2_times += 1;
        info!("high2");
        // led.set_high();
        Timer::after_millis(200).await;
        // block_delay(100);

        info!("low2");
        // led.set_low();
        // block_delay(100);
        Timer::after_millis(200).await;
        if count2_times >= LIMIT_TIME{
            break;
        }
    }
    let end = Instant::now();
    let ms = end.duration_since(start).as_millis();
    info!("task_1_2 end with times:{}", count2_times);
    info!("task_1_2 execute time:{}ms", ms);
}

#[embassy_executor::task]
async fn blink1_3() {
    // record it's execute time
    let mut count3_times:usize = 0;
    let start = Instant::now();
    loop {
        count3_times += 1;
        info!("high3");
        // led.set_high();
        Timer::after_millis(300).await;
        // block_delay(1000);
        info!("low3");
        // led.set_low();
        Timer::after_millis(300).await;
        // block_delay(1000);
        if count3_times >= LIMIT_TIME {
            break;
        }
    }
    let end = Instant::now();
    let ms = end.duration_since(start).as_millis();
    info!("task_1_3 end with times:{}", count3_times);
    info!("task_1_3 execute time:{}ms", ms);
}

#[embassy_executor::task]
async fn blink1_4() {
    let mut count4_times:usize = 0;
    let start = Instant::now();
    loop {
        count4_times += 1;
        info!("high4");
        // led.set_high();
        Timer::after_millis(400).await;
        // block_delay(100);

        info!("low4");
        // led.set_low();
        // block_delay(100);
        Timer::after_millis(400).await;
        if count4_times >= LIMIT_TIME{
            break;
        }
    }
    let end = Instant::now();
    let ms = end.duration_since(start).as_millis();
    info!("task_1_4 end with times:{}", count4_times);
    info!("task_1_4 execute time:{}ms", ms);
}

#[embassy_executor::task]
async fn blink1_5() {
    // record it's execute time
    let mut count5_times:usize = 0;
    let start = Instant::now();
    loop {
        count5_times += 1;
        info!("high5");
        // led.set_high();
        Timer::after_millis(500).await;
        // block_delay(1000);
        info!("low5");
        // led.set_low();
        Timer::after_millis(500).await;
        // block_delay(1000);
        if count5_times >= LIMIT_TIME {
            break;
        }
    }
    let end = Instant::now();
    let ms = end.duration_since(start).as_millis();
    info!("task_1_5 end with times:{}", count5_times);
    info!("task_1_5 execute time:{}ms", ms);
}

#[embassy_executor::task]
async fn blink1_6() {
    // record it's execute time
    let mut count6_times:usize = 0;
    let start = Instant::now();
    loop {
        count6_times += 1;
        info!("high6");
        // led.set_high();
        Timer::after_millis(600).await;
        // block_delay(1000);
        info!("low6");
        // led.set_low();
        Timer::after_millis(600).await;
        // block_delay(1000);
        if count6_times >= LIMIT_TIME {
            break;
        }
    }
    let end = Instant::now();
    let ms = end.duration_since(start).as_millis();
    info!("task_1_6 end with times:{}", count6_times);
    info!("task_1_6 execute time:{}ms", ms);
}


#[embassy_executor::task]
async fn blink1_7() {
    // record it's execute time
    let mut count7_times:usize = 0;
    let start = Instant::now();
    loop {
        count7_times += 1;
        info!("high7");
        // led.set_high();
        Timer::after_millis(700).await;
        // block_delay(1000);
        info!("low7");
        // led.set_low();
        Timer::after_millis(700).await;
        // block_delay(1000);
        if count7_times >= LIMIT_TIME {
            break;
        }
    }
    let end = Instant::now();
    let ms = end.duration_since(start).as_millis();
    info!("task_1_7 end with times:{}", count7_times);
    info!("task_1_7 execute time:{}ms", ms);
}

#[embassy_executor::task]
async fn blink1_8() {
    // record it's execute time
    let mut count8_times:usize = 0;
    let start = Instant::now();
    loop {
        count8_times += 1;
        info!("high8");
        // led.set_high();
        Timer::after_millis(800).await;
        // block_delay(1000);
        info!("low8");
        // led.set_low();
        Timer::after_millis(800).await;
        // block_delay(1000);
        if count8_times >= LIMIT_TIME {
            break;
        }
    }
    let end = Instant::now();
    let ms = end.duration_since(start).as_millis();
    info!("task_1_8 end with times:{}", count8_times);
    info!("task_1_8 execute time:{}ms", ms);
}

#[embassy_executor::task]
async fn blink1_9() {
    // record it's execute time
    let mut count9_times:usize = 0;
    let start = Instant::now();
    loop {
        count9_times += 1;
        info!("high8");
        // led.set_high();
        Timer::after_millis(900).await;
        // block_delay(1000);
        info!("low8");
        // led.set_low();
        Timer::after_millis(900).await;
        // block_delay(1000);
        if count9_times >= LIMIT_TIME {
            break;
        }
    }
    let end = Instant::now();
    let ms = end.duration_since(start).as_millis();
    info!("task_1_9 end with times:{}", count9_times);
    info!("task_1_9 execute time:{}ms", ms);
}

#[embassy_executor::task]
async fn blink1_10() {
    // record it's execute time
    let mut count10_times:usize = 0;
    let start = Instant::now();
    loop {
        count10_times += 1;
        info!("high8");
        // led.set_high();
        Timer::after_millis(1000).await;
        // block_delay(1000);
        info!("low8");
        // led.set_low();
        Timer::after_millis(1000).await;
        // block_delay(1000);
        if count10_times >= LIMIT_TIME {
            break;
        }
    }
    let end = Instant::now();
    let ms = end.duration_since(start).as_millis();
    info!("task_1_10 end with times:{}", count10_times);
    info!("task_1_10 execute time:{}ms", ms);
}

#[embassy_executor::task]
async fn blink2_1() {
    // record it's execute time
    let mut count11_times:usize = 0;
    let start = Instant::now();
    loop {
        count11_times += 1;
        info!("high8");
        // led.set_high();
        Timer::after_millis(100).await;
        // block_delay(1000);
        info!("low8");
        // led.set_low();
        Timer::after_millis(100).await;
        // block_delay(1000);
        if count11_times >= LIMIT_TIME {
            break;
        }
    }
    let end = Instant::now();
    let ms = end.duration_since(start).as_millis();
    info!("task_2_1 end with times:{}", count11_times);
    info!("task_2_1 execute time:{}ms", ms);
}

#[embassy_executor::task]
async fn blink2_2() {
    // record it's execute time
    let mut count12_times:usize = 0;
    let start = Instant::now();
    loop {
        count12_times += 1;
        info!("high8");
        // led.set_high();
        Timer::after_millis(200).await;
        // block_delay(1000);
        info!("low8");
        // led.set_low();
        Timer::after_millis(200).await;
        // block_delay(1000);
        if count12_times >= LIMIT_TIME {
            break;
        }
    }
    let end = Instant::now();
    let ms = end.duration_since(start).as_millis();
    info!("task_2_2 end with times:{}", count12_times);
    info!("task_2_2 execute time:{}ms", ms);
}

#[embassy_executor::task]
async fn blink2_3() {
    // record it's execute time
    let mut count13_times:usize = 0;
    let start = Instant::now();
    loop {
        count13_times += 1;
        info!("high8");
        // led.set_high();
        Timer::after_millis(300).await;
        // block_delay(1000);
        info!("low8");
        // led.set_low();
        Timer::after_millis(300).await;
        // block_delay(1000);
        if count13_times >= LIMIT_TIME {
            break;
        }
    }
    let end = Instant::now();
    let ms = end.duration_since(start).as_millis();
    info!("task_2_3 end with times:{}", count13_times);
    info!("task_2_3 execute time:{}ms", ms);
}

#[embassy_executor::task]
async fn blink2_4() {
    // record it's execute time
    let mut count14_times:usize = 0;
    let start = Instant::now();
    loop {
        count14_times += 1;
        info!("high8");
        // led.set_high();
        Timer::after_millis(400).await;
        // block_delay(1000);
        info!("low8");
        // led.set_low();
        Timer::after_millis(400).await;
        // block_delay(1000);
        if count14_times >= LIMIT_TIME {
            break;
        }
    }
    let end = Instant::now();
    let ms = end.duration_since(start).as_millis();
    info!("task_2_4 end with times:{}", count14_times);
    info!("task_2_4 execute time:{}ms", ms);
}

#[embassy_executor::task]
async fn blink2_5() {
    // record it's execute time
    let mut count15_times:usize = 0;
    let start = Instant::now();
    loop {
        count15_times += 1;
        info!("high8");
        // led.set_high();
        Timer::after_millis(500).await;
        // block_delay(1000);
        info!("low8");
        // led.set_low();
        Timer::after_millis(500).await;
        // block_delay(1000);
        if count15_times >= LIMIT_TIME {
            break;
        }
    }
    let end = Instant::now();
    let ms = end.duration_since(start).as_millis();
    info!("task_2_5 end with times:{}", count15_times);
    info!("task_2_5 execute time:{}ms", ms);
}

#[embassy_executor::task]
async fn blink2_6() {
    // record it's execute time
    let mut count16_times:usize = 0;
    let start = Instant::now();
    loop {
        count16_times += 1;
        info!("high8");
        // led.set_high();
        Timer::after_millis(600).await;
        // block_delay(1000);
        info!("low8");
        // led.set_low();
        Timer::after_millis(600).await;
        // block_delay(1000);
        if count16_times >= LIMIT_TIME {
            break;
        }
    }
    let end = Instant::now();
    let ms = end.duration_since(start).as_millis();
    info!("task_2_6 end with times:{}", count16_times);
    info!("task_2_6 execute time:{}ms", ms);
}

#[embassy_executor::task]
async fn blink2_7() {
    // record it's execute time
    let mut count17_times:usize = 0;
    let start = Instant::now();
    loop {
        count17_times += 1;
        info!("high8");
        // led.set_high();
        Timer::after_millis(700).await;
        // block_delay(1000);
        info!("low8");
        // led.set_low();
        Timer::after_millis(700).await;
        // block_delay(1000);
        if count17_times >= LIMIT_TIME {
            break;
        }
    }
    let end = Instant::now();
    let ms = end.duration_since(start).as_millis();
    info!("task_2_7 end with times:{}", count17_times);
    info!("task_2_7 execute time:{}ms", ms);
}

#[embassy_executor::task]
async fn blink2_8() {
    // record it's execute time
    let mut count18_times:usize = 0;
    let start = Instant::now();
    loop {
        count18_times += 1;
        info!("high8");
        // led.set_high();
        Timer::after_millis(800).await;
        // block_delay(1000);
        info!("low8");
        // led.set_low();
        Timer::after_millis(800).await;
        // block_delay(1000);
        if count18_times >= LIMIT_TIME {
            break;
        }
    }
    let end = Instant::now();
    let ms = end.duration_since(start).as_millis();
    info!("task_2_8 end with times:{}", count18_times);
    info!("task_2_8 execute time:{}ms", ms);
}

#[embassy_executor::task]
async fn blink2_9() {
    // record it's execute time
    let mut count19_times:usize = 0;
    let start = Instant::now();
    loop {
        count19_times += 1;
        info!("high8");
        // led.set_high();
        Timer::after_millis(900).await;
        // block_delay(1000);
        info!("low8");
        // led.set_low();
        Timer::after_millis(900).await;
        // block_delay(1000);
        if count19_times >= LIMIT_TIME {
            break;
        }
    }
    let end = Instant::now();
    let ms = end.duration_since(start).as_millis();
    info!("task_2_9 end with times:{}", count19_times);
    info!("task_2_9 execute time:{}ms", ms);
}

#[embassy_executor::task]
async fn blink2_10() {
    // record it's execute time
    let mut count20_times:usize = 0;
    let start = Instant::now();
    loop {
        count20_times += 1;
        info!("high8");
        // led.set_high();
        Timer::after_millis(1000).await;
        // block_delay(1000);
        info!("low8");
        // led.set_low();
        Timer::after_millis(1000).await;
        // block_delay(1000);
        if count20_times >= LIMIT_TIME {
            break;
        }
    }
    let end = Instant::now();
    let ms = end.duration_since(start).as_millis();
    info!("task_2_10 end with times:{}", count20_times);
    info!("task_2_10 execute time:{}ms", ms);
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