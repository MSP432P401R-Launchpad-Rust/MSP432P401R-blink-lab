
#![no_main] // Tell the linker not to look for a standard main function, because we don't have one.
#![no_std]  // Do not include the standard library. This target can't handle it. Do note that you can still include specific parts of the library.

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate msp432p401r;

// Implements our panic function. You can implement your own if you need to.
extern crate panic_halt;

use cortex_m_rt::entry;
use cortex_m::asm;

#[entry]
fn main() -> ! {
    // Rust does not like globals. Since the registers of your hardware are implemented
    // as globals, authors of this access library have created a safe wrapper for them
    // that is implemented as a singleton. Trying to take this singleton twice will cause
    // a panic.
    let p = msp432p401r::Peripherals::take().unwrap();

    // Get the Watchdog Timer
    let wdt = p.WDT_A;

    // We shall disable the timer.
    wdt.wdtctl.write(|w| {
        // You must provide the "password" to access the watchdog timer's registers.
        unsafe {
            w.wdtpw().bits(0x5A); // Password to give us access to the registers. See page 763 of the MSP432P4XX Reference Manual.
        }
        w.wdthold().bit(true) // Disable the timer.
    });

    // Get the Digital I/O module
    let dio = p.DIO;

    // The red LED is on port 2 pin 0. Set it to be an output.
    dio.padir.modify(|r, w| unsafe { w.p2dir().bits(r.p2dir().bits() | 1) });

    // This loop will blink the red LED.
    loop {
        // Set LED output to on.
        dio.paout.modify(|r, w| unsafe { w.p2out().bits(r.p2out().bits() | 1) });
        asm::delay(1000000); // Lame way to delay. Just spins the processor for X number of instructions.

        // Let LED output to off.
        dio.paout.modify(|r, w| unsafe { w.p2out().bits(r.p2out().bits() & 0) });
        asm::delay(1000000);
    }
}