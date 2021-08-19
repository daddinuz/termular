#![feature(deadline_api)]

pub mod cursor;
mod mode;
pub mod nio;
pub mod screen;
pub mod vector;

use crate::cursor::Cursor;
pub use crate::mode::Mode;
use crate::nio::NonblockingStdin;
use crate::screen::Screen;
use crate::vector::Vector2;
use std::io::{self, StderrLock, StdoutLock, Write};
use std::time::Duration;

pub struct Term<'a> {
    stdin: NonblockingStdin,
    stdout: StdoutLock<'a>,
    stderr: StderrLock<'a>,
}

impl<'a> Term<'a> {
    #[must_use]
    pub fn with(mut stdout: StdoutLock<'a>, stderr: StderrLock<'a>) -> Self {
        best_effort(stdout.flush());
        let stdin = nio::stdin();
        Self {
            stdin,
            stdout,
            stderr,
        }
    }

    #[must_use]
    pub fn screen(&mut self) -> Screen<'_, StdoutLock<'a>> {
        Screen(Ok(&mut self.stdout))
    }

    #[must_use]
    pub fn cursor(&mut self) -> Cursor<'_, StdoutLock<'a>> {
        Cursor(Ok(&mut self.stdout))
    }

    #[must_use]
    pub fn stdin(&self) -> &NonblockingStdin {
        &self.stdin
    }

    #[must_use]
    pub fn stdin_mut(&mut self) -> &mut NonblockingStdin {
        &mut self.stdin
    }

    #[must_use]
    pub fn stdout(&self) -> &StdoutLock<'a> {
        &self.stdout
    }

    #[must_use]
    pub fn stdout_mut(&mut self) -> &mut StdoutLock<'a> {
        &mut self.stdout
    }

    #[must_use]
    pub fn stderr(&self) -> &StderrLock<'a> {
        &self.stderr
    }

    #[must_use]
    pub fn stderr_mut(&mut self) -> &mut StderrLock<'a> {
        &mut self.stderr
    }

    pub fn set_mode(&mut self, mode: Mode) -> io::Result<()> {
        mode::set(mode)
    }

    pub fn position(&mut self) -> io::Result<Vector2<u16>> {
        let mut buf = Vec::new();
        self.stderr.write_all(b"\x1B[6n")?;
        self.stdin
            .read_timeout_until(b'R', &mut buf, Duration::from_millis(512))?;
        if let [.., b'\x1B', b'[', row, b';', col, b'R'] = &buf[..] {
            let row = u16::from(row - b'1');
            let col = u16::from(col - b'1');
            Ok((col, row).into())
        } else {
            unreachable!();
        }
    }
}

impl<'a> Drop for Term<'a> {
    fn drop(&mut self) {
        best_effort(self.stdout.flush());
        best_effort(self.set_mode(Mode::Native));
    }
}

fn best_effort<T, E>(_: Result<T, E>) {}
