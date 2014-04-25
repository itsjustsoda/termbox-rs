extern crate termbox;

use termbox::init;
use termbox::Termbox;

fn main() {
    let init = termbox::init();
    let tb : Termbox;

    match init {
        Ok(termbox) => {
            tb = termbox;

            tb.present();
            tb.shutdown();
        }
        Err(e) => {
            fail!("error initializing termbox: {}", e);
        }
    }
}
