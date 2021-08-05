mod cursor;
mod vector;

use crate::cursor::Cursor;
use std::io::{self, Write};

pub struct Term<W: Write>(W);

impl<W: Write> Term<W> {
    #[must_use]
    pub fn new(w: W) -> Self {
        Self(w)
    }

    pub fn alternative_screen(&mut self, enable: bool) -> io::Result<()> {
        match enable {
            true => write!(&mut self.0, "\x1B[?1049h"),
            false => write!(&mut self.0, "\x1B[?1049l"),
        }
    }

    #[must_use]
    pub fn cursor(&mut self) -> Cursor<'_, W> {
        Cursor(Ok(&mut self.0))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
