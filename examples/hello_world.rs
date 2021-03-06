use std::{io, time::Duration};
use termular::nio::ReadNonblock;
use termular::printer::{Color, FontWeight, Styled};
use termular::screen::Buffer;
use termular::{Mode, Term};

fn main() -> io::Result<()> {
    let (stdout, stderr) = (io::stdout(), io::stderr());

    let mut term = Term::open(stdout.lock(), stderr.lock())?;
    let center = term.size()? / 2;

    term.flow()
        .set_mode(Mode::Raw)
        .screen()
        .set_buffer(Buffer::Alternate)
        .clear()
        .cursor()
        .hide()
        .set_position(center - [5, 4])
        .printer()
        .print(
            "Hello world"
                .with_foreground(Color::Green)
                .with_weight(FontWeight::Bold),
        )
        .cursor()
        .set_position(center - [2, 2])
        .printer()
        .print("press")
        .cursor()
        .set_position(center - [5, 1])
        .printer()
        .print("- <SPACE> -")
        .cursor()
        .set_position(center - [3, 0])
        .printer()
        .print("to exit")
        .flush()?;

    term.stdin_mut()
        .read_timeout_until(b' ', &mut Vec::new(), Duration::from_secs(30))
        .map(|_| ())
}
