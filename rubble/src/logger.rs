use {crate::ble::time::Timer, bbqueue::Producer, core::fmt};

/// A `fmt::Write` adapter that prints a timestamp before each line.
pub struct StampedLogger<T: Timer, L: fmt::Write> {
    timer: T,
    inner: L,
}

impl<T: Timer, L: fmt::Write> StampedLogger<T, L> {
    /// Creates a new `StampedLogger` that will print to `inner` and obtains timestamps using
    /// `timer`.
    pub fn new(inner: L, timer: T) -> Self {
        Self { inner, timer }
    }
}

impl<T: Timer, L: fmt::Write> fmt::Write for StampedLogger<T, L> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for (i, line) in s.split('\n').enumerate() {
            if i != 0 {
                write!(self.inner, "\n{} - ", self.timer.now())?;
            }

            self.inner.write_str(line)?;
        }

        Ok(())
    }
}

/// A `fmt::Write` sink that writes to a `BBQueue`.
///
/// The sink will panic when the `BBQueue` doesn't have enough space to the data. This is to ensure
/// that we never block or drop data.
pub struct BbqLogger {
    p: Producer,
}

impl BbqLogger {
    pub fn new(p: Producer) -> Self {
        Self { p }
    }
}

impl fmt::Write for BbqLogger {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let mut bytes = s.as_bytes();

        while !bytes.is_empty() {
            let mut grant = self.p.grant_max(bytes.len()).expect("log buffer overflow");
            let size = grant.buf().len();
            grant.buf().copy_from_slice(&bytes[..size]);
            bytes = &bytes[size..];
            self.p.commit(size, grant);
        }

        Ok(())
    }
}
