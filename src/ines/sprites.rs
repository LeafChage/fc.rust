use super::sprite_byte::SpriteByte;

pub struct Sprites;

const SPRITE_LENGTH_B: usize = 16;

impl Sprites {
    pub fn parse(rom: &[u8]) -> Vec<Sprite> {
        let mut result = Vec::with_capacity(rom.len() / SPRITE_LENGTH_B);
        for i in 0..(rom.len() / SPRITE_LENGTH_B) {
            let from = i * SPRITE_LENGTH_B;
            let to = from + SPRITE_LENGTH_B;
            let sprite = Sprite::parse(rom[from..to].try_into().unwrap());
            result.push(sprite);
        }
        result
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sprite {
    raw: [[u8; 8]; 8],
}

impl Sprite {
    pub fn parse(data: &[u8; SPRITE_LENGTH_B]) -> Sprite {
        let a = data[0..8].iter().map(|v| SpriteByte::from(v.clone()));
        let b = data[8..16].iter().map(|v| SpriteByte::from(v.clone()));
        let sprites = std::iter::zip(a, b)
            .map(|(v1, v2)| v1 + v2)
            .map(|s| <[u8; 8]>::from(s))
            .collect::<Vec<[u8; 8]>>();
        Sprite {
            raw: sprites.try_into().unwrap(),
        }
    }
}

pub mod debug {
    extern crate image;
    use super::*;
    use image::{ImageBuffer, Rgb, RgbImage};

    const LENGTH: u32 = 8;
    const N: u32 = 20;
    pub fn create(sprites: Vec<Sprite>) {
        let mut i = 0;
        for s in sprites {
            let mut image: RgbImage = ImageBuffer::new(LENGTH * N, LENGTH * N);
            for w in 0..LENGTH {
                for h in 0..LENGTH {
                    let v = s.raw[h as usize][w as usize];
                    let v = match v {
                        0 => 0,
                        1 => 127,
                        _ => 255,
                    };
                    let p = Rgb([v, v, v]);

                    {
                        // image.put_pixel(w as u32, h as u32, p);
                        // w, hを基準に係数N分の範囲を塗りつぶす
                        let from = w * N;
                        let to = from + N;
                        for i in from..to {
                            let from = h * N;
                            let to = from + N;
                            for j in from..to {
                                image.put_pixel(i as u32, j as u32, p);
                            }
                        }
                    }
                }
            }
            image.save(format!("./{}.png", i)).unwrap();
            i += 1;
        }
    }
}
