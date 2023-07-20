#![deny(warnings)]
#![no_main]
#![no_std]

use panic_semihosting as _;

#[rtic::app(device = psoc6_pac, peripherals = true, dispatchers = [SCB_12_INTERRUPT])]
mod app {
    // use cortex_m_semihosting::hprintln;
    use systick_monotonic::*;

    #[monotonic(binds = SysTick, default = true)]
    type MyMono = Systick<100>; // 100 Hz / 10 ms granularity

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init(local = [x: u32 = 0])]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        // Fixup the VTOR that the ROM/bootloader didn't set
        extern "C" {
            static _svectors: u32;
        }
        // hprintln!("VTOR was: 0x{:08x}", cx.core.SCB.vtor.read());
        unsafe {
            cx.core.SCB.vtor.write(&_svectors as *const u32 as u32);
        }
        // hprintln!("VTOR is : 0x{:08x}", cx.core.SCB.vtor.read());

        // Initialize the monotonic timer (SysTick rate on PSoC6 is 50 MHz)
        let mono = Systick::new(cx.core.SYST, 50_000_000);

        // "User LED" LED4 is on P13_7 (Port 13, Pin 7).
        // Set as output with strong drive and no input buffer.
        cx.device.GPIO.prt13.cfg.write(|w| {
            w.in_en7().clear_bit();
            w.drive_mode7()
                .variant(psoc6_pac::gpio::prt::cfg::DRIVE_MODE0_A::STRONG.into());
            w
        });

        // "User Button" SW2 is P0_4 (Port 0, Pin 4)
        cx.device.GPIO.prt0.cfg.write(|w| {
            w.in_en4().set_bit();
            w.drive_mode4()
                .variant(psoc6_pac::gpio::prt::cfg::DRIVE_MODE0_A::HIGHZ.into());
            w
        });

        on::spawn().unwrap();

        // hprintln!(">> init complete");

        (Shared {}, Local {}, init::Monotonics(mono))
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            cortex_m::asm::wfe();
        }
    }

    #[task]
    fn on(_: on::Context) {
        // hprintln!(">> on");
        let p = unsafe { psoc6_pac::Peripherals::steal() };
        p.GPIO.prt13.out_clr.write(|w| {
            w.out7().set_bit();
            w
        });
        off::spawn_at(monotonics::now() + 1.secs()).unwrap();
    }

    #[task]
    fn off(_: off::Context) {
        // hprintln!(">> off");
        let p = unsafe { psoc6_pac::Peripherals::steal() };
        p.GPIO.prt13.out_set.write(|w| {
            w.out7().set_bit();
            w
        });
        on::spawn_at(monotonics::now() + 1.secs()).unwrap();
    }
}
