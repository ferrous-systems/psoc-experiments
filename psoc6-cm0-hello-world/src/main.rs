#![no_std]
#![no_main]

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use panic_semihosting as _;

#[entry]
fn main() -> ! {
    hprintln!("Hello, I am the Cortex-M0+!");

    let p = unsafe { psoc6_pac::Peripherals::steal() };
    let cp = unsafe { cortex_m::Peripherals::steal() };

    // Print the CPUID register just to remind us which CPU we are on.
    // The Cortex-M0+ will report 0x41_0_cc60_1 (ARM, Rev 0, ARMv6-M Cortex-M0+, Patch 1).
    // The Cortex-M4F will report 0x41_0_fc24_1 (ARM, Rev 0, Cortex-M4, Patch 1).
    hprintln!("CPUID.base = 0x{:08x}", cp.CPUID.base.read());

    // Let's try some GPIO. There are eight pins per port, and most signals can
    // be routed to most pins via the high-speed I/O matrix (HSIOM).

    // "User LED" LED4 is on P13_7 (Port 13, Pin 7).
    // Set as output with strong drive and no input buffer.
    p.GPIO.prt13.cfg.write(|w| {
        w.in_en7().clear_bit();
        w.drive_mode7()
            .variant(psoc6_pac::gpio::prt::cfg::DRIVE_MODE0_A::STRONG.into());
        w
    });

    // "User Button" SW2 is P0_4 (Port 0, Pin 4)
    p.GPIO.prt0.cfg.write(|w| {
        w.in_en4().set_bit();
        w.drive_mode4()
            .variant(psoc6_pac::gpio::prt::cfg::DRIVE_MODE0_A::HIGHZ.into());
        w
    });

    // 50 MHz seems to be our default speed
    let mut delay = cortex_m::delay::Delay::new(cp.SYST, 50_000_000);

    loop {
        // Read button SW4
        hprintln!(
            "Button is {}",
            if p.GPIO.prt0.in_.read().in4().bit_is_clear() {
                "pressed"
            } else {
                "released"
            }
        );

        // Set GPIO pin high which turns LED off
        hprintln!("LED Off");
        p.GPIO.prt13.out_set.write(|w| {
            w.out7().set_bit();
            w
        });
        delay.delay_ms(1000);

        // Set GPIO pin low which turns LED on
        hprintln!("LED On");
        p.GPIO.prt13.out_clr.write(|w| {
            w.out7().set_bit();
            w
        });
        delay.delay_ms(250);
    }
}
