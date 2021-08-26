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

impl Default for TextDecoration {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FontWeight {
    Normal,
    Light,
    Bold,
}

impl Default for FontWeight {
    fn default() -> Self {
        Self::Normal
    }
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

impl Default for Color {
    fn default() -> Self {
        Self::Default
    }
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
    pub fn print(self, s: impl AsRef<str>) -> Printer<'a, 'b> {
        let style = self.style;
        Printer(self.term.and_then(|t| {
            write!(
                t.stdout_mut(),
                "{}{}{}{}{}{}",
                foreground_fmt(style.foreground()),
                background_fmt(style.background()),
                decoration_fmt(style.decoration()),
                weight_fmt(style.weight()),
                s.as_ref(),
                reset_fmt()
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
    pub fn set_weight(self, weight: FontWeight) -> Self {
        self.chain(|t| write!(t.stdout_mut(), "{}", weight_fmt(weight)))
    }

    #[must_use]
    pub fn set_decoration(self, decoration: TextDecoration) -> Self {
        self.chain(|t| write!(t.stdout_mut(), "{}", decoration_fmt(decoration)))
    }

    #[must_use]
    pub fn set_foreground(self, color: Color) -> Self {
        self.chain(|t| write!(t.stdout_mut(), "{}", foreground_fmt(color)))
    }

    #[must_use]
    pub fn set_background(self, color: Color) -> Self {
        self.chain(|t| write!(t.stdout_mut(), "{}", background_fmt(color)))
    }

    #[must_use]
    pub fn reset(self) -> Self {
        self.chain(|t| write!(t.stdout_mut(), "{}", reset_fmt()))
    }

    #[must_use]
    pub fn print(self, s: impl AsRef<str>) -> Self {
        self.chain(|t| write!(t.stdout_mut(), "{}", s.as_ref()))
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
fn weight_fmt(weight: FontWeight) -> &'static str {
    match weight {
        FontWeight::Normal => "\x1B[22m",
        FontWeight::Light => "\x1B[2m",
        FontWeight::Bold => "\x1B[1m",
    }
}

#[must_use]
fn decoration_fmt(decoration: TextDecoration) -> &'static str {
    match decoration {
        TextDecoration::None => "\x1B[29;24m",
        TextDecoration::Strike => "\x1B[9m",
        TextDecoration::Underline => "\x1B[4m",
    }
}

#[must_use]
fn foreground_fmt(color: Color) -> &'static str {
    match color {
        Color::Default => "\x1B[39m",
        Color::Black => "\x1B[30m",
        Color::Red => "\x1B[31m",
        Color::Green => "\x1B[32m",
        Color::Yellow => "\x1B[33m",
        Color::Blue => "\x1B[34m",
        Color::Magenta => "\x1B[35m",
        Color::Cyan => "\x1B[36m",
        Color::White => "\x1B[37m",
    }
}

#[must_use]
fn background_fmt(color: Color) -> &'static str {
    match color {
        Color::Default => "\x1B[49m",
        Color::Black => "\x1B[40m",
        Color::Red => "\x1B[41m",
        Color::Green => "\x1B[42m",
        Color::Yellow => "\x1B[43m",
        Color::Blue => "\x1B[44m",
        Color::Magenta => "\x1B[45m",
        Color::Cyan => "\x1B[46m",
        Color::White => "\x1B[47m",
    }
}

#[must_use]
fn reset_fmt() -> &'static str {
    "\x1B[22;29;24;39;49m"
}
