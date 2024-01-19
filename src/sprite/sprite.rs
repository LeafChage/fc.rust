use super::sprite_byte::SpriteByte;

pub const SPRITE_LENGTH: usize = 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sprite {
    raw: [[u8; SPRITE_LENGTH]; SPRITE_LENGTH],
}

impl Sprite {
    pub fn new(value: &[u8; SPRITE_LENGTH * 2]) -> Self {
        let a = value[0..8].iter().map(|v| SpriteByte::from(v.clone()));
        let b = value[8..16].iter().map(|v| SpriteByte::from(v.clone()));
        let sprites = std::iter::zip(a, b)
            .map(|(v1, v2)| v1 + v2)
            .map(|s| <[u8; 8]>::from(s))
            .collect::<Vec<[u8; 8]>>();
        Sprite {
            raw: sprites.try_into().unwrap(),
        }
    }

    pub fn zero(&self) -> bool {
        self.raw
            .iter()
            .filter(|line| line.iter().filter(|v| v != &&0).count() != 0)
            .count()
            == 0
    }

    pub fn bits(&self) -> [[u8; SPRITE_LENGTH]; SPRITE_LENGTH] {
        self.raw
    }

    #[cfg(test)]
    pub fn debug_new(raw: [[u8; SPRITE_LENGTH]; SPRITE_LENGTH]) -> Self {
        Self { raw }
    }
}
