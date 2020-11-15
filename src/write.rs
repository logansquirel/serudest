use crate::Error;

pub trait Write {
    fn write(&mut self, byte: &[u8]) -> Result<(), Error>;
}
