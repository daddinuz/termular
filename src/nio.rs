use std::io::{self, Read};
use std::sync::mpsc::{self, Receiver};
use std::time::{Duration, Instant};
use std::{mem, thread};

pub struct StdinNonblock {
    receiver: Receiver<io::Result<u8>>,
    last_err: io::Result<()>,
}

impl StdinNonblock {
    #[must_use]
    fn instance() -> Self {
        let (sender, receiver) = mpsc::channel();
        let last_err = Ok(());

        thread::spawn(move || {
            let stdin = io::stdin();
            let handle = stdin.lock();
            let mut stream = handle.bytes();

            // From: (https://doc.rust-lang.org/std/sync/mpsc/struct.SendError.html)
            // >>> A send operation can only fail if the receiving end of a channel is disconnected, implying that the data could never be received.
            while stream.try_for_each(|io| sender.send(io)).is_ok() {}
        });

        Self { receiver, last_err }
    }
}

impl Read for StdinNonblock {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        // If last time we had an error, return the error.
        mem::replace(&mut self.last_err, Ok(()))?;

        let mut counter = 0;

        // From: (https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.zip)
        // >>> If the first iterator returns None, zip will short-circuit and next will not be called on the second iterator.
        for (item, io) in buf.iter_mut().zip(self.receiver.try_iter()) {
            match io {
                Ok(byte) => {
                    *item = byte;
                    counter += 1;
                }
                Err(err) => {
                    if counter > 0 {
                        self.last_err = Err(err);
                        break;
                    }
                    return Err(err);
                }
            }
        }

        Ok(counter)
    }
}

impl StdinNonblock {
    pub fn read_timeout(&mut self, buf: &mut [u8], timeout: Duration) -> io::Result<usize> {
        // If last time we had an error, return the error.
        mem::replace(&mut self.last_err, Ok(()))?;

        let mut counter = 0;
        let deadline = Instant::now() + timeout;

        for item in buf.iter_mut() {
            match self.receiver.recv_deadline(deadline) {
                Ok(io) => match io {
                    Ok(byte) => {
                        *item = byte;
                        counter += 1;
                    }
                    Err(err) => {
                        if counter > 0 {
                            self.last_err = Err(err);
                            break;
                        }
                        return Err(err);
                    }
                },
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

    pub fn read_timeout_until(
        &mut self,
        delimiter: u8,
        buf: &mut Vec<u8>,
        timeout: Duration,
    ) -> io::Result<usize> {
        // If last time we had an error, return the error.
        mem::replace(&mut self.last_err, Ok(()))?;

        let start_len = buf.len();
        let deadline = Instant::now() + timeout;

        loop {
            match self.receiver.recv_deadline(deadline) {
                Ok(io) => match io {
                    Ok(byte) => {
                        buf.push(byte);
                        if byte == delimiter {
                            break;
                        }
                    }
                    Err(err) => {
                        if buf.len() > start_len {
                            self.last_err = Err(err);
                            break;
                        }
                        return Err(err);
                    }
                },
                Err(err) => {
                    if buf.len() > start_len {
                        break;
                    }
                    return Err(io::Error::new(io::ErrorKind::TimedOut, err));
                }
            }
        }

        Ok(buf.len() - start_len)
    }
}

pub fn stdin() -> StdinNonblock {
    StdinNonblock::instance()
}
