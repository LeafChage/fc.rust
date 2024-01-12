use crate::ines::ProgramRom;
use crate::memory::{RAM, ROM};
use crate::ppu;
use crate::result::{e, Result};
use std::cell::RefCell;
use std::rc::Rc;

pub struct MemoryMap<'a> {
    memory: [u8; 0xFFFF],
    // wram: [u8; 0x07FF - 0x0000],
    // wram_mirror: [u8; 0x1FFF - 0x0800],
    // ppu_register: [u8; 0x2007 - 0x2000],
    // ppu_register_mirror: [u8; 0x3fff - 0x2008],
    // apu: [u8; 0x401F - 0x4000],
    // exrom: [u8; 0x5FFF - 0x4020],
    // exram: [u8; 0x7FFF - 0x6000],
    // rom: [u8; 0xFFFF - 0x8000],
    ppu_register: Rc<RefCell<ppu::Register>>,
    program_rom: ProgramRom<'a>,
}

impl<'a> MemoryMap<'a> {
    pub fn new(ppur: Rc<RefCell<ppu::Register>>, program_rom: ProgramRom<'a>) -> Self {
        MemoryMap {
            memory: [0; 0xffff],
            ppu_register: ppur,
            program_rom,
        }
    }
}

impl<'a> std::fmt::Display for MemoryMap<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "ppu_register: \n {}", &self.ppu_register.borrow())
    }
}

impl<'a> ROM for MemoryMap<'a> {
    fn range(&self) -> std::ops::Range<usize> {
        0x0000..0xFFFF
    }

    fn get(&self, i: usize) -> Result<u8> {
        let r = self.ppu_register.borrow();

        match i {
            n if r.range().contains(&n) => r.get(n),
            n if (0x8000..0xFFFF).contains(&n) => self.program_rom.get(n - 0x8000),
            n if self.range().contains(&n) => Ok(self.memory[n as usize]),
            _ => dbg!(Err(e::index_out_of_range(i, self.range()))),
        }
    }
}

impl<'a> RAM for MemoryMap<'a> {
    fn put(&mut self, i: usize, v: u8) -> Result<()> {
        let mut r = self.ppu_register.borrow_mut();
        match i {
            n if r.range().contains(&n) => r.put(n, v)?,
            n if self.program_rom.range().contains(&n) => return Err(e::readonly(n)),
            n if 0 < n && n < 0xFFFF => {
                self.memory[n as usize] = v;
            }
            _ => {
                return dbg!(Err(e::index_out_of_range(i, self.range())));
            }
        }
        Ok(())
    }
}
