/* automatically generated by rust-bindgen */

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
extern crate libc;

pub const HIGH: libc::c_uint = 1;
pub const LOW: libc::c_uint = 0;
pub const INPUT: libc::c_uint = 0;
pub const OUTPUT: libc::c_uint = 1;
pub type __uint8_t = libc::c_uchar;
extern "C" {
    pub fn pinMode(pin: u8, mode: u8);
}
extern "C" {
    pub fn digitalWrite(pin: u8, val: u8);
}
extern "C" {
    pub fn digitalRead(pin: u8) -> libc::c_int;
}
