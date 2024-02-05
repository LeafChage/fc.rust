use crate::array2::Array2;
use crate::memory::{RAM, ROM, WOM};
use crate::result::{e, Result};
use crate::vec2::Vec2;

pub const WIDTH: usize = 16;
pub const HEIGHT: usize = 15;

#[derive(Debug)]
pub struct AttributeTable {
    // index of palette
    palettes: Array2<u8>,
}

impl Default for AttributeTable {
    fn default() -> Self {
        Self {
            palettes: Array2::from_with_size(WIDTH, HEIGHT),
        }
    }
}

impl AttributeTable {
    fn analyze(v: u8) -> [[u8; 2]; 2] {
        let bottom_right = (0b11_00_00_00 & v) >> 6;
        let borrom_left = (0b00_11_00_00 & v) >> 4;
        let top_right = (0b00_00_11_00 & v) >> 2;
        let top_left = (0b00_00_00_11 & v) >> 0;
        [[top_left, top_right], [borrom_left, bottom_right]]
    }

    pub fn fetch_line(&self, pos: Vec2<usize>, length: usize) -> &[u8] {
        self.palettes.line(pos, length)
    }
}

pub const ATTIRBUTE_TABLE_LENGTH: usize = 64;
impl RAM<usize> for AttributeTable {}

impl WOM<usize> for AttributeTable {
    type Input = u8;
    fn put(&mut self, i: usize, v: Self::Input) -> Result<()> {
        if ATTIRBUTE_TABLE_LENGTH < i {
            Err(e::index_out_of_range(i))
        } else {
            let palettes = AttributeTable::analyze(v);
            let x = (i % (WIDTH / 2)) * 2;
            let y = (i / (WIDTH / 2)) * 2;
            self.palettes[[y, x]] = palettes[0][0];
            self.palettes[[y, x + 1]] = palettes[0][1];
            self.palettes[[y + 1, x]] = palettes[1][0];
            self.palettes[[y + 1, x + 1]] = palettes[1][1];
            Ok(())
        }
    }
}

impl ROM<usize> for AttributeTable {
    type Output = u8;

    fn get(&self, i: usize) -> Result<Self::Output> {
        if ATTIRBUTE_TABLE_LENGTH < i {
            Err(e::index_out_of_range(i))
        } else {
            let x = (i % (WIDTH / 2)) * 2;
            let y = (i / (WIDTH / 2)) * 2;

            Ok(self.palettes[[y + 1, x + 1]] << 6
                | self.palettes[[y + 1, x]] << 4
                | self.palettes[[y, x + 1]] << 2
                | self.palettes[[y, x]])
        }
    }
}

#[test]
fn it_put() {
    let mut a = AttributeTable::default();
    a.put(0, 0b_11_10_01_00).unwrap();
    assert_eq!(a.palettes[[0, 0]], 0);
    assert_eq!(a.palettes[[0, 1]], 1);
    assert_eq!(a.palettes[[1, 0]], 2);
    assert_eq!(a.palettes[[1, 1]], 3);
}

#[test]
fn it_put1() {
    let mut a = AttributeTable::default();
    a.put(9, 0b_11_10_01_00).unwrap();
    assert_eq!(a.palettes[[2, 2]], 0);
    assert_eq!(a.palettes[[2, 3]], 1);
    assert_eq!(a.palettes[[3, 2]], 2);
    assert_eq!(a.palettes[[3, 3]], 3);
}

#[test]
fn it_get() {
    let mut a = AttributeTable::default();
    a.put(9, 0b_11_10_01_00).unwrap();
    assert_eq!(a.get(9).unwrap(), 0b_11_10_01_00);
}
