use super::header::INesHeader;
use super::SpriteROM;

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

    pub fn program(&self) -> &[u8] {
        &self.raw[self.header.program_rom_range()]
    }

    pub fn sprites(&self) -> SpriteROM {
        let raw = &self.raw[self.header.character_rom_range()];
        SpriteROM::new(raw)
    }
}

impl<'a> std::fmt::Display for INes<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.header)
    }
}
