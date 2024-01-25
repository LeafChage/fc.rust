mod sprite;
mod sprite_byte;

pub use sprite::Sprite;
pub use sprite_byte::SpriteByte;

pub fn debug_sprite(sprites: Vec<Sprite>) {
    let mut image: image::RgbImage = image::ImageBuffer::new(256 * 2, 240 * 2);
    let sprite_per_line = 256 / 8;

    let mut i = 0;
    for sprite in sprites.iter() {
        let sprite_bits = sprite.bits();
        for iy in 0..8 {
            for ix in 0..8 {
                let c = match sprite_bits[iy][ix].clone() {
                    0 => 0,
                    1 => 127,
                    _ => 255,
                };

                let ty = (i / sprite_per_line) * 8 + iy;
                let tx = (i % sprite_per_line) * 8 + ix;
                image.put_pixel(tx as u32, ty as u32, image::Rgb([c, c, c]));
            }
        }
        i += 1;
    }
    image.save("./tmp.png").unwrap();
}
