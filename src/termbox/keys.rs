pub mod ffi;

pub enum Key {
    F1         = ffi::TB_KEY_F1,
    F2         = ffi::TB_KEY_F2,
    F3         = ffi::TB_KEY_F3,
    F4         = ffi::TB_KEY_F4,
    F5         = ffi::TB_KEY_F5,
    F6         = ffi::TB_KEY_F6,
    F7         = ffi::TB_KEY_F7,
    F8         = ffi::TB_KEY_F8,
    F9         = ffi::TB_KEY_F9,
    F10        = ffi::TB_KEY_F10,
    F11        = ffi::TB_KEY_F11,
    F12        = ffi::TB_KEY_F12,
    Insert     = ffi::TB_KEY_INSERT,
    Delete     = ffi::TB_KEY_DELETE,
    Home       = ffi::TB_KEY_HOME,
    End        = ffi::TB_KEY_END,
    PgUp       = ffi::TB_KEY_PGUP,
    PgDn       = ffi::TB_KEY_PGDN,
    ArrowUp    = ffi::TB_KEY_ARROW_UP,
    ArrowDown  = ffi::TB_KEY_ARROW_DOWN,
    ArrowLeft  = ffi::TB_KEY_ARROW_LEFT,
    ArrowRight = ffi::TB_KEY_ARROW_RIGHT,

    CtrlTilde  = ffi::TB_KEY_CTRL_TILDE,
    CtrlA      = ffi::TB_KEY_CTRL_A,
    CtrlB      = ffi::TB_KEY_CTRL_B,
    CtrlC      = ffi::TB_KEY_CTRL_C,
    CtrlD      = ffi::TB_KEY_CTRL_D,
    CtrlE      = ffi::TB_KEY_CTRL_E,
    CtrlF      = ffi::TB_KEY_CTRL_F,
    CtrlG      = ffi::TB_KEY_CTRL_G,
    CtrlH      = ffi::TB_KEY_CTRL_H,
    Tab        = ffi::TB_KEY_TAB,
    CtrlJ      = ffi::TB_KEY_CTRL_J,
    CtrlK      = ffi::TB_KEY_CTRL_K,
    CtrlL      = ffi::TB_KEY_CTRL_L,
    Enter      = ffi::TB_KEY_ENTER,
    CtrlN      = ffi::TB_KEY_CTRL_N,
    CtrlO      = ffi::TB_KEY_CTRL_O,
    CtrlP      = ffi::TB_KEY_CTRL_P,
    CtrlQ      = ffi::TB_KEY_CTRL_Q,
    CtrlR      = ffi::TB_KEY_CTRL_R,
    CtrlS      = ffi::TB_KEY_CTRL_S,
    CtrlT      = ffi::TB_KEY_CTRL_T,
    CtrlU      = ffi::TB_KEY_CTRL_U,
    CtrlV      = ffi::TB_KEY_CTRL_V,
    CtrlW      = ffi::TB_KEY_CTRL_W,
    CtrlX      = ffi::TB_KEY_CTRL_X,
    CtrlY      = ffi::TB_KEY_CTRL_Y,
    CtrlZ      = ffi::TB_KEY_CTRL_Z,
    Esc        = ffi::TB_KEY_ESC,
    Ctrl4      = ffi::TB_KEY_CTRL_4,
    Ctrl5      = ffi::TB_KEY_CTRL_5,
    Ctrl6      = ffi::TB_KEY_CTRL_6,
    Ctrl7      = ffi::TB_KEY_CTRL_7,
    Space      = ffi::TB_KEY_SPACE,
    Backspace2 = ffi::TB_KEY_BACKSPACE2,
}

// Clashing codes
pub static Backspace      : Key = CtrlH;
pub static Ctrl2          : Key = CtrlTilde;
pub static CtrlI          : Key = Tab;
pub static CtrlM          : Key = Enter;
pub static CtrlLsqBracket : Key = Esc;
pub static Ctrl3          : Key = Esc;
pub static CtrlBackslash  : Key = Ctrl4;
pub static CtrlRsqBracket : Key = Ctrl5;
pub static CtrlSlash      : Key = Ctrl7;
pub static CtrlUnderscore : Key = Ctrl7;
pub static Ctrl8          : Key = Backspace2;
