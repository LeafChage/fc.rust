use super::header::INesHeader;
use super::program::ProgramRom;
// use crate::result::Result;

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

    pub fn program(&self) -> ProgramRom<'a> {
        ProgramRom::new(&self.raw[self.header.program_rom_range()])
    }

    // pub fn sprites(&self, header: &INesHeader) -> anyhow::Result<Vec<Sprite>, anyhow::Error> {
    //     let sprites_rom = &self.data[header.character_rom_range()];
    //     Ok(Sprites::parse(sprites_rom))
    // }
}
