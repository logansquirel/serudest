use crate::Error;

pub trait Read {
    fn peek(&mut self) -> Result<Option<u8>, Error>;
    fn next(&mut self) -> Result<Option<u8>, Error>;
}
