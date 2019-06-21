#![allow(dead_code)]
#![no_std]
#![no_main]

use arduino_simple_io::io::{IOState::*, Input, Output, Pin};

#[no_mangle]
pub extern "C" fn main() {
    let mut inputs = [Pin::<Input>::port(3), Pin::port(4), Pin::port(5)];
    let mut pin13 = Pin::<Output>::port(13);

    inputs.iter_mut().for_each(|i| i.on());

    loop {
        let mut flg = false;
        for i in inputs.iter() {
            if let OFF = i.read() {
                flg = true;
            }
        }

        if flg {
            pin13.on();
        } else {
            pin13.off();
        }
    }
}
