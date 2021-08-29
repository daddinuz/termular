use std::io::{self, Read};
use std::sync::mpsc::{self, Receiver};
use std::time::{Duration, Instant};
use std::{iter, mem, thread};

const READ_TIMEOUT: Duration = Duration::from_millis(8);

pub trait ReadNonblock {
    fn read_timeout(&mut self, buf: &mut [u8], timeout: Duration) -> io::Result<usize>;

    fn read_timeout_until(
        &mut self,
        delimiter: u8,
        buf: &mut Vec<u8>,
        timeout: Duration,
    ) -> io::Result<usize>;
}

pub struct Stdin {
    receiver: Receiver<io::Result<Option<u8>>>,
    last_err: io::Result<()>,
}

#[must_use]
pub(crate) fn stdin() -> Stdin {
    let (sender, receiver) = mpsc::channel();
    let last_err = Ok(());

    thread::spawn(move || {
        let stdin = io::stdin();
        let handle = stdin.lock();
        let mut stream = handle.bytes();

        // From: (https://doc.rust-lang.org/std/sync/mpsc/struct.SendError.html)
        // >>> A send operation can only fail if the receiving end of a channel
        // >>> is disconnected, implying that the data could never be received.
        //
        // loop until the receiving end of the channel is disconnected.
        // EOF is transmitted as Ok(None).
        while stream.try_for_each(|io| sender.send(io.map(Some))).is_ok() {
            if sender.send(Ok(None)).is_err() {
                break;
            }
        }
    });

    Stdin { receiver, last_err }
}

impl Read for Stdin {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        // If last time we had an error, return the error.
        mem::replace(&mut self.last_err, Ok(()))?;

        // blocking-like iterator over stdin bytes: `recv` will block until
        // input is available, `recv_timeout` tries to pull all pending
        // bytes from the stream. When time-out occurs EOF is assumed.
        let stream = iter::once_with(|| self.receiver.recv().unwrap_or(Ok(None))).chain(
            iter::from_fn(|| self.receiver.recv_timeout(READ_TIMEOUT).ok()),
        );
        let mut counter = 0;

        // From: (https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.zip)
        // >>> If the first iterator returns None, zip will short-circuit
        // >>> and next will not be called on the second iterator.
        for (item, io) in buf.iter_mut().zip(stream) {
            match io {
                Ok(Some(byte)) => {
                    *item = byte;
                    counter += 1;
                    if byte == b'\n' {
                        break;
                    }
                }
                Ok(None) => break,
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

impl ReadNonblock for Stdin {
    fn read_timeout(&mut self, buf: &mut [u8], timeout: Duration) -> io::Result<usize> {
        // If last time we had an error, return the error.
        mem::replace(&mut self.last_err, Ok(()))?;

        let mut counter = 0;
        let deadline = Instant::now() + timeout;
        let stream = iter::from_fn(|| self.receiver.recv_deadline(deadline).ok());

        for (item, io) in buf.iter_mut().zip(stream) {
            match io {
                Ok(Some(byte)) => {
                    *item = byte;
                    counter += 1;
                    if byte == b'\n' {
                        break;
                    }
                }
                Ok(None) => break,
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

    fn read_timeout_until(
        &mut self,
        delimiter: u8,
        buf: &mut Vec<u8>,
        timeout: Duration,
    ) -> io::Result<usize> {
        // If last time we had an error, return the error.
        mem::replace(&mut self.last_err, Ok(()))?;

        let start_len = buf.len();
        let deadline = Instant::now() + timeout;
        let stream = iter::from_fn(|| self.receiver.recv_deadline(deadline).ok());

        for io in stream {
            match io {
                Ok(Some(byte)) => {
                    buf.push(byte);
                    if byte == delimiter {
                        break;
                    }
                }
                Ok(None) => break,
                Err(err) => {
                    if buf.len() > start_len {
                        self.last_err = Err(err);
                        break;
                    }
                    return Err(err);
                }
            }
        }

        Ok(buf.len() - start_len)
    }
}
