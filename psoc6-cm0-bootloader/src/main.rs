//! A basic bootloader for the Cortex-M0+
//!
//! Looks at the image in APP_IMAGE and gets the Cortex-M4 to boot it.
//!
//! Currently broken because the CM4 ends up in the CM0's hardfault handler :/
//!
//! When debugging, this program has the wrong stack pointer, so it messes up
//! the CM4's stack.

#![no_std]
#![no_main]

use cortex_m_rt::entry;
// use cortex_m_semihosting::hprintln;
use panic_halt as _;

/// Build this with
///
/// ```console
/// pushd ../psoc6-cm4-rtic
/// cargo build --release --features=use-bootloader
/// popd
/// arm-none-eabi-objcopy -O binary ../psoc6-cm4-rtic/target/thumbv7em-none-eabihf/release/psoc6-cm4-rtic ./src/app.bin
/// ```
#[link_section = ".app_flash"]
#[used]
pub static APP_IMAGE: [u8; include_bytes!("app.bin").len()] = *include_bytes!("app.bin");

/// We seem to end up here with the wrong stack pointer, so bounce us into a
/// main function with a correct stack pointer.
#[entry]
fn main() -> ! {
    let bl_stack_ptr_addr = 0x1000_0000 as *const u32;
    let bl_stack_ptr = unsafe { bl_stack_ptr_addr.read_volatile() } as *const u32;
    let rv = real_main as *const u32;

    unsafe { cortex_m::asm::bootstrap(bl_stack_ptr, rv) }
}

fn real_main() -> ! {
    let p = unsafe { psoc6_pac::Peripherals::steal() };
    let cp = unsafe { cortex_m::Peripherals::steal() };

    let app_stack_ptr_addr = 0x1000_8000 as *const u32;
    let app_reset_fn_addr = 0x1000_8004 as *const u32;
    let app_stack_ptr = unsafe { app_stack_ptr_addr.read_volatile() };
    let app_reset_fn = unsafe { app_reset_fn_addr.read_volatile() };

    // hprintln!("app_stack_ptr @ {:p} = {:p}", app_stack_ptr_addr, app_stack_ptr as *const u32);
    // hprintln!("app_reset_fn @ {:p} = {:p}", app_reset_fn_addr, app_reset_fn as *const u32);

    let bl_stack_ptr_addr = 0x1000_0000 as *const u32;
    let bl_reset_fn_addr = 0x1000_0004 as *const u32;
    let bl_stack_ptr = unsafe { bl_stack_ptr_addr.read_volatile() };
    let bl_reset_fn = unsafe { bl_reset_fn_addr.read_volatile() };
    // hprintln!("bl_stack_ptr @ {:p} = {:p}", bl_stack_ptr_addr, bl_stack_ptr as *const u32);
    // hprintln!("bl_reset_fn @ {:p} = {:p}", bl_reset_fn_addr, bl_reset_fn as *const u32);

    // 50 MHz seems to be our default speed
    let mut delay = cortex_m::delay::Delay::new(cp.SYST, 50_000_000);

    // "User LED" LED4 is on P13_7 (Port 13, Pin 7).
    // Set as output with strong drive and no input buffer.
    p.GPIO.prt13.cfg.write(|w| {
        w.in_en7().clear_bit();
        w.drive_mode7()
            .variant(psoc6_pac::gpio::prt::cfg::DRIVE_MODE0_A::STRONG.into());
        w
    });

    for _ in 0..10 {
        p.GPIO.prt13.out_clr.write(|w| {
            w.out7().set_bit();
            w
        });
        delay.delay_ms(100);
        p.GPIO.prt13.out_set.write(|w| {
            w.out7().set_bit();
            w
        });
        delay.delay_ms(100);
    }

    let cpu_state = p.CPUSS.cm4_status.read().bits() & 0x03;
    if cpu_state == 3 {
        // CPU is already running
        p.CPUSS.cm4_pwr_ctl.write(|w| {
            // 0 is disabled, 1 is reset, 2 is retained and 3 is enabled
            // plus 0x05fa0000 as the security key
            unsafe { w.bits(0x05fa0001) };
            w
        });
    }

    // hprintln!("power status: {:?}", p.CPUSS.cm4_status.read().bits());
    while p.CPUSS.cm4_status.read().pwr_done().bit_is_clear() {
        // loop until power state changed
        // hprintln!("power status: {:?}", p.CPUSS.cm4_status.read().bits());
    }

    // Program VTOR via this special register that should set it in the CM4's
    // private register for us.
    p.CPUSS.cm4_vector_table_base.write(|w| {
        unsafe { w.bits(app_stack_ptr_addr as u32) };
        w
    });

    // Take CM4 out of reset
    p.CPUSS.cm4_pwr_ctl.write(|w| {
        // 0 is disabled, 1 is reset, 2 is retained and 3 is enabled
        // plus 0x05fa0000 as the security key
        unsafe { w.bits(0x05fa0003) };
        w
    });

    // hprintln!("power status 2: {:?}", p.CPUSS.cm4_status.read().bits());
    while p.CPUSS.cm4_status.read().pwr_done().bit_is_clear() {
        // loop until power state changed
        // hprintln!("power status 2: {:?}", p.CPUSS.cm4_status.read().bits());
    }

    loop {
        cortex_m::asm::wfe();
    }
}
