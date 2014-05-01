extern crate termbox;

use termbox::init;
use termbox::Termbox;
use termbox::cell;

fn main() {
    let init = termbox::init();
    let tb : Termbox;

    match init {
        Ok(termbox) => {
            tb = termbox;
        }
        Err(e) => {
            fail!("error initializing termbox: {}", e);
        }
    }

    tb.blit(1, 3, 10, 10,
        [
            cell::Cell {
                ch: 'a',
                fg: cell::ColorStyle(cell::Red),
                bg: cell::ColorStyle(cell::Yellow)
            },
            ..1
        ]);

    tb.present();
    tb.poll_event();
    tb.shutdown();
}
