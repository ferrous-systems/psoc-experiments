#![no_std]
#![no_main]

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use panic_semihosting as _;

#[entry]
fn main() -> ! {
    hprintln!("Hello, world!");

    let p = unsafe { psoc6_pac::Peripherals::steal() };
    let cp = unsafe { cortex_m::Peripherals::steal() };

    // Can we read Cortex-M registers
    hprintln!("CPUID.base = 0x{:08x}", cp.CPUID.base.read());

    // Can we read PSoC6 registers?
    hprintln!(
        "CPUSS.cm4_clock_ctl = 0x{:08x}",
        p.CPUSS.cm4_clock_ctl.read().bits()
    );

    panic!("End of main()");
}
