use std::io;
use std::os::unix::io::{AsRawFd, RawFd};
use std::sync::Once;
use termios::{cfmakeraw, tcsetattr, Termios, TCSANOW};

pub enum Mode {
    Native,
    Raw,
}

pub fn set(mode: Mode) -> io::Result<()> {
    INIT.call_once(|| {
        // TODO: think twice before unwrapping.
        let state = Termios::from_fd(stdin_descriptor()).unwrap();
        if unsafe { STATE.replace(state) }.is_some() {
            panic!("Already initialized state");
        }
    });

    let descriptor = stdin_descriptor();
    let state = unsafe { STATE.as_ref().unwrap() };

    match mode {
        Mode::Native => tcsetattr(descriptor, TCSANOW, state),
        Mode::Raw => {
            let mut raw_state = *state;
            cfmakeraw(&mut raw_state);
            tcsetattr(descriptor, TCSANOW, &raw_state)
        }
    }
}

fn stdin_descriptor() -> RawFd {
    let stdin = io::stdin();
    stdin.as_raw_fd()
}

static INIT: Once = Once::new();
static mut STATE: Option<Termios> = None;
