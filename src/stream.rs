use crate::cursor::Cursor;
use crate::printer::Printer;
use crate::screen::Screen;
use crate::Term;

use std::io::{self, Write};

pub struct Stream<'a: 'b, 'b>(pub(crate) io::Result<&'b mut Term<'a>>);

impl<'a, 'b> Stream<'a, 'b> {
    #[must_use]
    pub fn cursor(self) -> Cursor<'a, 'b> {
        Cursor(self.0)
    }

    #[must_use]
    pub fn printer(self) -> Printer<'a, 'b> {
        Printer(self.0)
    }

    #[must_use]
    pub fn screen(self) -> Screen<'a, 'b> {
        Screen(self.0)
    }

    pub fn flush(self) -> io::Result<()> {
        self.0?.stdout_mut().flush()
    }
}
