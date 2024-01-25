use super::color::{Color, COLORS};
use crate::memory::{RAM, ROM, WOM};
use crate::result::{e, Result};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Palette(pub Color, pub Color, pub Color, pub Color);

#[derive(Default, Debug)]
pub struct PaletteTable {
    raw: [u8; 0x20],
}

impl RAM<usize> for PaletteTable {}

impl WOM<usize> for PaletteTable {
    type Input = u8;
    fn put(&mut self, i: usize, v: Self::Input) -> Result<()> {
        if self.raw.len() < i {
            Err(e::index_out_of_range(i))
        } else {
            self.raw[i] = v;
            Ok(())
        }
    }
}

impl ROM<usize> for PaletteTable {
    type Output = u8;

    fn get(&self, i: usize) -> Result<Self::Output> {
        if self.raw.len() < i {
            Err(e::index_out_of_range(i))
        } else {
            Ok(self.raw[i])
        }
    }
}

impl PaletteTable {
    pub fn background_color(&self) -> Color {
        COLORS[self.raw[0x00] as usize]
    }

    pub fn background_palette(&self, i: usize) -> Palette {
        match i {
            0 => self.background_palette0(),
            1 => self.background_palette1(),
            2 => self.background_palette2(),
            3 => self.background_palette3(),
            _ => unreachable!(),
        }
    }
    fn background_palette0(&self) -> Palette {
        Palette(
            self.background_color(),
            COLORS[self.raw[0x01] as usize],
            COLORS[self.raw[0x02] as usize],
            COLORS[self.raw[0x03] as usize],
        )
    }
    fn background_palette1(&self) -> Palette {
        Palette(
            self.background_color(),
            COLORS[self.raw[0x05] as usize],
            COLORS[self.raw[0x06] as usize],
            COLORS[self.raw[0x07] as usize],
        )
    }
    fn background_palette2(&self) -> Palette {
        Palette(
            self.background_color(),
            COLORS[self.raw[0x09] as usize],
            COLORS[self.raw[0x0A] as usize],
            COLORS[self.raw[0x0B] as usize],
        )
    }
    fn background_palette3(&self) -> Palette {
        Palette(
            self.background_color(),
            COLORS[self.raw[0x0D] as usize],
            COLORS[self.raw[0x0E] as usize],
            COLORS[self.raw[0x0F] as usize],
        )
    }

    pub fn sprite_palette0(&self) -> Palette {
        Palette(
            COLORS[self.raw[0x11] as usize],
            COLORS[self.raw[0x12] as usize],
            COLORS[self.raw[0x13] as usize],
            COLORS[self.raw[0x04] as usize], // mirror
        )
    }
    pub fn sprite_palette1(&self) -> Palette {
        Palette(
            COLORS[self.raw[0x15] as usize],
            COLORS[self.raw[0x16] as usize],
            COLORS[self.raw[0x17] as usize],
            COLORS[self.raw[0x08] as usize], // mirror
        )
    }
    pub fn sprite_palette2(&self) -> Palette {
        Palette(
            COLORS[self.raw[0x19] as usize],
            COLORS[self.raw[0x1A] as usize],
            COLORS[self.raw[0x1B] as usize],
            COLORS[self.raw[0x0C] as usize], // mirror
        )
    }
    pub fn sprite_palette3(&self) -> Palette {
        Palette(
            COLORS[self.raw[0x1D] as usize],
            COLORS[self.raw[0x1E] as usize],
            COLORS[self.raw[0x1F] as usize],
            self.background_color(), // TODO: よくわかってない
        )
    }
}
