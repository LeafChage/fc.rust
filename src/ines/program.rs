use crate::memory::ROM;
use crate::result::{e, Result};

#[derive(Debug)]
pub struct ProgramRom<'a> {
    rom: &'a [u8],
}

impl<'a> ROM for ProgramRom<'a> {
    fn range(&self) -> std::ops::Range<usize> {
        0..(self.rom.len())
    }

    fn get(&self, i: usize) -> Result<u8> {
        match i {
            i if self.range().contains(&i) => Ok(self.rom[i]),
            _ => dbg!(Err(e::index_out_of_range(i, self.range()))),
        }
    }
}

impl<'a> ProgramRom<'a> {
    pub fn new(rom: &'a [u8]) -> Self {
        Self { rom }
    }
}
