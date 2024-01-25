use super::header::INesHeader;
use crate::sprite::Sprite;

#[derive(Debug)]
pub struct INes<'a> {
    raw: &'a [u8],
    header: INesHeader,
}

impl<'a> INes<'a> {
    pub fn parse(raw: &'a [u8]) -> anyhow::Result<INes<'a>, anyhow::Error> {
        let header = INesHeader::parser(&raw)?;
        Ok(INes { header, raw })
    }

    pub fn program(&self) -> Vec<u8> {
        self.raw[self.header.program_rom_range()].to_vec()
    }

    pub fn sprites(&self) -> Vec<Sprite> {
        let rom = &self.raw[self.header.character_rom_range()];
        let mut v = vec![];
        for i in 0..(rom.len() / 16) {
            let from = i * 16;
            let to = from + 16;
            let s = &rom[from..to];
            v.push(Sprite::new(&s.try_into().unwrap()));
        }
        v
    }
}

impl<'a> std::fmt::Display for INes<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.header)
    }
}
