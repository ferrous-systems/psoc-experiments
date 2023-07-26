#![no_std]
#![no_main]

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use panic_semihosting as _;

#[entry]
fn main() -> ! {
    hprintln!("peripheries are fun if they work or not");

    let p = unsafe { psoc6_pac::Peripherals::steal() };
    let cp = unsafe { cortex_m::Peripherals::steal() };

    // 50 MHz seems to be our default speed
    let mut delay = cortex_m::delay::Delay::new(cp.SYST, 50_000_000);
    
    loop {
        hprintln!("peri peri sauce");
        delay.delay_ms(1000);

        p.GPIO.prt8.cfg.write(|w| {
            w.in_en1().clear_bit();
            w
        });
        hprintln!("==================== {:?}", p.GPIO.prt8.cfg.read().bits());

        p.GPIO.prt8.cfg.write(|w| {
            w.in_en1().set_bit();
            w
        });
        hprintln!("==================== {:?}", p.GPIO.prt8.cfg.read().bits());

        p.GPIO.prt8.cfg.write(|w| {
            w.in_en3().set_bit();
            w
        });
        hprintln!("==================== {:?}", p.GPIO.prt8.cfg.read().bits());

        p.GPIO.prt8.cfg.write(|w| {
            w.in_en7().set_bit();
            w
        });
        hprintln!("==================== {:?}", p.GPIO.prt8.cfg.read().bits());
    }
}