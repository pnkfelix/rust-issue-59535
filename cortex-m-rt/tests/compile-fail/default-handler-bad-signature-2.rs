#![no_main]
#![no_std]

extern crate cortex_m_rt;
extern crate panic_halt;

use cortex_m_rt::{entry, exception};

#[entry]
fn foo() -> ! {
    loop {}
}

#[exception]
fn DefaultHandler(_irqn: i16) -> u32 {
    //~^ ERROR `DefaultHandler` must have signature `[unsafe] fn(i16) [-> !]`
    0
}
