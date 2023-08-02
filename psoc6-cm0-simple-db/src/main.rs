#![no_std]
#![no_main]

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use panic_semihosting as _;

#[entry]
fn main() -> ! {
    hprintln!("Hello, I am the Cortex-M0+!");

    let cp = cortex_m::Peripherals::take().unwrap();

    // Print the CPUID register just to remind us which CPU we are on.
    // The Cortex-M0+ will report 0x41_0_cc60_1 (ARM, Rev 0, ARMv6-M Cortex-M0+, Patch 1).
    // The Cortex-M4F will report 0x41_0_fc24_1 (ARM, Rev 0, Cortex-M4, Patch 1).
    hprintln!("CPUID.base = 0x{:08x}", cp.CPUID.base.read());

    let test_cases = [
        "PUBLISH hello, world\n",
        "PUBLISH\n",
        "RETRIEVE\n",
        "FETCH\n",
    ];

    for input in test_cases {
        let result = simple_db::parse(input);
        hprintln!("Input {:?} => {:?}", input, result);
    }

    panic!("Finished");
}
