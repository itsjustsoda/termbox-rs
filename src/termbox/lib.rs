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
        match ffi::tb_init() {
            ffi::TB_EUNSUPPORTED_TERMINAL => Err(init_error::UnsupportedTerminal),
            ffi::TB_EFAILED_TO_OPEN_TTY => Err(init_error::FailedToOpenTTY),
            ffi::TB_EPIPE_TRAP_ERROR => Err(init_error::PipeTrapError),
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

    pub fn width(&self) -> int {
        unsafe { ffi::tb_width() as int }
    }

    pub fn height(&self) -> int {
        unsafe { ffi::tb_height() as int }
    }

    pub fn set_cursor(&self, cx: int, cy: int) {
        unsafe { ffi::tb_set_cursor(cx as i32, cy as i32) };
    }

    pub fn hide_cursor(&self) {
        unsafe { ffi::tb_set_cursor(ffi::TB_HIDE_CURSOR, ffi::TB_KEY_F1) };
    }

    pub fn put_cell(&self, x: int, y: int, cell: &cell::Cell) {
        let tb_cell = ffi::tb_cell {
            ch: cell.ch as u32,
            fg: cell.fg.to_u16(),
            bg: cell.bg.to_u16()
        };

        let tb_cell_ptr : *ffi::tb_cell = &tb_cell;

        unsafe { ffi::tb_put_cell(x as i32, y as i32, tb_cell_ptr) }
    }

    // TODO: blit()
 
    // TODO: select_input_mode()

    pub fn select_output_mode(&self, output: output::Output) -> output::Output {
        unsafe {
            match ffi::tb_select_output_mode(output as i32) {
                ffi::TB_OUTPUT_NORMAL => output::Normal,
                ffi::TB_OUTPUT_256  => output::_256,
                ffi::TB_OUTPUT_216  =>  output::_216,
                ffi::TB_OUTPUT_GRAYSCALE => output::Grayscale,
                _ => fail!("somehow returned a nonexistent output mode")
            }
        }
    }

    pub fn peek_event(&self, timeout: int) -> Result<event::Event, ~str> {
        unsafe { 
            let ev = ffi::tb_event {
                _type: 0,
                _mod: 0,
                key: 0,
                ch: 0,
                w: 0,
                h: 0
            };

            let ev_ptr : *ffi::tb_event = &ev;

            ffi::tb_peek_event(ev_ptr, timeout as i32);

            match ev._type {
                1 => Ok(event::KeyEvent(event::Mod::from_u8(ev._mod),
                        ev.key,
                        std::char::from_u32(ev.ch).unwrap())),
                2 => Ok(event::ResizeEvent(ev.w, ev.h)),
                _ => Err(box "event error: timeout")
            }
        }
    }

    pub fn poll_event(&self) -> Result<event::Event, ~str> {
        unsafe { 
            let ev = ffi::tb_event {
                _type: 0,
                _mod: 0,
                key: 0,
                ch: 0,
                w: 0,
                h: 0
            };

            let ev_ptr : *ffi::tb_event = &ev;

            ffi::tb_poll_event(ev_ptr);

            match ev._type {
                1 => Ok(event::KeyEvent(event::Mod::from_u8(ev._mod),
                        ev.key,
                        std::char::from_u32(ev.ch).unwrap())),
                2 => Ok(event::ResizeEvent(ev.w, ev.h)),
                _ => Err(box "event error")
            }
        }
    }
}

pub mod event {
    use ffi;

    pub enum Event {
        KeyEvent(Option<Mod>, u16, char),
        ResizeEvent(i32, i32)
    }

    pub enum EventType {
        Key = ffi::TB_EVENT_KEY as u8,
        Resize = ffi::TB_EVENT_RESIZE as u8
    }

    impl Mod {
        pub fn from_u8(i: u8) -> Option<Mod> {
            match i {
                1 => Some(Alt),
                _ => None
            }
        }
    }

    pub enum Mod {
        Alt = ffi::TB_MOD_ALT as u8
    }
}


pub mod output {
    use ffi;
    pub enum Output {
        Current = ffi::TB_OUTPUT_CURRENT,
        Normal = ffi::TB_OUTPUT_NORMAL,
        _256 = ffi::TB_OUTPUT_256,
        _216 = ffi::TB_OUTPUT_216,
        Grayscale = ffi::TB_OUTPUT_GRAYSCALE
    }
}
