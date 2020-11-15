use crate::Error;

pub trait Write {
    fn write(&mut self, bytes: &[u8]) -> Result<(), Error>;
}

pub struct IoWrite<W> {
    writer: W,
}

impl<W> IoWrite<W>
where
    W: std::io::Write,
{
    pub fn new(writer: W) -> Self {
        IoWrite { writer }
    }
}

impl<W> Write for IoWrite<W>
where
    W: std::io::Write,
{
    fn write(&mut self, bytes: &[u8]) -> Result<(), Error> {
        self.writer.write_all(bytes).map_err(|_| Error)
    }
}

pub struct FmtWrite<W> {
    writer: W,
}

impl<W> FmtWrite<W>
where
    W: std::fmt::Write,
{
    pub fn new(writer: W) -> Self {
        FmtWrite { writer }
    }
}

impl<W> Write for FmtWrite<W>
where
    W: std::fmt::Write,
{
    fn write(&mut self, bytes: &[u8]) -> Result<(), Error> {
        let string = unsafe { std::str::from_utf8_unchecked(bytes) };
        self.writer.write_str(string).map_err(|_| Error)
    }
}
