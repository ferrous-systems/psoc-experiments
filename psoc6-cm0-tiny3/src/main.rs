#![no_std]
#![no_main]

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

#[entry]
fn main() -> ! {
    for i in 0..20 {
        hprintln!("{:03}: Hello, I am the Cortex-M0+!", i);
        for _ in 0..1_000_000 {
            unsafe {
                core::arch::asm!("nop");
            }
        }
    }
    panic!("Ran out of loops!");
}

#[panic_handler]
fn panic(panic_info: &core::panic::PanicInfo<'_>) -> ! {
    hprintln!("Panic: {:#?}", panic_info);
    loop {
        unsafe {
            core::arch::asm!("bkpt");
        }
    }
}
