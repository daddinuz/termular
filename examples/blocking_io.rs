use std::io::{self, BufRead};
use term::screen::Buffer;
use term::Term;

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

        term.stdin_mut().read_line(&mut line)?;
        println!("# {}", line);

        line.clear();
    }
}
