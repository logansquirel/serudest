use crate::Error;

pub trait Read {
    fn peek(&mut self) -> Result<Option<u8>, Error>;
    fn next(&mut self) -> Result<Option<u8>, Error>;
}

pub struct IoRead<R> {
    bytes: std::io::Bytes<R>,
    next: Option<u8>,
}

impl<R> IoRead<R>
where
    R: std::io::Read,
{
    pub fn new(reader: R) -> Self {
        IoRead {
            bytes: reader.bytes(),
            next: None,
        }
    }
}

impl<R> Read for IoRead<R>
where
    R: std::io::Read,
{
    fn peek(&mut self) -> Result<Option<u8>, Error> {
        match self.next {
            Some(x) => Ok(Some(x)),
            None => match self.bytes.next() {
                Some(Err(_)) => Err(Error),
                Some(Ok(x)) => Ok(Some(x)),
                None => Ok(None),
            },
        }
    }

    fn next(&mut self) -> Result<Option<u8>, Error> {
        match self.next.take() {
            Some(x) => Ok(Some(x)),
            None => match self.bytes.next() {
                Some(Err(_)) => Err(Error),
                Some(Ok(x)) => Ok(Some(x)),
                None => Ok(None),
            },
        }
    }
}

pub struct SliceRead<'a> {
    slice: &'a [u8],
    index: usize,
}

impl<'a> SliceRead<'a> {
    pub fn new(slice: &'a [u8]) -> Self {
        SliceRead { slice, index: 0 }
    }
}

impl<'a> Read for SliceRead<'a> {
    fn peek(&mut self) -> Result<Option<u8>, Error> {
        if self.index < self.slice.len() {
            Ok(Some(self.slice[self.index]))
        } else {
            Ok(None)
        }
    }

    fn next(&mut self) -> Result<Option<u8>, Error> {
        if self.index < self.slice.len() {
            let ch = self.slice[self.index];
            self.index += 1;
            Ok(Some(ch))
        } else {
            Ok(None)
        }
    }
}

pub struct StrRead<'a> {
    bytes: &'a [u8],
    index: usize,
}

impl<'a> StrRead<'a> {
    pub fn new(string: &'a str) -> Self {
        StrRead {
            bytes: string.as_bytes(),
            index: 0,
        }
    }
}

impl<'a> Read for StrRead<'a> {
    fn peek(&mut self) -> Result<Option<u8>, Error> {
        if self.index < self.bytes.len() {
            Ok(Some(self.bytes[self.index]))
        } else {
            Ok(None)
        }
    }

    fn next(&mut self) -> Result<Option<u8>, Error> {
        if self.index < self.bytes.len() {
            let ch = self.bytes[self.index];
            self.index += 1;
            Ok(Some(ch))
        } else {
            Ok(None)
        }
    }
}
