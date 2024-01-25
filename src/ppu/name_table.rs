use crate::memory::{RAM, ROM, WOM};
use crate::rect::Rect;
use crate::result::{e, Result};
use crate::vec2::Vec2;

pub const NAME_TABLE_LENGTH: usize = 0x03C0;
pub const WIDTH: usize = 32;
pub const HEIGHT: usize = 30;

#[derive(Default, Debug)]
pub struct NameTable {
    tiles: [[u8; WIDTH]; HEIGHT],
}

impl NameTable {
    /// TODO: AttributeTableにも同じ処理があるので、
    /// 多次元配列へのアクセスを共通化したい。
    pub fn fetch(&self, r: Rect) -> Vec<&[u8]> {
        let Vec2(w, h) = r.size();
        let Vec2(x, y) = r.pos();

        let mut result = Vec::new();
        for y in y..(y + h) {
            result.push(&self.tiles[y][x..(x + w)])
        }
        result
    }
}

impl RAM<usize> for NameTable {}

impl WOM<usize> for NameTable {
    type Input = u8;
    fn put(&mut self, i: usize, v: Self::Input) -> Result<()> {
        if NAME_TABLE_LENGTH < i {
            Err(e::index_out_of_range(i))
        } else {
            self.tiles[i / WIDTH][i % WIDTH] = v;
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
            Ok(self.tiles[i / WIDTH][i % WIDTH])
        }
    }
}

#[test]
fn it_get() {
    let mut name = NameTable {
        tiles: [[0; WIDTH]; HEIGHT],
    };
    name.tiles[1][2] = 1;
    assert_eq!(name.get(WIDTH + 2).unwrap(), 1);
}

#[test]
fn it_put() {
    let mut name = NameTable {
        tiles: [[0; WIDTH]; HEIGHT],
    };
    name.put(WIDTH + 2, 1).unwrap();
    assert_eq!(name.tiles[1][2], 1)
}
