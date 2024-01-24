use crate::result::{e, Result};
use crate::x::RangeInside;

pub trait ROM<Idx: Sized> {
    type Output: Sized;
    fn get(&self, i: Idx) -> Result<Self::Output>;
}

pub trait RAM<Idx: Sized>: ROM<usize, Output = u8> {
    fn put(&mut self, i: Idx, v: Self::Output) -> Result<()>;
}

impl<T> ROM<usize> for T
where
    T: std::ops::Index<usize, Output = u8> + Len,
{
    type Output = T::Output;

    fn get(&self, i: usize) -> Result<Self::Output> {
        if self.length() > i {
            Ok(self[i])
        } else {
            Err(e::index_out_of_range(i))
        }
    }
}

impl<T> ROM<std::ops::Range<usize>> for T
where
    T: std::ops::Index<std::ops::Range<usize>, Output = [u8]> + Len,
{
    type Output = Vec<u8>;

    fn get(&self, i: std::ops::Range<usize>) -> Result<Self::Output> {
        if (0..self.length()).inside(&i) {
            Ok(self[i].to_vec())
        } else {
            Err(e::index_out_of_range(i.start))
        }
    }
}

impl<T> RAM<usize> for T
where
    T: std::ops::IndexMut<usize, Output = u8> + Len,
{
    fn put(&mut self, i: usize, v: u8) -> Result<()> {
        if self.length() > i {
            self[i] = v;
        } else {
            return Err(e::index_out_of_range(i));
        }
        Ok(())
    }
}

pub trait Len {
    fn length(&self) -> usize;
}

impl Len for Vec<u8> {
    fn length(&self) -> usize {
        self.len()
    }
}

impl<T> Len for &[T] {
    fn length(&self) -> usize {
        self.len()
    }
}
