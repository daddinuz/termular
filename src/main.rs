use std::{io, time::Duration};
use term::printer::{Color, FontWeight, Style};
use term::screen::Buffer;
use term::{Mode, Term};

fn main() -> io::Result<()> {
    let (stdout, stderr) = (io::stdout(), io::stderr());
    let mut term = Term::init(stdout.lock(), stderr.lock())?;
    let center = term.size()? / 2;

    term.set_mode(Mode::Raw)?;
    term.screen()
        .set_buffer(Buffer::Alternate)
        .clear()
        .cursor()
        .hide()
        .set_position(center - [5, 1])
        .printer()
        .using(Style::from(FontWeight::Bold).with_foreground(Color::Green))
        .print("Hello world")
        .cursor()
        .set_position(center - [5, 0])
        .printer()
        .print("- <SPACE> -")
        .flush()?;

    let _ = term
        .stdin_mut()
        .read_timeout_until(b' ', &mut Vec::new(), Duration::from_secs(5));

    Ok(())
}
