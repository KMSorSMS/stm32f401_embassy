#![no_std]
#![no_main]
// #![feature(impl_trait_in_assoc_type)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::{
    gpio::{Level, Output, Speed},
    rcc::Pll,
};
use embassy_time::Timer;
use stm32_metapac::rcc::vals;
use {defmt_rtt as _, panic_probe as _};
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // embassy_stm32::rcc::Config {
    //     hsi: false,
    //     hse,
    //     sys: vals::Sw::PLL1_P, // use pll as sys
    //     pll_src: vals::Pllsrc::HSE,
    //     //the division number is:PLLM=4, PLLN=84, PLLP=2, PLLQ=4
    //     pll,
    //     plli2s: None,
    //     // apb1_div=2 apb2_div=1
    //     ahb_pre: vals::Hpre::DIV1,
    //     apb1_pre: vals::Ppre::DIV2,
    //     apb2_pre: vals::Ppre::DIV2,
    //     // // low speed clock just use the default
    //     // ls: Default::default(),
    //     // // let the mux choose the pllclk
    //     // mux: Default::default(),
    //     ..Default::default()
    // };

    // embassy_stm32::Config {
    //     rcc,
    //     enable_debug_during_sleep:true,
    //     dma_interrupt_priority: Priority::P0,
    //     ..Default::default()
    // };
    // try to set my peripheral init

    // let p = embassy_stm32::init(Default::default());
    // we use 84Mhz sys from 8Mhz HSE with
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

    // let config = embassy_stm32::Config::default();

    let p = embassy_stm32::init(config);
    info!("Hello World!");

    let mut led = Output::new(p.PA5, Level::High, Speed::Low);

    loop {
        info!("high");
        led.set_high();
        Timer::after_millis(300).await;

        info!("low");
        led.set_low();
        Timer::after_millis(300).await;
    }
}
