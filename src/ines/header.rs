#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct INesHeader {
    /// Constant $4E $45 $53 $1A (ASCII "NES" followed by MS-DOS end-of-file)
    magic: [u8; 4],
    /// Size of PRG ROM in 16 KB units
    program_rom_unit_count: u8,
    /// Size of CHR ROM in 8 KB units (value 0 means the board uses CHR RAM)
    character_rom_unit_count: u8,
    /// Flags 6 – Mapper, mirroring, battery, trainer
    flag6: u8,
    /// Flags 7 – Mapper, VS/Playchoice, NES 2.0
    flag7: u8,
    /// Flags 8 – PRG-RAM size (rarely used extension)
    flag8: u8,
    /// Flags 9 – TV system (rarely used extension)
    flag9: u8,
    /// Flags 10 – TV system, PRG-RAM presence (unofficial, rarely used extension)
    flag10: u8,
    /// Unused padding (should be filled with zero, but some rippers put their name across bytes 7-15)
    padding: [u8; 5],
}

const MAGIC: [u8; 4] = [0x4E, 0x45, 0x53, 0x1A];
const PROGRAM_ROM_UNIT_SIZE: usize = 16384;
const CHARACTER_ROM_UNIT_SIZE: usize = 8192;

impl INesHeader {
    pub const INES_HEADER_LENGTH: usize = 16;

    pub fn parser(data: &[u8]) -> anyhow::Result<INesHeader, anyhow::Error> {
        if !(data.len() > Self::INES_HEADER_LENGTH
            && data[0..4] == MAGIC
            && data[11..16] == [0u8; 5])
        {
            return Err(anyhow::anyhow!("not ines format"));
        }

        Ok(INesHeader {
            magic: MAGIC,
            program_rom_unit_count: data[4],
            character_rom_unit_count: data[5],
            flag6: data[6],
            flag7: data[7],
            flag8: data[8],
            flag9: data[9],
            flag10: data[10],
            padding: [0; 5],
        })
    }

    pub fn program_rom_range(&self) -> std::ops::Range<usize> {
        let from = Self::INES_HEADER_LENGTH;
        let to = from + self.program_rom_size();
        from..to
    }

    pub fn character_rom_range(&self) -> std::ops::Range<usize> {
        let from = self.program_rom_range().end;
        let to = from + self.character_rom_size();
        from..to
    }

    fn program_rom_size(&self) -> usize {
        self.program_rom_unit_count as usize * PROGRAM_ROM_UNIT_SIZE
    }

    fn character_rom_size(&self) -> usize {
        self.character_rom_unit_count as usize * CHARACTER_ROM_UNIT_SIZE
    }
}
