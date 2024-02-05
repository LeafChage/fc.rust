use super::background_table::BackgroundTable;
use super::palette::PaletteTable;
use crate::memory::{RAM, ROM, WOM};
use crate::result::{e, Result};
use crate::sprite::Sprite;
use crate::ines::SpriteROM;

pub struct MemoryMap {
    // 0x0000～0x0FFF
    // 0x1000～0x1FFF
    pub pattern: SpriteROM,

    /// name0: 0x2000～0x23BF
    /// attribute0: 0x23C0～0x23FF
    pub background0: BackgroundTable,

    /// name1 0x2400～0x27BF
    /// attribute1 0x27C0～0x27FF
    pub background1: BackgroundTable,

    /// name2 0x2800～0x2BBF
    /// attribute2 0x2BC0～0x2BFF
    pub background2: BackgroundTable,

    /// name3 0x2C00～0x2FBF
    /// attribute3 0x2FC0～0x2FFF
    pub background3: BackgroundTable,

    /// mirror of name and attribute table
    /// 0x3000～0x3EFF
    _mirror1: [u8; 0],

    pub palette: PaletteTable, // 0x3F00～0x3F1F

    /// mirror of  palette
    /// 0x3F20～0x3FFF
    _mirror2: [u8; 0],
}

impl MemoryMap {
    pub fn sprite(&self, index: usize) -> Sprite {
        self.pattern[index]
    }
    pub fn new(pattern: SpriteROM) -> Self {
        MemoryMap {
            pattern,
            background0: BackgroundTable::default(),
            background1: BackgroundTable::default(),
            background2: BackgroundTable::default(),
            background3: BackgroundTable::default(),
            _mirror1: [0; 0],
            palette: PaletteTable::default(),
            _mirror2: [0; 0],
        }
    }
}

impl RAM<usize> for MemoryMap {}

impl ROM<usize> for MemoryMap {
    type Output = u8;

    fn get(&self, i: usize) -> Result<Self::Output> {
        match i {
            i if (0x0000..=0x1FFF).contains(&i) => unreachable!(), // this is sprite
            i if (0x2000..=0x23FF).contains(&i) => self.background0.get(i - 0x2000),
            i if (0x2400..=0x27FF).contains(&i) => self.background1.get(i - 0x2400),
            i if (0x2800..=0x2BFF).contains(&i) => self.background2.get(i - 0x2800),
            i if (0x2C00..=0x2FFF).contains(&i) => self.background3.get(i - 0x2C00),
            i if (0x3000..=0x3EFF).contains(&i) => match i - 0x3000 {
                i if (0x0000..=0x03FF).contains(&i) => self.background0.get(i),
                i if (0x0400..=0x07FF).contains(&i) => self.background1.get(i - 0x0400),
                i if (0x0800..=0x0BFF).contains(&i) => self.background2.get(i - 0x0800),
                i if (0x0C00..=0x0FFF).contains(&i) => self.background3.get(i - 0x0C00),
                _ => Err(e::index_out_of_range(i)),
            },
            i if (0x3F00..=0x3F1F).contains(&i) => self.palette.get(i - 0x3f00),
            i if (0x3F20..=0x3FFF).contains(&i) => self.palette.get(i - 0x3f20),
            _ => Err(e::index_out_of_range(i)),
        }
    }
}

impl WOM<usize> for MemoryMap {
    type Input = u8;
    fn put(&mut self, i: usize, v: Self::Input) -> Result<()> {
        match i {
            i if (0x0000..=0x1FFF).contains(&i) => Err(e::readonly(i)),
            i if (0x2000..=0x23FF).contains(&i) => self.background0.put(i - 0x2000, v),
            i if (0x2400..=0x27FF).contains(&i) => self.background1.put(i - 0x2400, v),
            i if (0x2800..=0x2BFF).contains(&i) => self.background2.put(i - 0x2800, v),
            i if (0x2C00..=0x2FFF).contains(&i) => self.background3.put(i - 0x2C00, v),
            i if (0x3000..=0x3EFF).contains(&i) => {
                unreachable!("this is mirror {}", i)
            }
            i if (0x3F00..=0x3F1F).contains(&i) => self.palette.put(i - 0x3F00, v),
            i if (0x3F20..=0x3FFF).contains(&i) => {
                unreachable!("this is mirror {}", i)
            }
            _ => Err(e::index_out_of_range(i)),
        }
    }
}
