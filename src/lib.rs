pub mod cursor;
pub mod screen;
pub mod vector;

use crate::cursor::Cursor;
use crate::screen::Screen;
use std::io::Write;

pub struct Term<W: Write>(W);

impl<W: Write> Term<W> {
    #[must_use]
    pub fn new(w: W) -> Self {
        Self(w)
    }

    #[must_use]
    pub fn screen(&mut self) -> Screen<'_, W> {
        Screen(Ok(&mut self.0))
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
