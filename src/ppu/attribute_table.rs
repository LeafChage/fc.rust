use crate::memory::{RAM, ROM, WOM};
use crate::rect::Rect;
use crate::vec2::Vec2;
use crate::result::{e, Result};

pub const WIDTH: usize = 16;
pub const HEIGHT: usize = 15;

#[derive(Default, Debug)]
pub struct AttributeTable {
    // index of palette
    palettes: [[u8; WIDTH]; HEIGHT],
}

impl AttributeTable {
    fn analyze(v: u8) -> [[u8; 2]; 2] {
        let bottom_right = (0b11_00_00_00 & v) >> 6;
        let borrom_left = (0b00_11_00_00 & v) >> 4;
        let top_right = (0b00_00_11_00 & v) >> 2;
        let top_left = (0b00_00_00_11 & v) >> 0;
        [[top_left, top_right], [borrom_left, bottom_right]]
    }

    /// TODO: NameTableにも同じ処理があるので、
    /// 多次元配列へのアクセスを共通化したい。
    pub fn fetch(&self, r: Rect) -> Vec<&[u8]> {
        let Vec2(w, h) = r.size();
        let Vec2(x, y) = r.pos();

        let mut result = Vec::new();
        for y in y..(y + h) {
            result.push(&self.palettes[y][x..(x + w)]);
        }
        result
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
            self.palettes[y][x] = palettes[0][0];
            self.palettes[y][x + 1] = palettes[0][1];
            self.palettes[y + 1][x] = palettes[1][0];
            self.palettes[y + 1][x + 1] = palettes[1][1];
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

            Ok(self.palettes[y + 1][x + 1] << 6
                | self.palettes[y + 1][x] << 4
                | self.palettes[y][x + 1] << 2
                | self.palettes[y][x])
        }
    }
}

#[test]
fn it_put() {
    let mut a = AttributeTable {
        palettes: [[0; WIDTH]; HEIGHT],
    };
    a.put(0, 0b_11_10_01_00).unwrap();
    assert_eq!(a.palettes[0][0], 0);
    assert_eq!(a.palettes[0][1], 1);
    assert_eq!(a.palettes[1][0], 2);
    assert_eq!(a.palettes[1][1], 3);
}

#[test]
fn it_put1() {
    let mut a = AttributeTable {
        palettes: [[0; WIDTH]; HEIGHT],
    };
    a.put(9, 0b_11_10_01_00).unwrap();
    assert_eq!(a.palettes[2][2], 0);
    assert_eq!(a.palettes[2][3], 1);
    assert_eq!(a.palettes[3][2], 2);
    assert_eq!(a.palettes[3][3], 3);
}

#[test]
fn it_get() {
    let mut a = AttributeTable {
        palettes: [[0; WIDTH]; HEIGHT],
    };
    a.put(9, 0b_11_10_01_00).unwrap();
    assert_eq!(a.get(9).unwrap(), 0b_11_10_01_00);
}
