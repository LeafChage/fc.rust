use super::header::INesHeader;

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

    pub fn sprites(&self) -> Vec<u8> {
        self.raw[self.header.character_rom_range()].to_vec()
    }
}
