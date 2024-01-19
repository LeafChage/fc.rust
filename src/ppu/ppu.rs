use super::register::Register;
use crate::display::Display;
use crate::memory::{RAM, ROM};
use crate::result::{e, Result};
use crate::sprite::Sprite;
use std::cell::RefCell;
use std::rc::Rc;

pub struct PPU<VRAM, CHROM>
where
    VRAM: RAM,
    CHROM: ROM<usize> + ROM<std::ops::Range<usize>, Output = Vec<u8>>,
{
    cycle: usize,
    register: RefCell<Register>,
    memory: VRAM,
    display: Rc<RefCell<Display>>,
    character_bus: CHROM,
}

impl<VRAM, CHROM> PPU<VRAM, CHROM>
where
    VRAM: RAM,
    CHROM: ROM<usize> + ROM<std::ops::Range<usize>, Output = Vec<u8>>,
{
    pub fn new(
        register: RefCell<Register>,
        memory: VRAM,
        display: Rc<RefCell<Display>>,
        character_bus: CHROM,
    ) -> Self {
        PPU {
            cycle: 0,
            register,
            memory,
            display,
            character_bus,
        }
    }

    pub fn exec(&mut self, cycle: usize) -> Result<usize> {
        self.cycle += cycle;
        if self.cycle < 341 * 240 {
            return Ok(self.cycle);
        }
        self.cycle = 0;
        for i in 0x2000..=0x23BF {
            let v = self.memory.get(i)? as usize;
            let s = self.character_bus.get((v * 16)..(v * 16 + 16))?;
            let sprite = Sprite::new(&s[0..16].try_into()?);
            let i = i - 0x2000;
            self.display.borrow_mut().put_image(i % 32, i / 32, &sprite);
        }
        Ok(0)
    }
}

impl<VRAM, CHROM> std::fmt::Display for PPU<VRAM, CHROM>
where
    VRAM: RAM,
    CHROM: ROM<usize> + ROM<std::ops::Range<usize>, Output = Vec<u8>>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.register.borrow())
    }
}

impl<VRAM, CHROM> ROM<usize> for PPU<VRAM, CHROM>
where
    VRAM: RAM,
    CHROM: ROM<usize> + ROM<std::ops::Range<usize>, Output = Vec<u8>>,
{
    type Output = u8;
    fn get(&self, index: usize) -> Result<Self::Output> {
        match index {
            0 => Err(e::writeonly(index)),
            1 => Err(e::writeonly(index)),
            2 => Err(e::unimplemented()),
            3 => Err(e::writeonly(index)),
            4 => Err(e::unimplemented()),
            5 => Err(e::writeonly(index)),
            6 => Err(e::writeonly(index)),
            7 => {
                let mut r = self.register.borrow_mut();
                let addr = r.ppu_addr();
                r.increment_ppu_addr();
                self.memory.get(addr as usize)
            }
            _ => Err(e::index_out_of_range(index)),
        }
    }
}

impl<VRAM, CHROM> RAM for PPU<VRAM, CHROM>
where
    VRAM: RAM,
    CHROM: ROM<usize> + ROM<std::ops::Range<usize>, Output = Vec<u8>>,
{
    fn put(&mut self, index: usize, v: u8) -> Result<()> {
        match index {
            0 => {}
            1 => {}
            2 => {
                return Err(e::readonly(index));
            }
            3 => {}
            4 => {}
            5 => {}
            6 => {
                self.register.borrow_mut().put_addr(v);
            }
            7 => {
                let addr = self.register.borrow().ppu_addr();
                let result = self.memory.put(addr as usize, v);
                self.register.borrow_mut().increment_ppu_addr();
                return result;
            }
            _ => {
                return Err(e::index_out_of_range(index));
            }
        }
        Ok(())
    }
}
