use crate::result::Result;

pub trait ROM<Idx: Sized> {
    type Output: Sized;
    fn get(&self, i: Idx) -> Result<Self::Output>;
}

pub trait RAM: ROM<usize, Output = u8> {
    fn put(&mut self, i: usize, v: u8) -> Result<()>;
}

impl<T> ROM<usize> for T
where
    T: std::ops::Index<usize, Output = u8>,
{
    type Output = T::Output;

    fn get(&self, i: usize) -> Result<Self::Output> {
        Ok(self[i])
    }
}

impl<T> ROM<std::ops::Range<usize>> for T
where
    T: std::ops::Index<std::ops::Range<usize>, Output = [u8]>,
{
    type Output = Vec<u8>;

    fn get(&self, i: std::ops::Range<usize>) -> Result<Self::Output> {
        Ok(self.index(i).to_vec())
    }
}

impl<T> RAM for T
where
    T: std::ops::IndexMut<usize, Output = u8>,
{
    fn put(&mut self, i: usize, v: u8) -> Result<()> {
        self[i] = v;
        Ok(())
    }
}
