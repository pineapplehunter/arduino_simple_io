#![allow(dead_code)]
#![no_std]
#![no_main]

use arduino_simple_io::delay::delay_ms;
use arduino_simple_io::io::{Output, Pin};

#[no_mangle]
pub extern "C" fn main() {
    let mut pin13: Pin<Output> = Pin::port(13);

    loop {
        delay_ms(1000);
        pin13.toggle();
    }
}
