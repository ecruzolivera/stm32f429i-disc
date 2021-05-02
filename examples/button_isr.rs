//! # Button handling using ISR
//!
//! This example handles the stm32f429-disco board user button using an ISR and a flag
//! The leds are toggle using a global flag for no particular reason.

#![no_main]
#![no_std]

use core::{
    cell::RefCell,
    sync::atomic::{AtomicBool, Ordering},
};

use cortex_m::interrupt as cm_interrupt;
use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use stm32f429i_disc as board;

use board::led::{Color, Leds};
use board::{gpio::Edge, hal::prelude::*};
use board::{
    gpio::{Input, PullDown},
    hal::stm32,
    hal::stm32::interrupt,
};

static BUTTON: Mutex<RefCell<Option<stm32f4xx_hal::gpio::gpioa::PA0<Input<PullDown>>>>> =
    Mutex::new(RefCell::new(None));

static FLAG: AtomicBool = AtomicBool::new(false);

#[entry]
fn main() -> ! {
    if let Some(p) = stm32::Peripherals::take() {
        // Constrain clock registers
        let rcc = p.RCC.constrain();
        // Configure clock to 180 MHz (i.e. the maximum) and freeze it
        let _ = rcc.cfgr.sysclk(180.mhz()).freeze();
        // PA0 is user button
        let gpioa = p.GPIOA.split();
        // leds are in GPIOG
        let gpiog = p.GPIOG.split();
        // Initialize on-board LEDs
        let mut leds = Leds::new(gpiog);
        let mut syscfg = p.SYSCFG.constrain();
        let mut exti = p.EXTI;
        let mut button = gpioa.pa0.into_pull_down_input();

        button.make_interrupt_source(&mut syscfg);
        button.trigger_on_edge(&mut exti, Edge::RISING);
        button.enable_interrupt(&mut exti);
        cm_interrupt::free(|cs| BUTTON.borrow(cs).replace(Some(button)));
        unsafe {
            stm32::NVIC::unmask(stm32::Interrupt::EXTI0);
        }

        leds[Color::Red].off();
        leds[Color::Green].on();
        loop {
            if FLAG.load(Ordering::Relaxed) {
                leds[Color::Red].toggle();
                leds[Color::Green].toggle();
                FLAG.store(false, Ordering::Relaxed);
                defmt::info!("Toggling leds");
            }
        }
    }
    loop {
        defmt::panic!("Error taking the peripherals");
    }
}

#[interrupt]
fn EXTI0() {
    defmt::info!("Irq Triggered");
    FLAG.store(true, Ordering::Relaxed);
    cm_interrupt::free(|cs| {
        if let Some(ref mut button) = *BUTTON.borrow(cs).borrow_mut() {
            button.clear_interrupt_pending_bit();
        }
    })
}
