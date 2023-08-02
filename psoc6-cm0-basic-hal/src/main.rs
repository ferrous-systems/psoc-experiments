#![no_std]
#![no_main]

use core::marker::PhantomData;

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use embedded_hal::digital::v2::OutputPin;

#[derive(Debug)]
enum Error {}

#[allow(unused)]
struct GpioPort<const PORT: usize> {
    pin0: GpioPin<PORT, 0>,
    pin1: GpioPin<PORT, 1>,
    pin2: GpioPin<PORT, 2>,
    pin3: GpioPin<PORT, 3>,
    pin4: GpioPin<PORT, 4>,
    pin5: GpioPin<PORT, 5>,
    pin6: GpioPin<PORT, 6>,
    pin7: GpioPin<PORT, 7>,
}

fn make_ports(_pac_object: psoc6_pac::GPIO) -> (GpioPort<0>, GpioPort<13>) {
    unsafe { (GpioPort::<0>::new_port0(), GpioPort::<13>::new_port13()) }
}

impl GpioPort<0> {
    const unsafe fn new_port0() -> GpioPort<0> {
        GpioPort {
            pin0: GpioPin {
                _inner: PhantomData,
            },
            pin1: GpioPin {
                _inner: PhantomData,
            },
            pin2: GpioPin {
                _inner: PhantomData,
            },
            pin3: GpioPin {
                _inner: PhantomData,
            },
            pin4: GpioPin {
                _inner: PhantomData,
            },
            pin5: GpioPin {
                _inner: PhantomData,
            },
            pin6: GpioPin {
                _inner: PhantomData,
            },
            pin7: GpioPin {
                _inner: PhantomData,
            },
        }
    }
}

impl GpioPort<13> {
    const unsafe fn new_port13() -> GpioPort<13> {
        GpioPort {
            pin0: GpioPin {
                _inner: PhantomData,
            },
            pin1: GpioPin {
                _inner: PhantomData,
            },
            pin2: GpioPin {
                _inner: PhantomData,
            },
            pin3: GpioPin {
                _inner: PhantomData,
            },
            pin4: GpioPin {
                _inner: PhantomData,
            },
            pin5: GpioPin {
                _inner: PhantomData,
            },
            pin6: GpioPin {
                _inner: PhantomData,
            },
            pin7: GpioPin {
                _inner: PhantomData,
            },
        }
    }
}

struct GpioPin<const PORT: usize, const PIN: usize> {
    _inner: PhantomData<*const ()>,
}

impl<const PORT: usize, const PIN: usize> embedded_hal::digital::v2::OutputPin
    for GpioPin<PORT, PIN>
{
    type Error = Error;

    fn set_low(&mut self) -> Result<(), Self::Error> {
        let p = unsafe { psoc6_pac::Peripherals::steal() };
        let ptr = match PORT {
            0 => &p.GPIO.prt0,
            13 => &p.GPIO.prt13,
            _ => {
                unreachable!()
            }
        };

        unsafe {
            ptr.out_clr.write_with_zero(|w| w.bits(1 << PIN));
        }

        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        let p = unsafe { psoc6_pac::Peripherals::steal() };
        let ptr = match PORT {
            0 => &p.GPIO.prt0,
            13 => &p.GPIO.prt13,
            _ => {
                unreachable!()
            }
        };

        unsafe {
            ptr.out_set.write_with_zero(|w| w.bits(1 << PIN));
        }

        Ok(())
    }
}

#[entry]
fn main() -> ! {
    hprintln!("Hello, I am the Cortex-M0+!");

    let p = unsafe { psoc6_pac::Peripherals::steal() };

    let test_cases = [
        "PUBLISH ice cream\n",
        "RETRIEVE\n",
        "MISTAKE\n",
        "oops this isn't valid at all",
    ];

    // let gpio_port13_out = 0x40310680 as *mut u32;
    // let gpio_port13_cfg = 0x403106C4 as *mut u32;

    // Peripheral Access Crate
    p.GPIO.prt13.cfg.modify(|_r, w| {
        w.drive_mode7().variant(6);
        w
    });

    p.GPIO.prt13.out_clr.write(|w| {
        w.out7().set_bit();
        w
    });

    // Mid-Level Drivers
    let (mut _port0, mut port13) = make_ports(p.GPIO);
    for _ in 0..10 {
        port13.pin7.set_low().unwrap();
        for _ in 0..1000000 {
            unsafe { core::arch::asm!("nop") };
        }
        port13.pin7.set_high().unwrap();
        for _ in 0..1000000 {
            unsafe { core::arch::asm!("nop") };
        }
    }

    // Board Support Package
    // let board = Board::new();
    // board.led.on();

    // let led_pin = 7;

    // unsafe {
    //     gpio_port13_cfg.write_volatile(6 << (led_pin * 4));
    //     gpio_port13_out.write_volatile(0);
    // }

    for test_case in test_cases {
        let result = simple_db::parse(test_case);
        hprintln!("String {:?} => {:?}", test_case, result);
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
