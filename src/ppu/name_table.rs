use crate::array2::Array2;
use crate::memory::{RAM, ROM, WOM};
use crate::result::{e, Result};
use crate::vec2::Vec2;

pub const NAME_TABLE_LENGTH: usize = 0x03C0;
pub const WIDTH: usize = 32;
pub const HEIGHT: usize = 30;

#[derive(Debug)]
pub struct NameTable {
    tiles: Array2<u8>,
}

impl Default for NameTable {
    fn default() -> Self {
        Self {
            tiles: Array2::from_with_size(WIDTH, HEIGHT),
        }
    }
}

impl NameTable {
    pub fn fetch_line(&self, pos: Vec2<usize>, length: usize) -> &[u8] {
        self.tiles.line(pos, length)
    }
}

impl RAM<usize> for NameTable {}

impl WOM<usize> for NameTable {
    type Input = u8;
    fn put(&mut self, i: usize, v: Self::Input) -> Result<()> {
        if NAME_TABLE_LENGTH < i {
            Err(e::index_out_of_range(i))
        } else {
            self.tiles[[i / WIDTH, i % WIDTH]] = v;
            Ok(())
        }
    }
}

impl ROM<usize> for NameTable {
    type Output = u8;

    fn get(&self, i: usize) -> Result<Self::Output> {
        if NAME_TABLE_LENGTH < i {
            Err(e::index_out_of_range(i))
        } else {
            Ok(self.tiles[[i / WIDTH, i % WIDTH]])
        }
    }
}

#[test]
fn it_get() {
    let mut name = NameTable::default();
    name.tiles[[1, 2]] = 1;
    assert_eq!(name.get(WIDTH + 2).unwrap(), 1);
}

#[test]
fn it_put() {
    let mut name = NameTable::default();
    name.put(WIDTH + 2, 1).unwrap();
    assert_eq!(name.tiles[[1, 2]], 1)
}
