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
        delay.delay_ms(1000);
        hprintln!("---- Loop start ----");

        // PORT8_1 is supposedly a button on the Capsense
        // reset it to 0
        p.GPIO.prt8.cfg.write(|w| {
            w.in_en1().clear_bit();
            w
        });
        hprintln!("Port8 bits: {:#32x}", p.GPIO.prt8.cfg.read().bits());
        hprintln!("Capsense sense duty bits: {:#32x}", p.CSD0.sense_duty.read().bits());

        // PORT8_1 is supposedly a button on the Capsense
        // set the bit to ... high? on?
        p.GPIO.prt8.cfg.write(|w| {
            w.in_en1().set_bit();
            w
        });
        hprintln!("Port8 bits: {:#32x}", p.GPIO.prt8.cfg.read().bits());
        hprintln!("Capsense sense duty bits: {:#32x}", p.CSD0.sense_duty.read().bits());

        // PORT8_3 to PORT8_7 are slider intervals on the Capsense
        p.GPIO.prt8.cfg.write(|w| {
            w.in_en7().set_bit();
            w
        });
        hprintln!("Port8 bits: {:#32x}", p.GPIO.prt8.cfg.read().bits());
        // trying to find out how to set the sense_duty
        // how do we access the value from peripheral rather than setting it here?
        p.CSD0.sense_duty.write(|w| unsafe {
            w.bits(21);
            w
        });
        hprintln!("Capsense sense duty bits: {:#32x}", p.CSD0.sense_duty.read().bits());

        // Lower the Capsense slider for kicks
        p.GPIO.prt8.cfg.write(|w| {
            w.in_en4().set_bit();
            w
        });
        hprintln!("Port8 bits: {:#32x}", p.GPIO.prt8.cfg.read().bits());
        hprintln!("Capsense sense duty bits: {:#32x}", p.CSD0.sense_duty.read().bits());

        hprintln!("---- Loop End ----");
    }
}