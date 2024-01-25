use crate::result::{e, Result};
use crate::x::range;

/// read only memory
pub trait ROM<Idx: Sized> {
    type Output: Sized;
    fn get(&self, i: Idx) -> Result<Self::Output>;
}

/// write only memory
pub trait WOM<Idx: Sized> {
    type Input: Sized;
    fn put(&mut self, i: Idx, v: Self::Input) -> Result<()>;
}

pub trait RAM<Idx: Sized>: ROM<Idx> + WOM<Idx> {}

/**
 * Vec<u8>
 */
impl RAM<usize> for Vec<u8> {}
impl ROM<usize> for Vec<u8> {
    type Output = u8;

    fn get(&self, i: usize) -> Result<Self::Output> {
        if self.len() > i {
            Ok(self[i])
        } else {
            Err(e::index_out_of_range(i))
        }
    }
}
impl ROM<std::ops::Range<usize>> for Vec<u8> {
    type Output = Vec<u8>;

    fn get(&self, i: std::ops::Range<usize>) -> Result<Self::Output> {
        if range::inside(0..self.len(), &i) {
            Ok(self[i].to_vec())
        } else {
            Err(e::index_out_of_range(i.start))
        }
    }
}
impl WOM<usize> for Vec<u8> {
    type Input = u8;
    fn put(&mut self, i: usize, v: Self::Input) -> Result<()> {
        if self.len() > i {
            self[i] = v;
        } else {
            return Err(e::index_out_of_range(i));
        }
        Ok(())
    }
}

/**
 * [u8]
 */
impl RAM<usize> for [u8] {}
impl ROM<usize> for [u8] {
    type Output = u8;

    fn get(&self, i: usize) -> Result<Self::Output> {
        if self.len() > i {
            Ok(self[i])
        } else {
            Err(e::index_out_of_range(i))
        }
    }
}
impl ROM<std::ops::Range<usize>> for [u8] {
    type Output = Vec<u8>;

    fn get(&self, i: std::ops::Range<usize>) -> Result<Self::Output> {
        if range::inside(0..self.len(), &i) {
            Ok(self[i].to_vec())
        } else {
            Err(e::index_out_of_range(i.start))
        }
    }
}
impl WOM<usize> for [u8] {
    type Input = u8;
    fn put(&mut self, i: usize, v: Self::Input) -> Result<()> {
        if self.len() > i {
            self[i] = v;
        } else {
            return Err(e::index_out_of_range(i));
        }
        Ok(())
    }
}
