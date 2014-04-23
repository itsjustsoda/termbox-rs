#![crate_id = "termbox#0.1.0-pre"]

#![desc = "Termbox-rs"]
#![license = "MIT"]

#![crate_type = "lib"]

extern crate libc;

pub mod ffi;
pub mod keys;

pub fn init() -> int {
    unsafe { ffi::tb_init() as int }
}

pub fn shutdown() {
    unsafe { ffi::tb_shutdown() };
}

pub fn present() {
    unsafe { ffi::tb_present() };
}

pub enum Mod {
    Alt = ffi::TB_MOD_ALT
}

pub enum Color {
    Default = ffi::TB_DEFAULT,
    Black = ffi::TB_BLACK,
    Red = ffi::TB_RED,
    Green = ffi::TB_GREEN,
    Yellow = ffi::TB_YELLOW,
    Blue = ffi::TB_BLUE,
    Magenta = ffi::TB_MAGENTA,
    Cyan = ffi::TB_CYAN,
    White = ffi::TB_WHITE
}

pub enum Attributes {
    Bold = ffi::TB_BOLD,
    Underline = ffi::TB_UNDERLINE,
    Reverse = ffi::TB_REVERSE
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
