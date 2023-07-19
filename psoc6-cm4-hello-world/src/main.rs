#![no_std]
#![no_main]

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use panic_semihosting as _;

#[entry]
fn main() -> ! {
    hprintln!("Hello, I am the Cortex-M4!");

    let _p = unsafe { psoc6_pac::Peripherals::steal() };
    let cp = unsafe { cortex_m::Peripherals::steal() };

    // Can we read Cortex-M registers
    // The Cortex-M0+ will report 0x41_0_cc60_1 (ARM, Rev 0, ARMv6-M Cortex-M0+, Patch 1).
    // The Cortex-M4F will report 0x41_0_fc24_1 (ARM, Rev 0, Cortex-M4, Patch 1).
    hprintln!("CPUID.base = 0x{:08x}", cp.CPUID.base.read());

    // 50 MHz seems to be our default speed
    let mut delay = cortex_m::delay::Delay::new(cp.SYST, 50_000_000);
    loop {
        // Set GPIO pin high
        hprintln!("Hello!");
        delay.delay_ms(1000);
    }
}
