use std::io::{self, Read};
use term::Term;

fn main() -> io::Result<()> {
    let (stdout, stderr) = (io::stdout(), io::stderr());
    let mut term = Term::init(stdout.lock(), stderr.lock())?;
    loop {
        let mut buf = [0; 16];
        let len = term.stdin_mut().read(&mut buf)?;
        println!("{:?}", &buf[..len]);
    }
}
