#![feature(deadline_api)]

pub mod cursor;
mod mode;
pub mod nio;
pub mod printer;
pub mod screen;
pub mod vector;

use crate::cursor::Cursor;
pub use crate::mode::Mode;
use crate::nio::NonblockingStdin;
use crate::printer::Printer;
use crate::screen::{Buffer, Screen};
use crate::vector::Vector2;

use libc::{ioctl, winsize, TIOCGWINSZ};
use std::io::{self, StderrLock, StdoutLock, Write};
use std::os::unix::io::AsRawFd;

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
    pub fn cursor(&mut self) -> Cursor<'a, '_> {
        Cursor(Ok(self))
    }

    #[must_use]
    pub fn printer(&mut self) -> Printer<'a, '_> {
        Printer(Ok(self))
    }

    #[must_use]
    pub fn screen(&mut self) -> Screen<'a, '_> {
        Screen(Ok(self))
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

    pub fn size(&mut self) -> io::Result<Vector2<u16>> {
        let fd = self.stdout.as_raw_fd();
        let mut term = winsize {
            ws_row: 0,
            ws_col: 0,
            ws_xpixel: 0,
            ws_ypixel: 0,
        };

        if unsafe { ioctl(fd, TIOCGWINSZ, &mut term as *mut _) } == -1 {
            Err(io::Error::last_os_error())
        } else {
            Ok([term.ws_col, term.ws_row].into())
        }
    }
}

impl<'a> Drop for Term<'a> {
    fn drop(&mut self) {
        best_effort(self.set_mode(Mode::Raw));
        best_effort(self.screen().set_buffer(Buffer::Alternative).flush());

        // From: (https://en.wikipedia.org/wiki/ANSI_escape_code#Fs_Escape_sequences)
        // RIS Reset to Initial State
        best_effort(write!(self.stderr_mut(), "\x1Bc"));

        best_effort(self.screen().set_buffer(Buffer::Canonical).flush());
        best_effort(self.set_mode(Mode::Cooked));
    }
}

fn best_effort<T, E>(_: Result<T, E>) {}
