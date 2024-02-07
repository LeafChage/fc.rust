use crate::array2::Array2;
use crate::memory::{RAM, ROM, WOM};
use crate::rect::Rect;
use crate::result::{e, Result};
use crate::vec2::Vec2;

pub const WIDTH: usize = 16; // block count
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
    fn analyze(v: u8) -> Array2<u8> {
        let bottom_right = (0b11_00_00_00 & v) >> 6;
        let borrom_left = (0b00_11_00_00 & v) >> 4;
        let top_right = (0b00_00_11_00 & v) >> 2;
        let top_left = (0b00_00_00_11 & v) >> 0;
        Array2::new(vec![
            vec![top_left, top_right],
            vec![borrom_left, bottom_right],
        ])
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
        if i >= ATTIRBUTE_TABLE_LENGTH {
            Err(e::index_out_of_range(i))
        } else {
            let palettes = AttributeTable::analyze(v);
            let (width, _) = (self.palettes.dimention() / 2).xy();
            let x = (i % width) * 2;
            let y = (i / width) * 2;

            if y == 14 {
                // 最後の行だけ4つ分入らない
                self.palettes.put_array2(
                    Vec2::new(x, y),
                    palettes.part_of(Rect::from_pos_size(Vec2::zero(), Vec2::new(2, 1))),
                );
            } else {
                self.palettes.put_array2(Vec2::new(x, y), palettes);
            }
            Ok(())
        }
    }
}

impl ROM<usize> for AttributeTable {
    type Output = u8;

    fn get(&self, i: usize) -> Result<Self::Output> {
        if i >= ATTIRBUTE_TABLE_LENGTH {
            Err(e::index_out_of_range(i))
        } else {
            let (width, _) = self.palettes.dimention().xy();
            let x = (i % (width / 2)) * 2;
            let y = (i / (width / 2)) * 2;

            Ok(self.palettes[[x + 1, y + 1]] << 6
                | self.palettes[[x + 1, y]] << 4
                | self.palettes[[x, y + 1]] << 2
                | self.palettes[[x, y]])
        }
    }
}

#[test]
fn it_put() {
    let mut a = AttributeTable {
        palettes: Array2::from_with_size(3, 3),
    };
    a.put(0, 0b_11_10_01_00).unwrap();
    assert_eq!(a.palettes[[0, 0]], 0);
    assert_eq!(a.palettes[[1, 0]], 1);
    assert_eq!(a.palettes[[0, 1]], 2);
    assert_eq!(a.palettes[[1, 1]], 3);
}

#[test]
fn it_put1() {
    let mut a = AttributeTable {
        palettes: Array2::from_with_size(10, 10),
    };
    a.put(5, 0b_11_10_01_00).unwrap();
    dbg!(&a.palettes);
    assert_eq!(a.palettes[[0, 2]], 0);
    assert_eq!(a.palettes[[1, 2]], 1);
    assert_eq!(a.palettes[[0, 3]], 2);
    assert_eq!(a.palettes[[1, 3]], 3);
}

#[test]
fn it_get() {
    let mut a = AttributeTable {
        palettes: Array2::from_with_size(10, 10),
    };
    a.put(8, 0b_11_10_01_00).unwrap();
    dbg!(&a.palettes);
    assert_eq!(a.palettes[[6, 2]], 0);
    assert_eq!(a.palettes[[7, 2]], 1);
    assert_eq!(a.palettes[[6, 3]], 2);
    assert_eq!(a.palettes[[7, 3]], 3);
}
