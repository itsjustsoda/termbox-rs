use std::fmt;
//use ffi; // FIXME
pub mod ffi;

pub enum InitError {
    UnsupportedTerminal = ffi::TB_EUNSUPPORTED_TERMINAL,
    FailedToOpenTTY = ffi::TB_EFAILED_TO_OPEN_TTY,
    PipeTrapError = ffi::TB_EPIPE_TRAP_ERROR
}

impl fmt::Show for InitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UnsupportedTerminal => "unsupported terminal".fmt(f),
            FailedToOpenTTY => "failed to open tty".fmt(f),
            PipeTrapError => "pipe trap error".fmt(f)
        }
    }

}
