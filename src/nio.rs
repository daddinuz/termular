use std::io::{self, BufRead, BufReader, IoSliceMut, Read};
use std::sync::mpsc::{self, Receiver};
use std::time::{Duration, Instant};
use std::{iter, mem, thread};

pub struct Stdin {
    inner: BufReader<StdinRaw>,
}

#[must_use]
pub(crate) fn stdin() -> Stdin {
    Stdin {
        inner: BufReader::new(StdinRaw::new()),
    }
}

impl Read for Stdin {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.read(buf)
    }

    #[inline]
    fn read_vectored(&mut self, bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
        self.inner.read_vectored(bufs)
    }

    #[inline]
    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> io::Result<usize> {
        self.inner.read_to_end(buf)
    }

    #[inline]
    fn read_to_string(&mut self, buf: &mut String) -> io::Result<usize> {
        self.inner.read_to_string(buf)
    }

    #[inline]
    fn read_exact(&mut self, buf: &mut [u8]) -> io::Result<()> {
        self.inner.read_exact(buf)
    }
}

impl BufRead for Stdin {
    #[inline]
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        self.inner.fill_buf()
    }

    #[inline]
    fn consume(&mut self, amt: usize) {
        self.inner.consume(amt);
    }

    #[inline]
    fn read_until(&mut self, byte: u8, buf: &mut Vec<u8>) -> io::Result<usize> {
        self.inner.read_until(byte, buf)
    }

    #[inline]
    fn read_line(&mut self, buf: &mut String) -> io::Result<usize> {
        self.inner.read_line(buf)
    }
}

pub trait ReadNonblock {
    fn read_timeout(&mut self, buf: &mut [u8], timeout: Duration) -> io::Result<usize>;

    fn read_timeout_until(
        &mut self,
        delimiter: u8,
        buf: &mut Vec<u8>,
        timeout: Duration,
    ) -> io::Result<usize>;
}

impl ReadNonblock for Stdin {
    #[inline]
    fn read_timeout(&mut self, buf: &mut [u8], timeout: Duration) -> io::Result<usize> {
        self.inner.get_mut().read_timeout(buf, timeout)
    }

    #[inline]
    fn read_timeout_until(
        &mut self,
        delimiter: u8,
        buf: &mut Vec<u8>,
        timeout: Duration,
    ) -> io::Result<usize> {
        self.inner
            .get_mut()
            .read_timeout_until(delimiter, buf, timeout)
    }
}

const READ_TIMEOUT: Duration = Duration::from_millis(8);

struct StdinRaw {
    receiver: Receiver<io::Result<Option<u8>>>,
    last_err: io::Result<()>,
}

impl StdinRaw {
    #[must_use]
    fn new() -> StdinRaw {
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

        StdinRaw { receiver, last_err }
    }
}

impl Read for StdinRaw {
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

impl ReadNonblock for StdinRaw {
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
