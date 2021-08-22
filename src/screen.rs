use crate::cursor::Cursor;
use crate::printer::Printer;
use crate::Term;
use std::io::{self, Write};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Buffer {
    Primary,
    Alternate,
}

pub struct Screen<'a: 'b, 'b>(pub(crate) io::Result<&'b mut Term<'a>>);

impl<'a, 'b> Screen<'a, 'b> {
    #[must_use]
    pub fn cursor(self) -> Cursor<'a, 'b> {
        Cursor(self.0)
    }

    #[must_use]
    pub fn printer(self) -> Printer<'a, 'b> {
        Printer(self.0)
    }

    #[must_use]
    pub fn clear(self) -> Self {
        self.chain(|t| write!(t.stdout_mut(), "\x1B[2J"))
    }

    #[must_use]
    pub fn set_buffer(self, buffer: Buffer) -> Self {
        self.chain(|t| match buffer {
            Buffer::Primary => write!(t.stdout_mut(), "\x1B[?1049l"),
            Buffer::Alternate => write!(t.stdout_mut(), "\x1B[?1049h"),
        })
    }

    pub fn flush(self) -> io::Result<()> {
        self.0?.stdout_mut().flush()
    }

    #[inline]
    #[must_use]
    fn chain<F>(self, f: F) -> Self
    where
        F: FnOnce(&mut Term) -> io::Result<()>,
    {
        Self(self.0.and_then(|t| f(t).map(|_| t)))
    }
}
