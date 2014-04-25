#![crate_id = "termbox#0.1.0-pre"]

#![desc = "Termbox-rs"]
#![license = "MIT"]

#![crate_type = "lib"]

extern crate libc;

pub mod ffi;
pub mod key;
pub mod cell;
pub mod init_error;

pub struct Termbox;

pub fn init() -> Result<Termbox, init_error::InitError> {
    unsafe {
        match ffi::tb_init() as int  {
            -1 => Err(init_error::UnsupportedTerminal),
            -2 => Err(init_error::FailedToOpenTTY),
            -3 => Err(init_error::PipeTrapError),
            _ =>  Ok(Termbox)
        }
    }
}

impl Termbox {
    pub fn shutdown(&self) {
        unsafe { ffi::tb_shutdown() };
    }

    pub fn present(&self) {
        unsafe { ffi::tb_present() };
    }
}

pub enum Mod {
    Alt = ffi::TB_MOD_ALT
}

pub enum EventType {
    Key = ffi::TB_EVENT_KEY,
    Resize = ffi::TB_EVENT_RESIZE
}

pub enum Output {
    Current = ffi::TB_OUTPUT_CURRENT,
    Normal = ffi::TB_OUTPUT_NORMAL,
    _256 = ffi::TB_OUTPUT_256,
    _216 = ffi::TB_OUTPUT_216,
    Grayscale = ffi::TB_OUTPUT_GRAYSCALE
}
