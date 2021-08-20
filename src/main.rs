use std::{io, str, time::Duration};
use term::{screen::Buffer, Mode, Term};

fn main() {
    let (stdout, stderr) = (io::stdout(), io::stderr());
    let mut term = Term::with(stdout.lock(), stderr.lock());

    let size = term.size().unwrap();
    term.set_mode(Mode::Raw).unwrap();
    term.cursor()
        .save()
        .screen()
        .set_buffer(Buffer::Alternative)
        .clear()
        .cursor()
        .set_position((8, 8))
        .flush()
        .unwrap();

    let mut buf = Vec::new();
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

    // flush buffers, release streams locks and restore term::Mode::Native.
    drop(term);

    println!("{:?}", size);
    println!("{:?}", str::from_utf8(&buf[..result.unwrap_or(0)]));
}
