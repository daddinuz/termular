use std::{io, time::Duration};
use term::nio::ReadNonblock;
use term::screen::Buffer;
use term::{Mode, Term};

fn main() -> io::Result<()> {
    let (stdout, stderr) = (io::stdout(), io::stderr());

    let mut term = Term::open(stdout.lock(), stderr.lock())?;
    let center = term.size()? / 2;
    let start = center - [16, 8];

    term.set_mode(Mode::Raw)?;
    let mut printer = term
        .screen()
        .set_buffer(Buffer::Alternate)
        .clear()
        .cursor()
        .hide()
        .set_position(start)
        .printer();

    let seeds = [0x1F0A1, 0x1F0B1, 0x1F0C1, 0x1F0D1];
    for y in 0..4 {
        for x in 0..15 {
            printer = printer.print(format!(
                "{} ",
                char::from_u32(seeds[y as usize] + x as u32).unwrap()
            ))
        }
        printer = printer.cursor().set_position(start + [0, y + 1]).printer();
    }

    printer.flush()?;
    term.stdin_mut()
        .read_timeout_until(b' ', &mut Vec::new(), Duration::from_secs(30))
        .map(|_| ())
}
