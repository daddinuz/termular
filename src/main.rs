use std::{io, time::Duration};
use term::{screen::Buffer, Mode, Term};

fn main() {
    let (stdout, stderr) = (io::stdout(), io::stderr());
    let mut term = Term::with(stdout.lock(), stderr.lock());
    let mut buf = Vec::new();

    term.set_mode(Mode::Raw).unwrap();
    term.cursor()
        .save()
        .screen()
        .set_buffer(Buffer::Alternative)
        .clear()
        .cursor()
        .to((8, 8))
        .flush()
        .unwrap();

    let result = term
        .stdin_mut()
        .read_timeout_until(b' ', &mut buf, Duration::from_secs(5));

    term.screen()
        .clear()
        .set_buffer(Buffer::Canonical)
        .cursor()
        .restore()
        .flush()
        .unwrap();

    println!("{:?}", &buf[..result.unwrap_or(0)]);
}
