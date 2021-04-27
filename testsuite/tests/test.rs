#![no_std]
#![no_main]

use stm32f429i_disc as _; // memory layout + panic handler

// See https://crates.io/crates/defmt-test/0.1.0 for more documentation (e.g. about the 'state'
// feature)
#[defmt_test::tests]
mod tests {
    use defmt::{assert, assert_eq};

    #[test]
    fn assert_true() {
        assert!(true)
    }

    #[test]
    fn assert_eq() {
        assert_eq!(42, 42, "TODO: write actual tests")
    }
}
