use crate::sprite::Sprite;
use macroquad::prelude::{color_u8, Color, Image, BLACK};

pub const W: usize = 256;
pub const H: usize = 240;

pub struct Display {
    raw: Vec<Vec<u8>>,
    pub image: Image,
}

impl Default for Display {
    fn default() -> Self {
        let raw = vec![vec![0u8; W]; H];
        let image = Image::gen_image_color(W as u16, H as u16, BLACK);
        Display { raw, image }
    }
}

impl Display {
    pub fn put_image(&mut self, x: usize, y: usize, sprite: &Sprite) {
        if !((0..32).contains(&x) && (0..30).contains(&y)) {
            panic!("out of range, {}, {}", x, y);
        }

        let sprite_bits = sprite.bits();
        for iy in 0..8 {
            for ix in 0..8 {
                let ty = y * 8 + iy;
                let tx = x * 8 + ix;
                self.raw[ty][tx] = sprite_bits[iy][ix].clone();

                let c = match sprite_bits[iy][ix].clone() {
                    0 => 0,
                    1 => 127,
                    _ => 255,
                };

                self.image
                    .set_pixel(tx as u32, ty as u32, color_u8!(c, c, c, 255.0));
            }
        }
    }
}

#[rustfmt::skip]
#[test]
fn it_put_image() {
    let mut display = Display {
        raw: vec![vec![0u8; 16]; 16],
        image: Image::empty(),
    };
    display.put_image(
        1,
        1,
        &Sprite::debug_new([
            [1,  2,  3,  4,  5,  6,  7,  8 ],
            [11, 12, 13, 14, 15, 16, 17, 18],
            [21, 22, 23, 24, 25, 26, 27, 28],
            [31, 32, 33, 34, 35, 36, 37, 38],
            [41, 42, 43, 44, 45, 46, 47, 48],
            [51, 52, 53, 54, 55, 56, 57, 58],
            [61, 62, 63, 64, 65, 66, 67, 68],
            [71, 72, 73, 74, 75, 76, 77, 78],
            ]));
    assert_eq!(
        display.raw,
        vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0, 1,  2,  3,  4,  5,  6,  7,  8 ],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 11, 12, 13, 14, 15, 16, 17, 18],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 21, 22, 23, 24, 25, 26, 27, 28],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 31, 32, 33, 34, 35, 36, 37, 38],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 41, 42, 43, 44, 45, 46, 47, 48],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 51, 52, 53, 54, 55, 56, 57, 58],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 61, 62, 63, 64, 65, 66, 67, 68],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 71, 72, 73, 74, 75, 76, 77, 78],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0,  0,  0,  0,  0,  0,  0,  0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0,  0,  0,  0,  0,  0,  0,  0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0,  0,  0,  0,  0,  0,  0,  0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0,  0,  0,  0,  0,  0,  0,  0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0,  0,  0,  0,  0,  0,  0,  0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0,  0,  0,  0,  0,  0,  0,  0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0,  0,  0,  0,  0,  0,  0,  0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0,  0,  0,  0,  0,  0,  0,  0],
        ]
    )
}
