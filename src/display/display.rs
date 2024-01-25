use crate::ppu;
use crate::sprite::Sprite;
use macroquad::prelude::{color_u8, Color, Image, BLACK};

pub const W: usize = 256;
pub const H: usize = 240;

pub struct Display {
    pub image: Image,
}

impl Default for Display {
    fn default() -> Self {
        let image = Image::gen_image_color(W as u16, H as u16, BLACK);
        Display { image }
    }
}

impl Display {
    pub fn put_image(&mut self, x: usize, y: usize, sprite: &Sprite, palette: &ppu::Palette) {
        if !((0..32).contains(&x) && (0..30).contains(&y)) {
            panic!("out of range, {}, {}", x, y);
        }

        let sprite_bits = sprite.bits();
        for iy in 0..8 {
            for ix in 0..8 {
                let ty = y * 8 + iy;
                let tx = x * 8 + ix;

                let c = match sprite_bits[iy][ix].clone() {
                    0 => palette.0,
                    1 => palette.1,
                    2 => palette.2,
                    3 => palette.3,
                    _ => unreachable!(),
                };

                self.image
                    .set_pixel(tx as u32, ty as u32, color_u8!(c.r, c.g, c.b, 255.0));
            }
        }
    }

    pub fn put_plane(&mut self, x: usize, y: usize, color: &ppu::Color) {
        self.put_image(
            x,
            y,
            &Sprite::default(),
            &ppu::Palette(color.clone(), color.clone(), color.clone(), color.clone()),
        )
    }
}
