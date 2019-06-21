use arduino::{DDRB, DDRD, PINB, PIND, PORTB, PORTD};
use core::marker::PhantomData;
use core::ptr::{read_volatile, write_volatile};

pub trait IO: Sized {
    const OUT: bool;
}
pub struct Output;
pub struct Input;
impl IO for Output {
    const OUT: bool = true;
}
impl IO for Input {
    const OUT: bool = false;
}

#[derive(PartialEq, Clone)]
pub enum IOState {
    ON,
    OFF,
}

#[derive(Clone)]
pub struct Pin<T> {
    phantom: PhantomData<T>,
    port: PORT,
    state: IOState,
    mask: u8,
}

#[derive(Clone)]
enum PORT {
    A,
    B,
    C,
    D,
}

impl<T: IO> Pin<T> {
    pub fn port(num: u8) -> Self {
        let port = match num {
            0..=1 => unimplemented!(),
            2..=7 => PORT::D,
            8..=13 => PORT::B,
            _ => unimplemented!(),
        };

        let (ptr, offset) = match port {
            PORT::D => (DDRD, 0),
            PORT::B => (DDRB, 8),
            _ => unimplemented!(),
        };

        let mask = 1 << (num - offset);
        match T::OUT {
            true => unsafe { write_volatile(ptr, *ptr | mask) },
            false => unsafe { write_volatile(ptr, *ptr & !mask) },
        };

        let mut s = Self {
            phantom: PhantomData,
            port,
            state: IOState::OFF,
            mask,
        };
        s.off();
        s
    }

    pub fn toggle(&mut self) {
        use self::IOState::*;
        match self.state {
            OFF => self.on(),
            ON => self.off(),
        };
    }

    pub fn on(&mut self) {
        let ptr = match self.port {
            PORT::D => PORTD,
            PORT::B => PORTB,
            _ => unreachable!(),
        };
        unsafe { write_volatile(ptr, *ptr | self.mask) };
        self.state = IOState::ON;
    }

    pub fn off(&mut self) {
        let ptr = match self.port {
            PORT::D => PORTD,
            PORT::B => PORTB,
            _ => unreachable!(),
        };
        unsafe { write_volatile(ptr, *ptr & !self.mask) };
        self.state = IOState::OFF;
    }

    pub fn read(&self) -> IOState {
        let ptr = match self.port {
            PORT::D => PIND,
            PORT::B => PINB,
            _ => unreachable!(),
        };
        let val = unsafe { read_volatile(ptr) } & self.mask;

        if val == 0 {
            IOState::OFF
        } else {
            IOState::ON
        }
    }
}
