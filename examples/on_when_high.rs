#![allow(dead_code)]
#![no_std]
#![no_main]

use arduino_simple_io::delay::delay_ms;
use arduino_simple_io::io::{IOState::*, Input, Output, Pin};

#[no_mangle]
pub extern "C" fn main() {
    let mut pin13: Pin<Output> = Pin::port(13);
    let mut pin10: Pin<Input> = Pin::port(10);

    loop {
        let state = pin10.read();
        match state {
            ON => pin13.on(),
            OFF => pin13.off(),
        }
    }
}
