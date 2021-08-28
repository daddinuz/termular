use crate::vector::Vector2;
use libc::{ioctl, winsize, TIOCGWINSZ};
use std::io;
use std::os::unix::io::{AsRawFd, RawFd};
use termios::{cfmakeraw, tcsetattr, Termios, TCSANOW};

pub struct State {
    inner: Termios,
}

impl State {
    pub fn capture() -> io::Result<Self> {
        Termios::from_fd(stream()).map(|inner| State { inner })
    }

    #[must_use]
    pub fn raw(&self) -> Self {
        let mut inner = self.inner;
        cfmakeraw(&mut inner);
        Self { inner }
    }

    pub fn apply(&self) -> io::Result<()> {
        tcsetattr(stream(), TCSANOW, &self.inner)
    }
}

pub fn window_size() -> io::Result<Vector2<u16>> {
    let mut win = winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };

    if unsafe { ioctl(stream(), TIOCGWINSZ, &mut win as *mut _) } == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok([win.ws_col, win.ws_row].into())
    }
}

pub fn raw_scope<T, F>(f: F) -> io::Result<T>
where
    F: FnOnce() -> T,
{
    let state = State::capture()?;
    let result = state.raw().apply().map(|_| f());
    state.apply().and(result)
}

#[must_use]
fn stream() -> RawFd {
    let stdin = io::stdin();
    stdin.as_raw_fd()
}
