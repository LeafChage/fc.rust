use crate::result::Result;

pub trait ROM {
    fn range(&self) -> std::ops::Range<usize>;
    fn get(&self, i: usize) -> Result<u8>;
}

pub trait RAM: ROM {
    fn put(&mut self, i: usize, v: u8) -> Result<()>;
}

