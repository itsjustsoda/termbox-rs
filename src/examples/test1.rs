extern crate termbox;

use termbox::init;
use termbox::Termbox;

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

    tb.set_cursor(1, 2);
    tb.put_cell(1, 3, &termbox::cell::Cell {
        ch: 'a',
        fg: termbox::cell::ColorStyle(termbox::cell::Red),
        bg: termbox::cell::ColorStyle(termbox::cell::Yellow)
    });
    tb.present();

    let ev: termbox::event::Event = tb.poll_event().unwrap();

    match ev {
        termbox::event::KeyEvent(a, b, c) => println!("{}", c),
        _ => fail!()
    }

    tb.poll_event();
    tb.shutdown();
}
