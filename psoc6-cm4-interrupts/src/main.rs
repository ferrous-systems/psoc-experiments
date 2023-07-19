//! Shows how the svc instruction can call the SvCall exception.

#![no_std]
#![no_main]

use cortex_m_rt::{entry, exception};
use cortex_m_semihosting::hprintln;
use panic_semihosting as _;

#[entry]
fn main() -> ! {
    let mut cp = unsafe { cortex_m::peripheral::Peripherals::steal() };

    unsafe {
        cortex_m::interrupt::enable();
    }

    hprintln!("CPUID.base = 0x{:08x}", cp.CPUID.base.read());
    hprintln!("VTOR = 0x{:08x}", cp.SCB.vtor.read());
    unsafe { cp.SCB.vtor.write(0x1000_0000) };
    hprintln!("VTOR is now = 0x{:08x}", cp.SCB.vtor.read());

    unsafe {
        core::arch::asm!("svc 0");
    }
    hprintln!("main 2");
    loop {
        cortex_m::asm::wfi();
    }
}

#[exception]
fn SVCall() {
    hprintln!("SVCall: hi");
}

#[exception]
unsafe fn HardFault(frame: &cortex_m_rt::ExceptionFrame) -> ! {
    hprintln!("HardFault: {:#?}", frame);
    loop {
        cortex_m::asm::bkpt();
    }
}
