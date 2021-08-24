use crate::printer::Printer;
use crate::screen::Screen;
use crate::vector::Vector2;
use crate::{mode, Mode, Term};
use std::io::{self, Write};
use std::time::Duration;
use std::{error, str};

pub struct Cursor<'a: 'b, 'b>(pub(crate) io::Result<&'b mut Term<'a>>);

impl<'a, 'b> Cursor<'a, 'b> {
    #[must_use]
    pub fn printer(self) -> Printer<'a, 'b> {
        Printer(self.0)
    }

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
        self.0?.stdout_mut().flush()
    }

    pub fn position(self) -> io::Result<Vector2<u16>> {
        let term = self.0?;

        term.stdout_mut().flush()?;
        mode::with(Mode::Raw, || {
            let mut buf = Vec::new();

            term.stderr_mut().write_all(b"\x1B[6n")?;
            term.stdin_mut()
                .read_timeout_until(b'R', &mut buf, Duration::from_secs(1))?;

            parse_position(&buf)
        })?
    }

    #[must_use]
    pub fn set_position(self, pos: impl Into<Vector2<u16>>) -> Self {
        let pos = pos.into() + [1, 1];
        self.chain(|t| write!(t.stdout_mut(), "\x1B[{};{}H", pos.y(), pos.x()))
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

fn parse_position(bytes: &[u8]) -> io::Result<Vector2<u16>> {
    let delimiter = bytes
        .iter()
        .rposition(|b| *b == b'R')
        .ok_or_else(|| make_err("Unable to retrieve position: missing token `R`"))?;

    let semicolon = bytes[..delimiter]
        .iter()
        .rposition(|b| *b == b';')
        .ok_or_else(|| make_err("Unable to retrieve position: missing token `;`"))?;

    let square_bracket = bytes[..semicolon]
        .iter()
        .rposition(|b| *b == b'[')
        .ok_or_else(|| make_err("Unable to retrieve position: missing token `[`"))?;

    let row = parse::<u16>(&bytes[square_bracket + 1..semicolon])?;
    let col = parse::<u16>(&bytes[semicolon + 1..delimiter])?;
    Ok([col - 1, row - 1].into())
}

fn parse<T>(bytes: &[u8]) -> io::Result<T>
where
    T: str::FromStr,
    <T as str::FromStr>::Err: Into<Box<dyn error::Error + Send + Sync>>,
{
    str::from_utf8(bytes)
        .map_err(make_err)
        .and_then(|s| s.parse().map_err(make_err))
}

#[inline]
#[must_use]
fn make_err<E>(err: E) -> io::Error
where
    E: Into<Box<dyn error::Error + Send + Sync>>,
{
    io::Error::new(io::ErrorKind::Other, err)
}
