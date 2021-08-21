use crate::cursor::Cursor;
use crate::screen::Screen;
use crate::Term;
use std::io::{self, Write};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TextDecoration {
    None,
    Strike,
    Underline,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FontWeight {
    Normal,
    Light,
    Bold,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Color {
    Default,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

pub struct Printer<'a: 'b, 'b>(pub(crate) io::Result<&'b mut Term<'a>>);

impl<'a, 'b> Printer<'a, 'b> {
    #[must_use]
    pub fn cursor(self) -> Cursor<'a, 'b> {
        Cursor(self.0)
    }

    #[must_use]
    pub fn screen(self) -> Screen<'a, 'b> {
        Screen(self.0)
    }

    #[must_use]
    pub fn set_weight(self, weight: FontWeight) -> Self {
        match weight {
            FontWeight::Normal => self.chain(|t| write!(t.stdout_mut(), "\x1B[22m")),
            FontWeight::Light => self.chain(|t| write!(t.stdout_mut(), "\x1B[2m")),
            FontWeight::Bold => self.chain(|t| write!(t.stdout_mut(), "\x1B[1m")),
        }
    }

    #[must_use]
    pub fn set_decoration(self, decoration: TextDecoration) -> Self {
        match decoration {
            TextDecoration::None => self.chain(|t| write!(t.stdout_mut(), "\x1B[29;24m")),
            TextDecoration::Strike => self.chain(|t| write!(t.stdout_mut(), "\x1B[9m")),
            TextDecoration::Underline => self.chain(|t| write!(t.stdout_mut(), "\x1B[4m")),
        }
    }

    #[must_use]
    pub fn set_foreground(self, color: Color) -> Self {
        match color {
            Color::Default => self.chain(|t| write!(t.stdout_mut(), "\x1B[39m")),
            Color::Black => self.chain(|t| write!(t.stdout_mut(), "\x1B[30m")),
            Color::Red => self.chain(|t| write!(t.stdout_mut(), "\x1B[31m")),
            Color::Green => self.chain(|t| write!(t.stdout_mut(), "\x1B[32m")),
            Color::Yellow => self.chain(|t| write!(t.stdout_mut(), "\x1B[33m")),
            Color::Blue => self.chain(|t| write!(t.stdout_mut(), "\x1B[34m")),
            Color::Magenta => self.chain(|t| write!(t.stdout_mut(), "\x1B[35m")),
            Color::Cyan => self.chain(|t| write!(t.stdout_mut(), "\x1B[36m")),
            Color::White => self.chain(|t| write!(t.stdout_mut(), "\x1B[37m")),
        }
    }

    #[must_use]
    pub fn set_background(self, color: Color) -> Self {
        match color {
            Color::Default => self.chain(|t| write!(t.stdout_mut(), "\x1B[49m")),
            Color::Black => self.chain(|t| write!(t.stdout_mut(), "\x1B[40m")),
            Color::Red => self.chain(|t| write!(t.stdout_mut(), "\x1B[41m")),
            Color::Green => self.chain(|t| write!(t.stdout_mut(), "\x1B[42m")),
            Color::Yellow => self.chain(|t| write!(t.stdout_mut(), "\x1B[43m")),
            Color::Blue => self.chain(|t| write!(t.stdout_mut(), "\x1B[44m")),
            Color::Magenta => self.chain(|t| write!(t.stdout_mut(), "\x1B[45m")),
            Color::Cyan => self.chain(|t| write!(t.stdout_mut(), "\x1B[46m")),
            Color::White => self.chain(|t| write!(t.stdout_mut(), "\x1B[47m")),
        }
    }

    #[must_use]
    pub fn reset(self) -> Self {
        self.chain(|t| write!(t.stdout_mut(), "\x1B[22;29;24;39;49m"))
    }

    #[must_use]
    pub fn print(self, s: impl AsRef<str>) -> Self {
        self.chain(|t| write!(t.stdout_mut(), "{}", s.as_ref()))
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
