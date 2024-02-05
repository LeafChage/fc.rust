use crate::sprite::Sprite;

#[derive(Debug)]
pub struct SpriteROM {
    rom: Vec<Sprite>,
}

impl SpriteROM {
    #[cfg(debug_assertions)]
    pub fn raw(&self) -> &Vec<Sprite> {
        &self.rom
    }

    pub fn new<'a>(raw: &'a [u8]) -> Self {
        let mut rom = vec![];
        for i in 0..(raw.len() / 16) {
            let from = i * 16;
            let to = from + 16;
            let s = &raw[from..to];
            rom.push(Sprite::new(&s.try_into().unwrap()));
        }
        Self { rom }
    }
}

impl std::ops::Index<usize> for SpriteROM {
    type Output = Sprite;

    fn index(&self, index: usize) -> &Self::Output {
        &self.rom[index]
    }
}
