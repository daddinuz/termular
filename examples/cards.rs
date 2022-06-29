use std::{io, time::Duration};
use term::nio::ReadNonblock;
use term::printer::{Color, Style};
use term::screen::Buffer;
use term::{Mode, Term};

fn main() -> io::Result<()> {
    let (stdout, stderr) = (io::stdout(), io::stderr());
    let mut term = Term::init(stdout.lock(), stderr.lock())?;
    let center = term.size()? / 2;
    let start = center - [16, 8];

    term.set_mode(Mode::Raw)?;
    let mut op = term
        .screen()
        .set_buffer(Buffer::Alternate)
        .clear()
        .cursor()
        .hide()
        .set_position(start)
        .printer();

    let seeds = [0x1F0A1_u32, 0x1F0B1_u32, 0x1F0C1_u32, 0x1F0D1_u32];
    for y in 0..4 {
        for x in 0..15 {
            op = op.print(format!(
                "{} ",
                char::from_u32(seeds[y as usize] + x as u32).unwrap()
            ))
        }
        op = op.cursor().set_position(start + [0, y + 1]).printer();
    }

    op.flush()?;
    term.stdin_mut()
        .read_timeout_until(b' ', &mut Vec::new(), Duration::from_secs(30))
        .map(|_| ())
}
