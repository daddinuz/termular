use std::{io, time::Duration};
use term::printer::{Color, FontWeight};
use term::screen::Buffer;
use term::{Mode, Term};

fn main() {
    let (stdout, stderr) = (io::stdout(), io::stderr());
    let mut term = Term::with(stdout.lock(), stderr.lock());
    let size = term.size().unwrap();

    term.set_mode(Mode::Raw).unwrap();
    term.screen()
        .set_buffer(Buffer::Alternative)
        .clear()
        .cursor()
        .hide()
        .set_position((size / 2) - (5, 1).into())
        .printer()
        .set_weight(FontWeight::Bold)
        .set_foreground(Color::Green)
        .print("Hello world")
        .cursor()
        .set_position((size / 2) - (5, 0).into())
        .printer()
        .reset()
        .print("- <SPACE> -")
        .flush()
        .unwrap();

    term.stdin_mut()
        .read_timeout_until(b' ', &mut Vec::new(), Duration::from_secs(5))
        .unwrap();

    // flush buffers, release streams locks and restore terminal state.
    drop(term);
}
