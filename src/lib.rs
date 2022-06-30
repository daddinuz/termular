#![feature(deadline_api)]

pub mod cursor;
pub mod nio;
pub mod printer;
pub mod screen;
pub mod vector;

pub(crate) mod state;

use crate::cursor::Cursor;
use crate::nio::Stdin;
use crate::printer::Printer;
use crate::screen::{Buffer, Screen};
use crate::state::State;
use crate::vector::Vector2;

use std::io::{self, StderrLock, StdoutLock, Write};

// From: (https://en.wikipedia.org/wiki/Terminal_mode)
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Mode {
    Cooked,
    Raw,
}

pub struct Term<'a> {
    state: State,
    stdin: Stdin,
    stdout: StdoutLock<'a>,
    stderr: StderrLock<'a>,
}

impl<'a> Term<'a> {
    pub fn open(mut stdout: StdoutLock<'a>, stderr: StderrLock<'a>) -> io::Result<Self> {
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

    pub fn stdin(&self) -> &Stdin {
        &self.stdin
    }

    pub fn stdin_mut(&mut self) -> &mut Stdin {
        &mut self.stdin
    }

    pub fn stdout(&self) -> &StdoutLock<'a> {
        &self.stdout
    }

    pub fn stdout_mut(&mut self) -> &mut StdoutLock<'a> {
        &mut self.stdout
    }

    pub fn stderr(&self) -> &StderrLock<'a> {
        &self.stderr
    }

    pub fn stderr_mut(&mut self) -> &mut StderrLock<'a> {
        &mut self.stderr
    }

    pub fn set_mode(&mut self, mode: Mode) -> io::Result<()> {
        match mode {
            Mode::Cooked => self.state.apply(),
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
                .restore()
                .screen()
                .clear()
                .set_buffer(Buffer::Primary)
                .flush(),
        );
        best_effort(self.set_mode(Mode::Cooked));
    }
}

fn best_effort<T, E>(_: Result<T, E>) {}
