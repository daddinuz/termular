use std::time::Duration;
use std::{io, thread};
use term::Term;

fn main() {
    let stdout = io::stdout();
    let mut term = Term::new(stdout.lock());

    term.alternative_screen(true).unwrap();
    term.cursor()
        .to((0, 0).into())
        .right(8)
        .down(8)
        .flush()
        .unwrap();

    println!("Hello world!");

    thread::sleep(Duration::from_secs(5));
    term.alternative_screen(false).unwrap();
}
