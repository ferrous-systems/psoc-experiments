#![no_main]
#![no_std]

use core::{arch::asm, panic::PanicInfo, sync::atomic};

// The reset vector, a pointer into the reset handler
#[link_section = ".vector_table.reset_vector"]
#[no_mangle]
pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = Reset;

static COUNT: atomic::AtomicU32 = atomic::AtomicU32::new(10000);

#[no_mangle]
pub unsafe extern "C" fn Reset() -> ! {
    extern "C" {
        static mut _sbss: u8;
        static mut _ebss: u8;

        static mut _sdata: u8;
        static mut _edata: u8;
        static _sidata: u8;
    }

    let count = &_ebss as *const u8 as usize - &_sbss as *const u8 as usize;
    core::ptr::write_bytes(&mut _sbss as *mut u8, 0, count);

    let count = &_edata as *const u8 as usize - &_sdata as *const u8 as usize;
    core::ptr::copy_nonoverlapping(&_sidata as *const u8, &mut _sdata as *mut u8, count);

    main();
    loop {
        asm!("bkpt");
    }
}

fn main() {
    let x = COUNT.load(atomic::Ordering::Relaxed);

    panic!("Oh no x = {}!", x);
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}
