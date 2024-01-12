use crate::memory::{RAM, ROM};
use crate::result::{e, Result};

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Register {
    ppu_ctrl: u8,
    ppu_mask: u8,
    ppu_status: u8,
    oam_addr: u8,
    oam_data: u8,
    ppu_scroll: u8,
    ppu_addr: u8,
    ppu_data: u8,
}

impl ROM for Register {
    fn range(&self) -> std::ops::Range<usize> {
        0x2000..0x2008
    }

    fn get(&self, i: usize) -> Result<u8> {
        match i {
            0x2000 => Err(e::writeonly(i)),
            0x2001 => Err(e::writeonly(i)),
            0x2002 => Ok(self.ppu_status),
            0x2003 => Err(e::writeonly(i)),
            0x2004 => Ok(self.oam_data),
            0x2005 => Err(e::writeonly(i)),
            0x2006 => Err(e::writeonly(i)),
            0x2007 => Ok(self.ppu_data),
            _ => Err(e::index_out_of_range(i, self.range())),
        }
    }
}

impl std::fmt::Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PPUCTRL: {:02X?} / PPUMASK: {:02X?} / PPUSTATUS: {:02X?} / OAMADDR: {:02X?} / OAMDATA: {:02X?} / PPUSCROLL: {:02X?} / PPUADDR: {:02X?} / PPUDATA: {:02X?}",
            self.ppu_ctrl,
            self.ppu_mask,
            self.ppu_status,
            self.oam_addr,
            self.oam_data,
            self.ppu_scroll,
            self.ppu_addr,
            self.ppu_data,
            )
    }
}

impl RAM for Register {
    fn put(&mut self, i: usize, v: u8) -> Result<()> {
        match i {
            0x2000 => {
                self.ppu_ctrl = v;
            }
            0x2001 => {
                self.ppu_mask = v;
            }
            0x2002 => return Err(e::readonly(i)),
            0x2003 => {
                self.oam_addr = v;
            }
            0x2004 => {
                self.oam_data = v;
            }
            0x2005 => {
                self.ppu_scroll = v;
            }
            0x2006 => {
                self.ppu_addr = v;
            }
            0x2007 => {
                self.ppu_data = v;
            }
            _ => {
                return Err(e::index_out_of_range(i, self.range()));
            }
        }
        Ok(())
    }
}
