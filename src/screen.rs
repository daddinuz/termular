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
        Self(self.0.and_then(|w| write!(w, "\x1B[2J").map(|_| w)))
    }

    #[must_use]
    pub fn set_buffer(self, buffer: Buffer) -> Self {
        Self(self.0.and_then(|w| {
            match buffer {
                Buffer::Canonical => write!(w, "\x1B[?1049h"),
                Buffer::Alternative => write!(w, "\x1B[?1049l"),
            }
            .map(|_| w)
        }))
    }

    pub fn flush(self) -> io::Result<()> {
        self.0.and_then(|w| w.flush())
    }
}