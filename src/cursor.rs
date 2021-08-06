use crate::screen::Screen;
use crate::vector::Vector2;
use std::io::{self, Write};

pub struct Cursor<'a, W: Write>(pub(crate) io::Result<&'a mut W>);

impl<'a, W: Write> Cursor<'a, W> {
    #[must_use]
    pub fn hide(self) -> Self {
        Self(self.0.and_then(|w| write!(w, "\x1B[?25l").map(|_| w)))
    }

    #[must_use]
    pub fn show(self) -> Self {
        Self(self.0.and_then(|w| write!(w, "\x1B[?25h").map(|_| w)))
    }

    #[must_use]
    pub fn save(self) -> Self {
        Self(self.0.and_then(|w| write!(w, "\x1B[s").map(|_| w)))
    }

    #[must_use]
    pub fn restore(self) -> Self {
        Self(self.0.and_then(|w| write!(w, "\x1B[u").map(|_| w)))
    }

    #[must_use]
    pub fn to(self, pos: impl Into<Vector2<u16>>) -> Self {
        let pos = pos.into();
        Self(
            self.0
                .and_then(|w| write!(w, "\x1B[{};{}H", pos.y() + 1, pos.x() + 1).map(|_| w)),
        )
    }

    #[must_use]
    pub fn up(self, n: u16) -> Self {
        Self(self.0.and_then(|w| write!(w, "\x1B[{}A", n).map(|_| w)))
    }

    #[must_use]
    pub fn down(self, n: u16) -> Self {
        Self(self.0.and_then(|w| write!(w, "\x1B[{}B", n).map(|_| w)))
    }

    #[must_use]
    pub fn right(self, n: u16) -> Self {
        Self(self.0.and_then(|w| write!(w, "\x1B[{}C", n).map(|_| w)))
    }

    #[must_use]
    pub fn left(self, n: u16) -> Self {
        Self(self.0.and_then(|w| write!(w, "\x1B[{}D", n).map(|_| w)))
    }

    #[must_use]
    pub fn screen(self) -> Screen<'a, W> {
        Screen(self.0)
    }

    pub fn flush(self) -> io::Result<()> {
        self.0.and_then(|w| w.flush())
    }
}
