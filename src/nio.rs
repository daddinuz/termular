use std::io::{self, Read};
use std::sync::mpsc::{self, Receiver};
use std::thread;
use std::time::{Duration, Instant};

pub struct NonblockingStdin {
    receiver: Receiver<io::Result<u8>>,
}

impl NonblockingStdin {
    #[must_use]
    fn instance() -> Self {
        let (sender, receiver) = mpsc::channel();

        thread::spawn(move || {
            let stdin = io::stdin();
            let handle = stdin.lock();
            let mut stream = handle.bytes();
            loop {
                // TODO: think twice before unwrapping.
                stream.try_for_each(|io| sender.send(io)).unwrap();
            }
        });

        Self { receiver }
    }
}

impl Read for NonblockingStdin {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        buf.iter_mut()
            // From: (https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.zip)
            // >>> If the first iterator returns None, zip will short-circuit and next will not be called on the second iterator.
            .zip(self.receiver.try_iter())
            .try_fold(0, |counter, (item, io)| {
                io.map(|byte| {
                    *item = byte;
                    counter + 1
                })
            })
    }
}

impl NonblockingStdin {
    pub fn read_timeout(&mut self, buf: &mut [u8], timeout: Duration) -> io::Result<usize> {
        let mut counter = 0;
        let deadline = Instant::now() + timeout;
        for item in buf.iter_mut() {
            match self.receiver.recv_deadline(deadline) {
                Ok(io) => {
                    *item = io?;
                    counter += 1;
                }
                Err(err) => {
                    if counter > 0 {
                        break;
                    }

                    return Err(io::Error::new(io::ErrorKind::TimedOut, err));
                }
            }
        }

        Ok(counter)
    }
}

pub fn stdin() -> NonblockingStdin {
    NonblockingStdin::instance()
}
