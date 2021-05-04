//! # SPI Polling example
//!
//! This example uses the SPI3 in blocking mode
//! The leds are toggle using a global flag for no particular reason.

#![no_main]
#![no_std]

use core::cmp;

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
        let gpiob = p.GPIOB.split();
        // leds are in GPIOG
        let gpiog = p.GPIOG.split();
        // Initialize on-board LEDs
        let mut leds = Leds::new(gpiog);

        let sck = gpiob.pb3.into_alternate_af6();
        let miso = gpiob.pb4.into_alternate_af6();
        let mosi = gpiob.pb5.into_alternate_af6();
        let mut spi = Spi::spi3(
            p.SPI3,
            (sck, miso, mosi),
            Mode {
                polarity: Polarity::IdleLow,
                phase: Phase::CaptureOnFirstTransition,
            },
            time::KiloHertz(2000).into(),
            clocks,
        );
        const SIZE: usize = 5;
        const ORIGINAL: [u8; SIZE] = [1u8, 2, 3, 4, 5];
        let mut mssg = [0; SIZE];
        // need to copy because the SPI driver stores the readed values in the same buffer (ie. mssg)
        mssg.copy_from_slice(&ORIGINAL);

        defmt::info!("Sending: {:?}", mssg);

        match spi.transfer(&mut mssg) {
            Ok(rx) => {
                leds[Color::Green].on();
                // rx is a reference to mssg
                // defmt::info!("Reading: {:?}", mssg);
                defmt::info!("Reading: {:?}", rx);
                // comparing each element and the length of the arrays if all elements are equal
                let same = mssg
                    .iter()
                    .zip(&ORIGINAL)
                    .map(|(x, y)| x.cmp(y))
                    .find(|&ord| ord != cmp::Ordering::Equal)
                    .unwrap_or(mssg.len().cmp(&ORIGINAL.len()));
                // if PB4 and PB5 are connected (loopback) then mssg == ORIGINAL
                if same == cmp::Ordering::Equal {
                    defmt::info!("Are equal!");
                } else {
                    defmt::warn!("Are not equal!");
                }
            }
            Err(_) => {
                leds[Color::Red].on();
                defmt::error!("Error tranfering spi");
            }
        }
    } else {
        defmt::error!("Error taking the peripherals");
    }
    loop {}
}
