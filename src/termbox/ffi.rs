#![allow(non_camel_case_types)]

extern crate libc;

use libc::{c_char, c_int, int32_t, uint8_t, uint16_t, uint32_t};

// Keys
pub static TB_KEY_F1               : c_int = 0xFFFF - 0;
pub static TB_KEY_F2               : c_int = 0xFFFF - 1;
pub static TB_KEY_F3               : c_int = 0xFFFF - 2;
pub static TB_KEY_F4               : c_int = 0xFFFF - 3;
pub static TB_KEY_F5               : c_int = 0xFFFF - 4;
pub static TB_KEY_F6               : c_int = 0xFFFF - 5;
pub static TB_KEY_F7               : c_int = 0xFFFF - 6;
pub static TB_KEY_F8               : c_int = 0xFFFF - 7;
pub static TB_KEY_F9               : c_int = 0xFFFF - 8;
pub static TB_KEY_F10              : c_int = 0xFFFF - 9;
pub static TB_KEY_F11              : c_int = 0xFFFF - 10;
pub static TB_KEY_F12              : c_int = 0xFFFF - 11;
pub static TB_KEY_INSERT           : c_int = 0xFFFF - 12;
pub static TB_KEY_DELETE           : c_int = 0xFFFF - 13;
pub static TB_KEY_HOME             : c_int = 0xFFFF - 14;
pub static TB_KEY_END              : c_int = 0xFFFF - 15;
pub static TB_KEY_PGUP             : c_int = 0xFFFF - 16;
pub static TB_KEY_PGDN             : c_int = 0xFFFF - 17;
pub static TB_KEY_ARROW_UP         : c_int = 0xFFFF - 18;
pub static TB_KEY_ARROW_DOWN       : c_int = 0xFFFF - 19;
pub static TB_KEY_ARROW_LEFT       : c_int = 0xFFFF - 20;
pub static TB_KEY_ARROW_RIGHT      : c_int = 0xFFFF - 21;

pub static TB_KEY_CTRL_TILDE       : c_int = 0x00;
pub static TB_KEY_CTRL_2           : c_int = 0x00;
pub static TB_KEY_CTRL_A           : c_int = 0x01;
pub static TB_KEY_CTRL_B           : c_int = 0x02;
pub static TB_KEY_CTRL_C           : c_int = 0x03;
pub static TB_KEY_CTRL_D           : c_int = 0x04;
pub static TB_KEY_CTRL_E           : c_int = 0x05;
pub static TB_KEY_CTRL_F           : c_int = 0x06;
pub static TB_KEY_CTRL_G           : c_int = 0x07;
pub static TB_KEY_CTRL_H           : c_int = 0x08; // clash with 'CTRL_BACKSPACE'
pub static TB_KEY_BACKSPACE        : c_int = 0x08;
pub static TB_KEY_TAB              : c_int = 0x09;
pub static TB_KEY_CTRL_I           : c_int = 0x09; // clash with 'TAB'
pub static TB_KEY_CTRL_J           : c_int = 0x0A;
pub static TB_KEY_CTRL_K           : c_int = 0x0B;
pub static TB_KEY_CTRL_L           : c_int = 0x0C;
pub static TB_KEY_ENTER            : c_int = 0x0D;
pub static TB_KEY_CTRL_M           : c_int = 0x0D; // clash with 'ENTER'
pub static TB_KEY_CTRL_N           : c_int = 0x0E;
pub static TB_KEY_CTRL_O           : c_int = 0x0F;
pub static TB_KEY_CTRL_P           : c_int = 0x10;
pub static TB_KEY_CTRL_Q           : c_int = 0x11;
pub static TB_KEY_CTRL_R           : c_int = 0x12;
pub static TB_KEY_CTRL_S           : c_int = 0x13;
pub static TB_KEY_CTRL_T           : c_int = 0x14;
pub static TB_KEY_CTRL_U           : c_int = 0x15;
pub static TB_KEY_CTRL_V           : c_int = 0x16;
pub static TB_KEY_CTRL_W           : c_int = 0x17;
pub static TB_KEY_CTRL_X           : c_int = 0x18;
pub static TB_KEY_CTRL_Y           : c_int = 0x19;
pub static TB_KEY_CTRL_Z           : c_int = 0x1A;
pub static TB_KEY_ESC              : c_int = 0x1B;
pub static TB_KEY_CTRL_LSQ_BRACKET : c_int = 0x1B; // clash with 'ESC'
pub static TB_KEY_CTRL_3           : c_int = 0x1B; // clash with 'ESC'
pub static TB_KEY_CTRL_4           : c_int = 0x1C;
pub static TB_KEY_CTRL_BACKSLASH   : c_int = 0x1C; // clash with 'CTRL_4'
pub static TB_KEY_CTRL_5           : c_int = 0x1D;
pub static TB_KEY_CTRL_RSQ_BRACKET : c_int = 0x1D; // clash with 'CTRL_5'
pub static TB_KEY_CTRL_6           : c_int = 0x1E;
pub static TB_KEY_CTRL_7           : c_int = 0x1F;
pub static TB_KEY_CTRL_SLASH       : c_int = 0x1F; // clash with 'CTRL_7'
pub static TB_KEY_CTRL_UNDERSCORE  : c_int = 0x1F; // clash with 'CTRL_7'
pub static TB_KEY_SPACE            : c_int = 0x20;
pub static TB_KEY_BACKSPACE2       : c_int = 0x7F;
pub static TB_KEY_CTRL_8           : c_int = 0x7F; // clash with 'DELETE'

pub static TB_MOD_ALT              : c_int = 0x01;

// Colors
pub static TB_DEFAULT   : c_int = 0x00;
pub static TB_BLACK     : c_int = 0x01;
pub static TB_RED       : c_int = 0x02;
pub static TB_GREEN     : c_int = 0x03;
pub static TB_YELLOW    : c_int = 0x04;
pub static TB_BLUE      : c_int = 0x05;
pub static TB_MAGENTA   : c_int = 0x06;
pub static TB_CYAN      : c_int = 0x07;
pub static TB_WHITE     : c_int = 0x08;

// Attributes
pub static TB_BOLD      : c_int = 0x0100;
pub static TB_UNDERLINE : c_int = 0x0200;
pub static TB_REVERSE   : c_int = 0x0400;

pub struct tb_cell {
    ch: uint32_t,
    fg: uint16_t,
    bg: uint16_t
}

// Events
pub static TB_EVENT_KEY    : c_int = 1;
pub static TB_EVENT_RESIZE : c_int = 2;

pub struct tb_event {
    _type : uint8_t,
    _mod  : uint8_t,
    key   : uint16_t,
    ch    : uint32_t,
    w     : int32_t,
    h     : int32_t
}

pub static TB_EUNSUPPORTED_TERMINAL : c_int = -1;
pub static TB_EFAILED_TO_OPEN_TTY   : c_int = -2;
pub static TB_EPIPE_TRAP_ERROR      : c_int = -3;

pub static TB_HIDE_CURSOR : c_int = -1;

// Outputs
pub static TB_OUTPUT_CURRENT   : c_int = 0;
pub static TB_OUTPUT_NORMAL    : c_int = 1;
pub static TB_OUTPUT_256       : c_int = 2;
pub static TB_OUTPUT_216       : c_int = 3;
pub static TB_OUTPUT_GRAYSCALE : c_int = 4;

// Utility
pub static TB_EOF : c_int = -1;

#[link(name = "termbox")]
extern "C" {
    pub fn tb_init() -> c_int;
    pub fn tb_shutdown();

    pub fn tb_width() -> c_int;
    pub fn tb_height() -> c_int;

    pub fn tb_clear();
    pub fn tb_set_clear_attributes(fg: uint16_t, bg: uint16_t);

    pub fn tb_present();

    pub fn tb_set_cursor(cx: c_int, cy: c_int);

    pub fn tb_put_cell(x: c_int, y: c_int, cell: *tb_cell);
    pub fn tb_change_cell(x: c_int, y: c_int, ch: uint32_t, fg: uint16_t, bg: uint16_t);

    pub fn tb_blit(x: c_int, y: c_int, w: c_int, h: c_int, cells: *tb_cell);

    pub fn tb_select_input_mode(mode: c_int) -> c_int;
    pub fn tb_select_output_mode(mode: c_int) -> c_int;

    pub fn tb_peek_event(event: *mut tb_event, timeout: c_int);
    pub fn tb_poll_event(event: *mut tb_event);

    pub fn tb_utf8_char_length(c: c_char) -> c_int;
    pub fn tb_utf8_char_to_unicode(out: *uint32_t, c: *c_char) -> c_int;
    pub fn tb_utf8_unicode_to_char(out: *char, c: *uint32_t) -> c_int;
}
