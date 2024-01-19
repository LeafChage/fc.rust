use crate::memory::{RAM, ROM};
use crate::result::{e, Result};
use std::cell::RefCell;
use std::rc::Rc;

pub struct MemoryMap<PROM, WRAM, PPU>
where
    PROM: ROM<usize>,
    WRAM: RAM,
    PPU: RAM,
{
    // wram: [u8; 0x07FF - 0x0000],
    // wram_mirror: [u8; 0x1FFF - 0x0800],
    // ppu_register: [u8; 0x2007 - 0x2000],
    // ppu_register_mirror: [u8; 0x3fff - 0x2008],
    // apu: [u8; 0x401F - 0x4000],
    // exrom: [u8; 0x5FFF - 0x4020],
    // exram: [u8; 0x7FFF - 0x6000],
    // rom: [u8; 0xFFFF - 0x8000],
    ppu_bus: Rc<RefCell<PPU>>,
    program_bus: PROM,
    wram_bus: WRAM,
}

impl<PROM, WRAM, PPU> MemoryMap<PROM, WRAM, PPU>
where
    PROM: ROM<usize>,
    WRAM: RAM,
    PPU: RAM,
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
    WRAM: RAM,
    PPU: RAM + std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "ppu_bus: \n {}", &self.ppu_bus.borrow())
    }
}

impl<PROM, WRAM, PPU> ROM<usize> for MemoryMap<PROM, WRAM, PPU>
where
    PROM: ROM<usize, Output = u8>,
    WRAM: RAM,
    PPU: RAM,
{
    type Output = u8;
    fn get(&self, i: usize) -> Result<Self::Output> {
        match i {
            _ if (0x0000..=0x07FF).contains(&i) => self.wram_bus.get(i),
            _ if (0x0800..=0x1FFF).contains(&i) => Err(e::unimplemented()),
            _ if (0x2000..=0x2007).contains(&i) => self.ppu_bus.borrow().get(i - 0x2000),
            _ if (0x2008..=0x3FFF).contains(&i) => Err(e::unimplemented()),
            _ if (0x4000..=0x401D).contains(&i) => Err(e::unimplemented()),
            _ if (0x4020..=0x5FFF).contains(&i) => Err(e::unimplemented()),
            _ if (0x6000..=0x7FFF).contains(&i) => Err(e::unimplemented()),
            _ if (0x8000..=0xFFFF).contains(&i) => self.program_bus.get(i - 0x8000),
            _ => dbg!(Err(e::index_out_of_range(i))),
        }
    }
}

impl<PROM, WRAM, PPU> RAM for MemoryMap<PROM, WRAM, PPU>
where
    PROM: ROM<usize, Output = u8>,
    WRAM: RAM,
    PPU: RAM,
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
