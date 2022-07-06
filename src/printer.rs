use crate::cursor::Cursor;
use crate::flow::Flow;
use crate::screen::Screen;
use crate::Term;

use std::fmt::{self, Debug, Display, Formatter};
use std::io::{self, Write};

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
    pub fn flow(self) -> Flow<'a, 'b> {
        Flow(self.0)
    }

    #[must_use]
    pub fn print(self, s: impl Display) -> Self {
        self.chain(|t| write!(t.stdout_mut(), "{}", s))
    }

    #[must_use]
    pub fn debug(self, s: impl Debug) -> Self {
        self.chain(|t| write!(t.stdout_mut(), "{:?}", s))
    }

    #[must_use]
    pub fn restore(self) -> Self {
        self.chain(|t| write!(t.stdout_mut(), "\x1B[{}m", fmt_restore()))
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

#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub struct Style {
    pub foreground: Color,
    pub background: Color,
    pub decoration: TextDecoration,
    pub weight: FontWeight,
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
    pub fn foreground(color: Color) -> Self {
        Style {
            foreground: color,
            ..Default::default()
        }
    }

    #[must_use]
    pub fn background(color: Color) -> Self {
        Style {
            foreground: color,
            ..Default::default()
        }
    }

    #[must_use]
    pub fn with_foreground(mut self, color: Color) -> Self {
        self.foreground = color;
        self
    }

    #[must_use]
    pub fn with_background(mut self, color: Color) -> Self {
        self.background = color;
        self
    }

    #[must_use]
    pub fn with_decoration(mut self, decoration: TextDecoration) -> Self {
        self.decoration = decoration;
        self
    }

    #[must_use]
    pub fn with_weight(mut self, weight: FontWeight) -> Self {
        self.weight = weight;
        self
    }
}

pub struct StyledContent<T> {
    pub style: Style,
    pub content: T,
}

impl<T> StyledContent<T> {
    #[must_use]
    pub fn with_foreground(mut self, color: Color) -> Self {
        self.style.foreground = color;
        self
    }

    #[must_use]
    pub fn with_background(mut self, color: Color) -> Self {
        self.style.background = color;
        self
    }

    #[must_use]
    pub fn with_decoration(mut self, decoration: TextDecoration) -> Self {
        self.style.decoration = decoration;
        self
    }

    #[must_use]
    pub fn with_weight(mut self, weight: FontWeight) -> Self {
        self.style.weight = weight;
        self
    }
}

impl<T> Debug for StyledContent<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let Self { style, content } = self;

        write!(
            f,
            "\x1B[{};{};{};{}m{:?}\x1B[{}m",
            fmt_foreground(style.foreground),
            fmt_background(style.background),
            fmt_decoration(style.decoration),
            fmt_weight(style.weight),
            content,
            fmt_restore()
        )
    }
}

impl<T> Display for StyledContent<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let Self { style, content } = self;

        write!(
            f,
            "\x1B[{};{};{};{}m{}\x1B[{}m",
            fmt_foreground(style.foreground),
            fmt_background(style.background),
            fmt_decoration(style.decoration),
            fmt_weight(style.weight),
            content,
            fmt_restore()
        )
    }
}

pub trait Styled: Sized {
    #[must_use]
    fn with_foreground(self, color: Color) -> StyledContent<Self> {
        StyledContent {
            style: Style::foreground(color),
            content: self,
        }
    }

    #[must_use]
    fn with_background(self, color: Color) -> StyledContent<Self> {
        StyledContent {
            style: Style::background(color),
            content: self,
        }
    }

    #[must_use]
    fn with_decoration(self, decoration: TextDecoration) -> StyledContent<Self> {
        StyledContent {
            style: Style::from(decoration),
            content: self,
        }
    }

    #[must_use]
    fn with_weight(self, weight: FontWeight) -> StyledContent<Self> {
        StyledContent {
            style: Style::from(weight),
            content: self,
        }
    }

    #[must_use]
    fn with_style(self, style: Style) -> StyledContent<Self> {
        StyledContent {
            style,
            content: self,
        }
    }
}

impl Styled for String {}

impl Styled for &mut str {}

impl Styled for &str {}

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
