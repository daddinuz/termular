use std::{io, time::Duration};
use term::nio::ReadNonblock;
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
        .set_position(center - [7, 5])
        .printer()
        .using(Style::from(FontWeight::Bold).with_foreground(Color::Green))
        .print("Type Something")
        .cursor()
        .set_position(center - [3, 2])
        .printer()
        .print("<CANC>")
        .cursor()
        .set_position(center - [1, 1])
        .printer()
        .print("to")
        .cursor()
        .set_position(center - [2, 0])
        .printer()
        .print("exit")
        .flush()?;

    let mut buf = [0; 4];
    while buf != [27, 91, 51, 126] {
        buf.fill(0);

        if term
            .stdin_mut()
            .read_timeout(&mut buf, Duration::from_millis(128))
            .is_ok()
        {
            term.cursor()
                .set_position(center - [6, 4])
                .erase_line()
                .printer()
                .debug(&buf)
                .flush()
                .unwrap();
        }
    }

    Ok(())
}
