use std::time::Duration;
use std::{io, thread};
use term::screen::Buffer;
use term::Term;

fn main() {
    let stdout = io::stdout();
    let mut term = Term::new(stdout.lock());

    term.screen()
        .set_buffer(Buffer::Alternative)
        .clear()
        .flush()
        .unwrap();
    term.cursor().to((8, 8).into()).flush().unwrap();

    println!("Hello world!");

    thread::sleep(Duration::from_secs(2));
    term.screen()
        .clear()
        .set_buffer(Buffer::Canonical)
        .flush()
        .unwrap();
}
