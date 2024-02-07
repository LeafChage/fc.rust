use crate::memory::{RAM, ROM, WOM};
use crate::result::{e, Result};
use std::cell::RefCell;
use std::rc::Rc;

pub struct MemoryMap<EXROM, EXRAM, PROM, APU, WRAM, PPU>
where
    EXROM: ROM<usize, Output = u8>,
    EXRAM: RAM<usize, Output = u8, Input = u8>,
    APU: RAM<usize, Output = u8, Input = u8>,
    PROM: ROM<usize, Output = u8>,
    WRAM: RAM<usize, Output = u8, Input = u8>,
    PPU: RAM<usize, Output = u8, Input = u8>,
{
    exrom: EXROM,
    exram: EXRAM,
    ppu_bus: Rc<RefCell<PPU>>,
    apu: APU,
    program_bus: PROM,
    wram_bus: WRAM,
}

impl<EXRAM, EXROM, APU, PROM, WRAM, PPU> MemoryMap<EXROM, EXRAM, PROM, APU, WRAM, PPU>
where
    EXROM: ROM<usize, Output = u8>,
    EXRAM: RAM<usize, Output = u8, Input = u8>,
    APU: RAM<usize, Output = u8, Input = u8>,
    PROM: ROM<usize, Output = u8>,
    WRAM: RAM<usize, Output = u8, Input = u8>,
    PPU: RAM<usize, Output = u8, Input = u8>,
{
    pub fn new(
        ppu_bus: Rc<RefCell<PPU>>,
        program_bus: PROM,
        wram_bus: WRAM,
        apu: APU,
        exram: EXRAM,
        exrom: EXROM,
    ) -> Self {
        MemoryMap {
            ppu_bus,
            program_bus,
            apu,
            wram_bus,
            exrom,
            exram,
        }
    }
}

impl<EXROM, EXRAM, PROM, APU, WRAM, PPU> std::fmt::Display
    for MemoryMap<EXROM, EXRAM, PROM, APU, WRAM, PPU>
where
    EXROM: ROM<usize, Output = u8>,
    EXRAM: RAM<usize, Output = u8, Input = u8>,
    APU: RAM<usize, Output = u8, Input = u8>,
    PROM: ROM<usize, Output = u8>,
    WRAM: RAM<usize, Output = u8, Input = u8>,
    PPU: RAM<usize, Output = u8, Input = u8> + std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "ppu_bus: \n {}", &self.ppu_bus.borrow())
    }
}

impl<EXROM, EXRAM, PROM, APU, WRAM, PPU> RAM<usize>
    for MemoryMap<EXROM, EXRAM, PROM, APU, WRAM, PPU>
where
    EXROM: ROM<usize, Output = u8>,
    EXRAM: RAM<usize, Output = u8, Input = u8>,
    APU: RAM<usize, Output = u8, Input = u8>,
    PROM: ROM<usize, Output = u8>,
    WRAM: RAM<usize, Output = u8, Input = u8>,
    PPU: RAM<usize, Output = u8, Input = u8>,
{
}

impl<EXROM, EXRAM, PROM, APU, WRAM, PPU> ROM<[usize; 2]>
    for MemoryMap<EXROM, EXRAM, PROM, APU, WRAM, PPU>
where
    EXROM: ROM<usize, Output = u8>,
    EXRAM: RAM<usize, Output = u8, Input = u8>,
    APU: RAM<usize, Output = u8, Input = u8>,
    PROM: ROM<usize, Output = u8>,
    WRAM: RAM<usize, Output = u8, Input = u8>,
    PPU: RAM<usize, Output = u8, Input = u8>,
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

impl<EXROM, EXRAM, PROM, APU, WRAM, PPU> ROM<usize>
    for MemoryMap<EXROM, EXRAM, PROM, APU, WRAM, PPU>
where
    EXROM: ROM<usize, Output = u8>,
    EXRAM: RAM<usize, Output = u8, Input = u8>,
    APU: RAM<usize, Output = u8, Input = u8>,
    PROM: ROM<usize, Output = u8>,
    WRAM: RAM<usize, Output = u8, Input = u8>,
    PPU: RAM<usize, Output = u8, Input = u8>,
{
    type Output = u8;

    fn get(&self, i: usize) -> Result<Self::Output> {
        match i {
            // wram: [u8; 0x07FF - 0x0000],
            _ if (0x0000..=0x07FF).contains(&i) => self.wram_bus.get(i),
            // wram_mirror: [u8; 0x1FFF - 0x0800],
            _ if (0x0800..=0x1FFF).contains(&i) => self.wram_bus.get(i - 0x0800),
            // ppu_register: [u8; 0x2007 - 0x2000],
            _ if (0x2000..=0x2007).contains(&i) => self.ppu_bus.borrow().get(i - 0x2000),
            // ppu_register_mirror: [u8; 0x3fff - 0x2008],
            _ if (0x2008..=0x3FFF).contains(&i) => self.ppu_bus.borrow().get(i - 0x2008),
            // apu: [u8; 0x401F - 0x4000],
            _ if (0x4000..=0x401D).contains(&i) => self.apu.get(i - 0x4000),
            // exrom: [u8; 0x5FFF - 0x4020],
            _ if (0x4020..=0x5FFF).contains(&i) => self.exrom.get(i - 0x4020),
            // exram: [u8; 0x7FFF - 0x6000],
            _ if (0x6000..=0x7FFF).contains(&i) => self.exram.get(i - 0x6000),
            // rom: [u8; 0xFFFF - 0x8000],
            _ if (0x8000..=0xFFFF).contains(&i) => self.program_bus.get(i - 0x8000),
            _ => dbg!(Err(e::index_out_of_range(i))),
        }
    }
}

impl<EXROM, EXRAM, PROM, APU, WRAM, PPU> WOM<usize>
    for MemoryMap<EXROM, EXRAM, PROM, APU, WRAM, PPU>
where
    EXROM: ROM<usize, Output = u8>,
    EXRAM: RAM<usize, Output = u8, Input = u8>,
    APU: RAM<usize, Output = u8, Input = u8>,
    PROM: ROM<usize, Output = u8>,
    WRAM: RAM<usize, Output = u8, Input = u8>,
    PPU: RAM<usize, Output = u8, Input = u8>,
{
    type Input = u8;
    fn put(&mut self, i: usize, v: u8) -> Result<()> {
        match i {
            _ if (0x0000..=0x07FF).contains(&i) => self.wram_bus.put(i, v),
            _ if (0x0800..=0x1FFF).contains(&i) => Err(e::readonly(i)),
            _ if (0x2000..=0x2007).contains(&i) => self.ppu_bus.borrow_mut().put(i - 0x2000, v),
            _ if (0x2008..=0x3FFF).contains(&i) => Err(e::unimplemented()),
            _ if (0x4000..=0x401D).contains(&i) => self.apu.put(i - 0x4000, v),
            _ if (0x4020..=0x5FFF).contains(&i) => Err(e::readonly(i)),
            _ if (0x6000..=0x7FFF).contains(&i) => self.exram.put(i - 0x6000, v),
            _ if (0x8000..=0xFFFF).contains(&i) => Err(e::readonly(i)),
            _ => {
                dbg!(Err(e::index_out_of_range(i)))
            }
        }
    }
}
