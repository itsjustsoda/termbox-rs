mod ffi;
// use ffi; // FIXME

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
