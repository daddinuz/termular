use crate::cursor::Cursor;
use crate::nio::{self, Stdin};
use crate::printer::Printer;
use crate::screen::{Buffer, Screen};
use crate::stream::Stream;
use crate::term;
use crate::vector::Vector2;

use std::io::{self, StderrLock, StdoutLock, Write};
use std::mem::MaybeUninit;
use std::os::unix::io::{AsRawFd, RawFd};
use std::sync::Once;

use libc::{ioctl, winsize, TIOCGWINSZ};
use termios::{cfmakeraw, tcsetattr, Termios, TCSANOW};

static mut DEFAULT_STATE: MaybeUninit<State> = MaybeUninit::uninit();
static INIT: Once = Once::new();

fn default_state() -> io::Result<State> {
    if !INIT.is_completed() {
        // If we are here does not mean `INIT` has not been executed,
        // `is_completed` may have returned a stale value.
        // If that's the case, someone else may have initialized `DEFAULT_STATE`
        // to the proper value and we must preserve it, therefore if `term::state()`
        // returns an Ok, it will be discarded because the call to `call_once_force`
        // won't run (since someone else alredy initialized `DEFAULT_STATE`).
        // If otherwise `term::state()` returns an Err, the latter will be returned,
        // even if `DEFAULT_STATE` has been properly initialized by someone else.
        // This is not good actually, anyway subsequent calls to this function
        // will return the right value, by now this function is best-effort.
        // If instead no one else initialized `DEFAULT_STATE` and `term::state()`
        // returned successfully we are the one that are going to initialize `DEFAULT_STATE`.
        let state = term::state()?;
        INIT.call_once_force(|_| {
            unsafe { DEFAULT_STATE.write(state) };
        })
    }

    Ok(unsafe { DEFAULT_STATE.assume_init() })
}

pub struct Term<'a> {
    stdin: Stdin,
    stdout: StdoutLock<'a>,
    stderr: StderrLock<'a>,
}

impl<'a> Term<'a> {
    pub fn open(stdout: StdoutLock<'a>, stderr: StderrLock<'a>) -> io::Result<Self> {
        term::setup(stdout, stderr)
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
    pub fn stream(&mut self) -> Stream<'a, '_> {
        Stream(Ok(self))
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

    pub fn size(&self) -> io::Result<Vector2<u16>> {
        term::size()
    }

    pub fn set_mode(&mut self, mode: Mode) -> io::Result<()> {
        term::set_mode(mode)
    }
}

impl<'a> Drop for Term<'a> {
    fn drop(&mut self) {
        best_effort(term::set_mode(Mode::Raw));

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

        best_effort(term::set_mode(Mode::Default));
    }
}

pub fn size() -> io::Result<Vector2<u16>> {
    let mut win = winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };

    if unsafe { ioctl(stdin_fileno(), TIOCGWINSZ, &mut win as *mut _) } == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok([win.ws_col, win.ws_row].into())
    }
}

// From: (https://en.wikipedia.org/wiki/Terminal_mode)
#[derive(Default, Debug, PartialEq, Eq, Copy, Clone)]
pub enum Mode {
    #[default]
    Default,
    Raw,
}

pub fn with_mode<T, F>(mode: Mode, f: F) -> io::Result<T>
where
    F: FnOnce() -> T,
{
    let state = term::state()?;
    term::set_mode(mode)?;
    let out = f();
    term::apply(&state)?;
    Ok(out)
}

// This function is not exported directly but as a `Term` method so
// that we ensure to restore `Mode::Default` when `Term` gets droppped.
fn set_mode(mode: Mode) -> io::Result<()> {
    // If we are here `DEFAULT_STATE` must have been initialized.
    debug_assert!(INIT.is_completed());

    let state = default_state().unwrap();
    match mode {
        Mode::Default => term::apply(&state),
        Mode::Raw => term::apply(&state.make_raw()),
    }
}

fn setup<'a>(mut stdout: StdoutLock<'a>, stderr: StderrLock<'a>) -> io::Result<Term<'a>> {
    // trigger DEFAULT_STATE initialization!!!
    default_state()?;
    debug_assert!(INIT.is_completed());

    // flush pending outputs
    stdout.flush()?;

    Ok(Term {
        stdin: nio::stdin(),
        stdout,
        stderr,
    })
}

#[derive(Copy, Clone)]
struct State(Termios);

impl State {
    #[must_use]
    fn make_raw(self) -> Self {
        let Self(mut inner) = self;
        cfmakeraw(&mut inner);
        Self(inner)
    }
}

fn state() -> io::Result<State> {
    Termios::from_fd(stdin_fileno()).map(State)
}

fn apply(state: &State) -> io::Result<()> {
    tcsetattr(stdin_fileno(), TCSANOW, &state.0)
}

#[must_use]
fn stdin_fileno() -> RawFd {
    let stdin = io::stdin();
    stdin.as_raw_fd()
}

fn best_effort<T, E>(_: Result<T, E>) {}
