use super::memory::MemoryMap;
use super::register::Register;
use crate::memory::{RAM, ROM};
use crate::result::Result;
use std::cell::RefCell;
use std::rc::Rc;

pub struct PPU {
    register: Rc<RefCell<Register>>,
    memory: MemoryMap,
}

impl PPU {
    pub fn new(register: Rc<RefCell<Register>>, memory: MemoryMap) -> Self {
        PPU { register, memory }
    }

    pub fn exec(&self) -> Result<()> {
        let a = self.register.borrow().get(0x2004)?;
        self.register.borrow_mut().put(0x2004, a)
        // todo!();
    }
}
