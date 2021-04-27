#![no_std]

use core::sync::atomic::{AtomicUsize, Ordering};

pub mod led;
pub use cortex_m_rt::*;
pub use cortex_m::*;
pub use crate::hal::*;
pub use crate::hal::stm32::*;
pub use crate::hal::stm32::interrupt::*;
pub use defmt_rtt as _; // global logger
pub use panic_probe as _;
pub use stm32f4xx_hal as hal; // memory layout

// same panicking *behavior* as `panic-probe` but doesn't print a panic message
// this prevents the panic message being printed *twice* when `defmt::panic` is invoked
#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}

static COUNT: AtomicUsize = AtomicUsize::new(0);
defmt::timestamp!("{=usize}", {
    // NOTE(no-CAS) `timestamps` runs with interrupts disabled
    let n = COUNT.load(Ordering::Relaxed);
    COUNT.store(n + 1, Ordering::Relaxed);
    n
});

/// Terminates the application and makes `probe-run` exit with exit-code = 0
pub fn exit() -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}
