//! # Button handling using ISR
//!
//! This example handles the stm32f429-disco board user button using an ISR and a flag
//! The leds are toggle using a global flag for no particular reason.

#![no_main]
#![no_std]

use cortex_m_rt::entry;
use stm32f429i_disc as board;

use board::{
    hal::{prelude::*, stm32},
    led::{Color, Leds},
    spi::{Mode, Phase, Polarity, Spi},
    time,
};

#[entry]
fn main() -> ! {
    if let Some(p) = stm32::Peripherals::take() {
        // Constrain clock registers
        let rcc = p.RCC.constrain();
        // Configure clock to 180 MHz (i.e. the maximum) and freeze it
        let clocks = rcc.cfgr.sysclk(180.mhz()).freeze();
        // spi gpio
        let gpiof = p.GPIOF.split();
        // leds are in GPIOG
        let gpiog = p.GPIOG.split();
        // Initialize on-board LEDs
        let mut leds = Leds::new(gpiog);

        let sck = gpiof.pf7.into_alternate_af5();
        let miso = gpiof.pf8.into_alternate_af5();
        let mosi = gpiof.pf9.into_alternate_af5();
        let mut spi = Spi::spi5(
            p.SPI5,
            (sck, miso, mosi),
            Mode {
                polarity: Polarity::IdleLow,
                phase: Phase::CaptureOnFirstTransition,
            },
            time::KiloHertz(2000).into(),
            clocks,
        );
        let mut mssg = [1, 2, 3, 4, 5];
        // let mut mssg = *b"Heyyy!";
        defmt::info!("Sending: {:?}", mssg);
        match spi.transfer(&mut mssg) {
            Ok(rx) => {
                leds[Color::Green].on();
                defmt::info!("receiving: {:?}", rx);
            }
            Err(_) => {
                leds[Color::Red].on();
                defmt::error!("error tranfering spi");
            }
        }

        loop {}
    }
    loop {
        defmt::panic!("Error taking the peripherals");
    }
}
