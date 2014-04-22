#![crate_id = "termbox#0.1.0-pre"]

#![desc = "Termbox-rs"]
#![license = "MIT"]

#![crate_type = "lib"]

extern crate libc;

pub mod ffi;

pub fn init() -> int {
    unsafe { ffi::tb_init() as int }
}

pub fn shutdown() {
    unsafe { ffi::tb_shutdown() }
}
