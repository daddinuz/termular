#![feature(deadline_api)]

pub mod cursor;
pub mod nio;
pub mod printer;
pub mod screen;
pub mod vector;

pub(crate) mod state;

use crate::cursor::Cursor;
use crate::nio::StdinNonblock;
use crate::printer::Printer;
use crate::screen::{Buffer, Screen};
use crate::state::State;
use crate::vector::Vector2;

use std::io::{self, StderrLock, StdoutLock, Write};

// From: (https://en.wikipedia.org/wiki/Terminal_mode)
pub enum Mode {
    Original,
    Raw,
}

pub struct Term<'a> {
    state: State,
    stdin: StdinNonblock,
    stdout: StdoutLock<'a>,
    stderr: StderrLock<'a>,
}

impl<'a> Term<'a> {
    pub fn init(mut stdout: StdoutLock<'a>, stderr: StderrLock<'a>) -> io::Result<Self> {
        let state = State::capture()?;
        let stdin = nio::stdin();
        stdout.flush()?;

        Ok(Self {
            state,
            stdin,
            stdout,
            stderr,
        })
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
    pub fn stdin(&self) -> &StdinNonblock {
        &self.stdin
    }

    #[must_use]
    pub fn stdin_mut(&mut self) -> &mut StdinNonblock {
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
        match mode {
            Mode::Original => self.state.apply(),
            Mode::Raw => self.state.raw().apply(),
        }
    }

    pub fn size(&self) -> io::Result<Vector2<u16>> {
        state::window_size()
    }
}

impl<'a> Drop for Term<'a> {
    fn drop(&mut self) {
        best_effort(self.set_mode(Mode::Raw));
        best_effort(
            self.screen()
                .set_buffer(Buffer::Alternate)
                .cursor()
                .show()
                .printer()
                .reset()
                .screen()
                .clear()
                .set_buffer(Buffer::Primary)
                .flush(),
        );
        best_effort(self.set_mode(Mode::Original));
    }
}

fn best_effort<T, E>(_: Result<T, E>) {}
