use crate::printer::Printer;
use crate::screen::Screen;
use crate::vector::Vector2;
use crate::Term;
use std::io::{self, Write};
use std::time::Duration;

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
        let mut buf = Vec::new();

        term.stdout_mut().flush()?;
        term.stderr_mut().write_all(b"\x1B[6n")?;
        term.stdin_mut()
            .read_timeout_until(b'R', &mut buf, Duration::from_millis(512))?;

        match &buf[..] {
            [.., b'\x1B', b'[', row, b';', col, b'R'] => {
                let row = u16::from(row - b'1');
                let col = u16::from(col - b'1');
                Ok((col, row).into())
            }
            [.., b'\x1B', b'[', row10, row, b';', col10, col, b'R'] => {
                let row = (u16::from(row10 - b'0') * 10 + u16::from(row - b'0')) - 1;
                let col = (u16::from(col10 - b'0') * 10 + u16::from(col - b'0')) - 1;
                Ok((col, row).into())
            }
            _ => unreachable!("{:?}", buf),
        }
    }

    #[must_use]
    pub fn set_position(self, pos: impl Into<Vector2<u16>>) -> Self {
        let pos = pos.into() + (1, 1).into();
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
