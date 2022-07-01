use crate::cursor::Cursor;
use crate::screen::Screen;
use crate::stream::Stream;
use crate::Term;

use std::fmt::{Debug, Display};
use std::io::{self, Write};

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub enum TextDecoration {
    #[default]
    None,
    Strike,
    Underline,
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub enum FontWeight {
    #[default]
    Normal,
    Light,
    Bold,
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub enum Color {
    #[default]
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

#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub struct Style {
    foreground: Color,
    background: Color,
    decoration: TextDecoration,
    weight: FontWeight,
}

impl From<TextDecoration> for Style {
    fn from(decoration: TextDecoration) -> Self {
        Self {
            decoration,
            ..Default::default()
        }
    }
}

impl From<FontWeight> for Style {
    fn from(weight: FontWeight) -> Self {
        Self {
            weight,
            ..Default::default()
        }
    }
}

impl Style {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn with_decoration(mut self, decoration: TextDecoration) -> Self {
        self.decoration = decoration;
        self
    }

    #[must_use]
    pub fn decoration(&self) -> TextDecoration {
        self.decoration
    }

    #[must_use]
    pub fn with_weight(mut self, weight: FontWeight) -> Self {
        self.weight = weight;
        self
    }

    #[must_use]
    pub fn weight(&self) -> FontWeight {
        self.weight
    }

    #[must_use]
    pub fn with_foreground(mut self, color: Color) -> Self {
        self.foreground = color;
        self
    }

    #[must_use]
    pub fn foreground(&self) -> Color {
        self.foreground
    }

    #[must_use]
    pub fn with_background(mut self, color: Color) -> Self {
        self.background = color;
        self
    }

    #[must_use]
    pub fn background(&self) -> Color {
        self.background
    }
}

pub struct StyledPrinter<'a: 'b, 'b> {
    pub(crate) style: Style,
    pub(crate) term: io::Result<&'b mut Term<'a>>,
}

impl<'a, 'b> StyledPrinter<'a, 'b> {
    #[must_use]
    pub fn print(self, any: impl Display) -> Printer<'a, 'b> {
        let style = self.style;
        Printer(self.term.and_then(|t| {
            write!(
                t.stdout_mut(),
                "\x1B[{};{};{};{}m{}\x1B[{}m",
                fmt_foreground(style.foreground()),
                fmt_background(style.background()),
                fmt_decoration(style.decoration()),
                fmt_weight(style.weight()),
                any,
                fmt_restore()
            )
            .map(|_| t)
        }))
    }

    #[must_use]
    pub fn debug(self, any: impl Debug) -> Printer<'a, 'b> {
        let style = self.style;
        Printer(self.term.and_then(|t| {
            write!(
                t.stdout_mut(),
                "\x1B[{};{};{};{}m{:?}\x1B[{}m",
                fmt_foreground(style.foreground()),
                fmt_background(style.background()),
                fmt_decoration(style.decoration()),
                fmt_weight(style.weight()),
                any,
                fmt_restore()
            )
            .map(|_| t)
        }))
    }
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
    pub fn stream(self) -> Stream<'a, 'b> {
        Stream(self.0)
    }

    #[must_use]
    pub fn set_weight(self, weight: FontWeight) -> Self {
        self.chain(|t| write!(t.stdout_mut(), "\x1B[{}m", fmt_weight(weight)))
    }

    #[must_use]
    pub fn set_decoration(self, decoration: TextDecoration) -> Self {
        self.chain(|t| write!(t.stdout_mut(), "\x1B[{}m", fmt_decoration(decoration)))
    }

    #[must_use]
    pub fn set_foreground(self, color: Color) -> Self {
        self.chain(|t| write!(t.stdout_mut(), "\x1B[{}m", fmt_foreground(color)))
    }

    #[must_use]
    pub fn set_background(self, color: Color) -> Self {
        self.chain(|t| write!(t.stdout_mut(), "\x1B[{}m", fmt_background(color)))
    }

    #[must_use]
    pub fn restore(self) -> Self {
        self.chain(|t| write!(t.stdout_mut(), "\x1B[{}m", fmt_restore()))
    }

    #[must_use]
    pub fn print(self, any: impl Display) -> Self {
        self.chain(|t| write!(t.stdout_mut(), "{}", any))
    }

    #[must_use]
    pub fn debug(self, any: impl Debug) -> Self {
        self.chain(|t| write!(t.stdout_mut(), "{:?}", any))
    }

    #[must_use]
    pub fn using(self, style: Style) -> StyledPrinter<'a, 'b> {
        StyledPrinter {
            style,
            term: self.0,
        }
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

#[must_use]
fn fmt_weight(weight: FontWeight) -> &'static str {
    match weight {
        FontWeight::Normal => "22",
        FontWeight::Light => "2",
        FontWeight::Bold => "1",
    }
}

#[must_use]
fn fmt_decoration(decoration: TextDecoration) -> &'static str {
    match decoration {
        TextDecoration::None => "29;24",
        TextDecoration::Strike => "9",
        TextDecoration::Underline => "4",
    }
}

#[must_use]
fn fmt_foreground(color: Color) -> &'static str {
    match color {
        Color::Default => "39",
        Color::Black => "30",
        Color::Red => "31",
        Color::Green => "32",
        Color::Yellow => "33",
        Color::Blue => "34",
        Color::Magenta => "35",
        Color::Cyan => "36",
        Color::White => "37",
    }
}

#[must_use]
fn fmt_background(color: Color) -> &'static str {
    match color {
        Color::Default => "49",
        Color::Black => "40",
        Color::Red => "41",
        Color::Green => "42",
        Color::Yellow => "43",
        Color::Blue => "44",
        Color::Magenta => "45",
        Color::Cyan => "46",
        Color::White => "47",
    }
}

#[must_use]
fn fmt_restore() -> &'static str {
    "22;29;24;39;49"
}
