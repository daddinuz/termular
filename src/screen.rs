use crate::cursor::Cursor;
use std::io::{self, Write};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Buffer {
    Canonical,
    Alternative,
}

#[derive(Debug)]
pub struct Screen<'a, W: Write>(pub(crate) io::Result<&'a mut W>);

impl<'a, W: Write> Screen<'a, W> {
    #[must_use]
    pub fn clear(self) -> Self {
        Self(self.0.and_then(|w| w.write_all(b"\x1B[2J").map(|_| w)))
    }

    #[must_use]
    pub fn set_buffer(self, buffer: Buffer) -> Self {
        Self(self.0.and_then(|w| {
            match buffer {
                Buffer::Canonical => w.write_all(b"\x1B[?1049l"),
                Buffer::Alternative => w.write_all(b"\x1B[?1049h"),
            }
            .map(|_| w)
        }))
    }

    #[must_use]
    pub fn cursor(self) -> Cursor<'a, W> {
        Cursor(self.0)
    }

    pub fn flush(self) -> io::Result<()> {
        self.0.and_then(|w| w.flush())
    }
}
