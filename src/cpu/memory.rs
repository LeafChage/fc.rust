use crate::memory::{Len, RAM, ROM};
use crate::result::{e, Result};
use std::cell::RefCell;
use std::rc::Rc;

pub struct MemoryMap<PROM, WRAM, PPU>
where
    PROM: ROM<usize>,
    WRAM: RAM<usize>,
    PPU: RAM<usize>,
{
    ppu_bus: Rc<RefCell<PPU>>,
    program_bus: PROM,
    wram_bus: WRAM,
}

impl<PROM, WRAM, PPU> MemoryMap<PROM, WRAM, PPU>
where
    PROM: ROM<usize>,
    WRAM: RAM<usize>,
    PPU: RAM<usize>,
{
    pub fn new(ppu_bus: Rc<RefCell<PPU>>, program_bus: PROM, wram_bus: WRAM) -> Self {
        MemoryMap {
            ppu_bus,
            program_bus,
            wram_bus,
        }
    }
}

impl<PROM, WRAM, PPU> std::fmt::Display for MemoryMap<PROM, WRAM, PPU>
where
    PROM: ROM<usize>,
    WRAM: RAM<usize>,
    PPU: RAM<usize> + std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "ppu_bus: \n {}", &self.ppu_bus.borrow())
    }
}

impl<PROM, WRAM, PPU> ROM<[usize; 2]> for MemoryMap<PROM, WRAM, PPU>
where
    PROM: ROM<usize, Output = u8>,
    WRAM: RAM<usize>,
    PPU: RAM<usize>,
{
    type Output = u16;

    fn get(&self, i: [usize; 2]) -> Result<Self::Output> {
        if 0xFFFF > i[0] && 0xFFFF > i[1] {
            Ok(u16::from_le_bytes([self.get(i[0])?, self.get(i[1])?]))
        } else {
            Err(e::index_out_of_range(i))
        }
    }
}

impl<PROM, WRAM, PPU> ROM<usize> for MemoryMap<PROM, WRAM, PPU>
where
    PROM: ROM<usize, Output = u8>,
    WRAM: RAM<usize>,
    PPU: RAM<usize>,
{
    type Output = u8;
    fn get(&self, i: usize) -> Result<Self::Output> {
        match i {
            // wram: [u8; 0x07FF - 0x0000],
            _ if (0x0000..=0x07FF).contains(&i) => self.wram_bus.get(i),
            // wram_mirror: [u8; 0x1FFF - 0x0800],
            _ if (0x0800..=0x1FFF).contains(&i) => Err(e::unimplemented()),
            // ppu_register: [u8; 0x2007 - 0x2000],
            _ if (0x2000..=0x2007).contains(&i) => self.ppu_bus.borrow().get(i - 0x2000),
            // ppu_register_mirror: [u8; 0x3fff - 0x2008],
            _ if (0x2008..=0x3FFF).contains(&i) => Err(e::unimplemented()),
            // apu: [u8; 0x401F - 0x4000],
            _ if (0x4000..=0x401D).contains(&i) => Err(e::unimplemented()),
            // exrom: [u8; 0x5FFF - 0x4020],
            _ if (0x4020..=0x5FFF).contains(&i) => Err(e::unimplemented()),
            // exram: [u8; 0x7FFF - 0x6000],
            _ if (0x6000..=0x7FFF).contains(&i) => Err(e::unimplemented()),
            // rom: [u8; 0xFFFF - 0x8000],
            _ if (0x8000..=0xFFFF).contains(&i) => self.program_bus.get(i - 0x8000),
            _ => dbg!(Err(e::index_out_of_range(i))),
        }
    }
}

impl<PROM, WRAM, PPU> RAM<usize> for MemoryMap<PROM, WRAM, PPU>
where
    PROM: ROM<usize, Output = u8>,
    WRAM: RAM<usize>,
    PPU: RAM<usize>,
{
    fn put(&mut self, i: usize, v: u8) -> Result<()> {
        match i {
            _ if (0x0000..=0x07FF).contains(&i) => self.wram_bus.put(i, v),
            _ if (0x0800..=0x1FFF).contains(&i) => Err(e::readonly(i)),
            _ if (0x2000..=0x2007).contains(&i) => self.ppu_bus.borrow_mut().put(i - 0x2000, v),
            _ if (0x2008..=0x3FFF).contains(&i) => Err(e::unimplemented()),
            _ if (0x4000..=0x401D).contains(&i) => Err(e::unimplemented()),
            _ if (0x4020..=0x5FFF).contains(&i) => Err(e::unimplemented()),
            _ if (0x6000..=0x7FFF).contains(&i) => Err(e::unimplemented()),
            _ if (0x8000..=0xFFFF).contains(&i) => Err(e::readonly(i)),
            _ => {
                dbg!(Err(e::index_out_of_range(i)))
            }
        }
    }
}
