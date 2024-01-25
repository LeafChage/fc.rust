use super::PPU;
use crate::memory::{RAM, ROM, WOM};
use crate::result::{e, Result};

impl RAM<usize> for PPU {}

impl ROM<usize> for PPU {
    type Output = u8;
    fn get(&self, index: usize) -> Result<Self::Output> {
        match index {
            0 => Err(e::writeonly(index)),
            1 => Err(e::writeonly(index)),
            2 => self.handle(|register, _| {
                let status = register.status;
                register.scroll_offset.clear();
                Ok(status)
            }),
            3 => Err(e::writeonly(index)),
            4 => Err(e::unimplemented()),
            5 => Err(e::writeonly(index)),
            6 => Err(e::writeonly(index)),
            7 => self.handle(|register, memory| {
                let addr = register.ppu_addr();
                register.increment_ppu_addr();
                memory.get(addr as usize)
            }),
            _ => Err(e::index_out_of_range(index)),
        }
    }
}

impl WOM<usize> for PPU {
    type Input = u8;
    fn put(&mut self, index: usize, v: u8) -> Result<()> {
        match index {
            0 => self.handle(|register, _| {
                register.control1 = v;
                Ok(())
            }),
            1 => self.handle(|register, _| {
                register.control2 = v;
                Ok(())
            }),
            2 => Err(e::readonly(index)),
            3 => self.handle(|register, _| {
                register.sprite_addr = v;
                Ok(())
            }),
            4 => self.handle(|register, _| {
                register.sprite_data = v;
                register.sprite_addr += 1;
                Ok(())
            }),
            5 => self.handle(|register, _| {
                register.put_scroll_offset(v);
                Ok(())
            }),
            6 => self.handle(|register, _| {
                register.put_addr(v);
                Ok(())
            }),
            7 => self.handle_mut(|register, memory| {
                let addr = register.ppu_addr();
                memory.put(addr as usize, v)?;
                register.increment_ppu_addr();
                Ok(())
            }),
            _ => Err(e::index_out_of_range(index)),
        }
    }
}
