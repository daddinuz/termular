use crate::screen::Screen;
use crate::vector::Vector2;
use crate::Term;
use std::io::{self, Write};

pub struct Cursor<'a: 'b, 'b>(pub(crate) io::Result<&'b mut Term<'a>>);

impl<'a, 'b> Cursor<'a, 'b> {
    #[must_use]
    pub fn screen(self) -> Screen<'a, 'b> {
        Screen(self.0)
    }

    #[must_use]
    pub fn hide(self) -> Self {
        self.chain(|t| write!(t.stdout_mut(), "\x1B[?25l"))
    }

    #[must_use]
    pub fn show(self) -> Self {
        self.chain(|t| write!(t.stdout_mut(), "\x1B[?25h"))
    }

    #[must_use]
    pub fn save(self) -> Self {
        self.chain(|t| write!(t.stdout_mut(), "\x1B[s"))
    }

    #[must_use]
    pub fn restore(self) -> Self {
        self.chain(|t| write!(t.stdout_mut(), "\x1B[u"))
    }

    #[must_use]
    pub fn to(self, pos: impl Into<Vector2<u16>>) -> Self {
        let pos = pos.into() + (1, 1).into();
        self.chain(|t| write!(t.stdout_mut(), "\x1B[{};{}H", pos.y(), pos.x()))
    }

    #[must_use]
    pub fn up(self, n: u16) -> Self {
        self.chain(|t| write!(t.stdout_mut(), "\x1B[{}A", n))
    }

    #[must_use]
    pub fn down(self, n: u16) -> Self {
        self.chain(|t| write!(t.stdout_mut(), "\x1B[{}B", n))
    }

    #[must_use]
    pub fn right(self, n: u16) -> Self {
        self.chain(|t| write!(t.stdout_mut(), "\x1B[{}C", n))
    }

    #[must_use]
    pub fn left(self, n: u16) -> Self {
        self.chain(|t| write!(t.stdout_mut(), "\x1B[{}D", n))
    }

    pub fn flush(self) -> io::Result<()> {
        self.0.and_then(|t| t.stdout_mut().flush())
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
