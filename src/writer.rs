use std::io;

pub struct PartialResult(pub usize, pub io::IoError);

pub trait NetWriter {
    fn write(&mut self, buf: &[u8]) -> Result<usize, PartialResult>;
}