#![feature(deadline_api)]

pub mod cursor;
pub mod nio;
pub mod screen;
pub mod vector;

use crate::cursor::Cursor;
use crate::nio::NonblockingStdin;
use crate::screen::Screen;
use std::io::{StderrLock, StdoutLock, Write};

pub struct Term<'a> {
    stdin: NonblockingStdin,
    stdout: StdoutLock<'a>,
    stderr: StderrLock<'a>,
}

impl<'a> Term<'a> {
    #[must_use]
    pub fn with(mut stdout: StdoutLock<'a>, stderr: StderrLock<'a>) -> Self {
        let stdin = nio::stdin();
        flush(&mut stdout);
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
}

impl<'a> Drop for Term<'a> {
    fn drop(&mut self) {
        flush(&mut self.stdout);
    }
}

fn flush(stream: &mut dyn Write) {
    // best-effort
    let _ = stream.flush();
}
