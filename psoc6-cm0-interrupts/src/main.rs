#![no_std]
#![no_main]

use cortex_m_rt::{entry, exception};
use cortex_m_semihosting::hprintln;
use panic_semihosting as _;

#[entry]
fn main() -> ! {
    hprintln!("main 1");
    cortex_m::interrupt::disable();
    let mut cp = unsafe { cortex_m::peripheral::Peripherals::steal() };
    unsafe {
        core::arch::asm!("svc 0");
    }
    hprintln!("main 2");
    loop {}
}

#[exception]
fn SVCall() {
    hprintln!("SVCall: hi");
}
