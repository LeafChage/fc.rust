use super::attribute_table::{AttributeTable, ATTIRBUTE_TABLE_LENGTH};
use super::name_table::{NameTable, NAME_TABLE_LENGTH};
use crate::memory::{RAM, ROM, WOM};
use crate::result::{e, Result};
use crate::vec2::Vec2;

#[derive(Default, Debug)]
pub struct BackgroundTable {
    name: NameTable,
    attribute: AttributeTable,
}

impl BackgroundTable {
    pub fn fetch_line(&self, pos: Vec2<usize>, length: usize) -> (&[u8], &[u8]) {
        (
            self.name.fetch_line(pos, length),
            self.attribute.fetch_line(pos / 2, length / 2),
        )
    }
}

impl RAM<usize> for BackgroundTable {}

impl WOM<usize> for BackgroundTable {
    type Input = u8;
    fn put(&mut self, i: usize, v: Self::Input) -> Result<()> {
        if i < NAME_TABLE_LENGTH {
            self.name.put(i, v)
        } else if i < ATTIRBUTE_TABLE_LENGTH {
            self.attribute.put(i - NAME_TABLE_LENGTH, v)
        } else {
            Err(e::index_out_of_range(i))
        }
    }
}

impl ROM<usize> for BackgroundTable {
    type Output = u8;
    fn get(&self, i: usize) -> Result<Self::Output> {
        if i < NAME_TABLE_LENGTH {
            self.name.get(i)
        } else if i < ATTIRBUTE_TABLE_LENGTH {
            self.attribute.get(i - NAME_TABLE_LENGTH)
        } else {
            Err(e::index_out_of_range(i))
        }
    }
}
