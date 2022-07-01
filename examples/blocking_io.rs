use std::io::{self, BufRead};
use termular::screen::Buffer;
use termular::Term;

fn main() -> io::Result<()> {
    let (stdout, stderr) = (io::stdout(), io::stderr());

    let mut term = Term::open(stdout.lock(), stderr.lock())?;
    let mut line = String::new();

    term.screen()
        .set_buffer(Buffer::Alternate)
        .clear()
        .flush()?;

    loop {
        term.printer().print("> ").flush()?;

        if 0 == term.stdin_mut().read_line(&mut line)? {
            return Ok(());
        }

        println!("# {}", line);
        line.clear();
    }
}
