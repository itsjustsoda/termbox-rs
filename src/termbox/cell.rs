mod ffi;
// use ffi; // FIXME

pub struct Cell {
    pub ch: char,
    pub fg: Style,
    pub bg: Style
}

pub enum Color {
    Default = ffi::TB_DEFAULT as u8,
    Black = ffi::TB_BLACK as u8,
    Red = ffi::TB_RED as u8,
    Green = ffi::TB_GREEN as u8,
    Yellow = ffi::TB_YELLOW as u8,
    Blue = ffi::TB_BLUE as u8,
    Magenta = ffi::TB_MAGENTA as u8,
    Cyan = ffi::TB_CYAN as u8,
    White = ffi::TB_WHITE as u8,
}

pub enum Attribute {
    Bold = ffi::TB_BOLD as u16,
    Underline = ffi::TB_UNDERLINE as u16,
    Reverse = ffi::TB_REVERSE as u16 
}

pub enum Style {
    ColorStyle(Color),
    Attr(Attribute),
    ColorAttr1(Color, Attribute),
    ColorAttr2(Color, Attribute, Attribute),
    ColorAttr3(Color, Attribute, Attribute, Attribute)
}

impl Style {
    pub fn to_u16(&self) -> u16 {
        match *self {
            ColorStyle(c) => c as u16,
            Attr(a) => a as u16,
            ColorAttr1(c, a) => c as u16 | a as u16,
            ColorAttr2(c, a, a2) => c as u16 | a as u16 | a2 as u16,
            ColorAttr3(c, a, a2, a3) => c as u16 | a as u16 | a2 as u16 | a3 as u16
        }
    }
}

