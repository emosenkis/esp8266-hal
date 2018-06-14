#![no_std]

extern crate embedded_hal;
extern crate libc;

mod bindings;
use bindings::*;

pub struct InputPin(u8);
pub struct OutputPin(u8);

impl InputPin {
    pub fn new(pin: u8) -> Self {
        InputPin::set_pin_mode(pin);
        InputPin::with_pin_mode_already_set(pin)
    }

    pub fn with_pin_mode_already_set(pin: u8) -> Self {
        InputPin(pin)
    }

    pub fn to_output(self) -> OutputPin {
        OutputPin::new(self.0)
    }

    fn set_pin_mode(pin: u8) {
        unsafe {
            pinMode(pin, INPUT as u8);
        }
    }
}

impl OutputPin {
    pub fn new(pin: u8) -> Self {
        OutputPin::set_pin_mode(pin);
        OutputPin::with_pin_mode_already_set(pin)
    }

    pub fn with_pin_mode_already_set(pin: u8) -> Self {
        OutputPin(pin)
    }

    pub fn to_input(self) -> InputPin {
        InputPin::new(self.0)
    }

    fn set_pin_mode(pin: u8) {
        unsafe {
            pinMode(pin, OUTPUT as u8);
        }
    }
}

impl embedded_hal::digital::InputPin for InputPin {
    fn is_low(&self) -> bool {
        let result;
        unsafe {
            result = digitalRead(self.0) as u8;
        }
        result == (LOW as u8)
    }
    fn is_high(&self) -> bool {
        let result;
        unsafe {
            result = digitalRead(self.0) as u8;
        }
        result == (HIGH as u8)
    }
}

impl embedded_hal::digital::OutputPin for OutputPin {
    fn set_low(&mut self) {
        unsafe {
            digitalWrite(self.0, LOW as u8);
        }
    }
    fn set_high(&mut self) {
        unsafe {
            digitalWrite(self.0, HIGH as u8);
        }
    }
}

impl embedded_hal::digital::StatefulOutputPin for OutputPin {
    fn is_set_low(&self) -> bool {
        let result;
        unsafe {
            result = digitalRead(self.0) as u8;
        }
        result == (LOW as u8)
    }
    fn is_set_high(&self) -> bool {
        let result;
        unsafe {
            result = digitalRead(self.0) as u8;
        }
        result == (HIGH as u8)
    }
}

impl embedded_hal::digital::toggleable::Default for OutputPin {}
