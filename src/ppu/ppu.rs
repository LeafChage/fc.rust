use super::cycle::{Line, PPUCycle};
use super::memory::MemoryMap;
use super::register::Register;
use crate::display::Display;
use crate::memory::{RAM, ROM};
use crate::result::Result;
use crate::vec2::Vec2;
use std::cell::RefCell;
use std::rc::Rc;

pub struct PPU {
    cycle: PPUCycle,
    register: RefCell<Register>,
    memory: MemoryMap,
    display: Rc<RefCell<Display>>,
}

impl PPU {
    pub fn new(
        register: RefCell<Register>,
        memory: MemoryMap,
        display: Rc<RefCell<Display>>,
    ) -> Self {
        PPU {
            cycle: PPUCycle::default(),
            register,
            memory,
            display,
        }
    }

    pub fn handle<R, Fn>(&self, mut fun: Fn) -> Result<R>
    where
        Fn: FnMut(&mut Register, &dyn ROM<usize, Output = u8>) -> Result<R>,
    {
        fun(&mut self.register.borrow_mut(), &self.memory)
    }

    pub fn handle_mut<R, Fn>(&mut self, mut fun: Fn) -> Result<R>
    where
        Fn: FnMut(&mut Register, &mut (dyn RAM<usize, Output = u8, Input = u8>)) -> Result<R>,
    {
        fun(&mut self.register.borrow_mut(), &mut self.memory)
    }

    /// 縦も同様に240列しかないが、20はHBlankが発生する。
    /// この間PPURegisterの値をHBlank中に変更する。
    /// http://pgate1.at-ninja.jp/NES_on_FPGA/nes_ppu.htm
    pub fn exec(&mut self, cycle: usize) -> Result<bool> {
        if let Some(line) = self.cycle.add_cycle_with_next(cycle) {
            let Line(y) = line;
            let v = self.register.borrow().scroll_offset.older() as usize;
            let h = self.register.borrow().scroll_offset.later() as usize;
            let (name, attribute) = self.fetch_background_line(Vec2::new(v, h + y));
            for x in 0..32 {
                // ここでの ｙ の単位はspriteの単位で8倍になってしまっているので
                //     計算が狂うbyte単位の列に直すか、8倍のまま計算して、
                // VBlankの部分もやるなおすしかない
                let sprite = self.memory.sprite(name[x] as usize);
                if !sprite.zero() {
                    let palette = &self
                        .memory
                        .palette
                        .background_palette(attribute[x / 2] as usize);
                    self.display.borrow_mut().put_image(x, y, &sprite, &palette);
                } else {
                    self.display.borrow_mut().put_plane(
                        x,
                        y,
                        &self.memory.palette.background_color(),
                    );
                }
            }

            if line.is_last() {
                self.register.borrow_mut().toggle_hbrank(true);
            }
        }

        if self.cycle.has_drawed() {
            self.cycle.rewind();
            // TODO: 無理やりすぎる
            return Ok(true);
        }
        Ok(false)
    }

    /// | 0 | 1 |
    /// | 2 | 3 |
    fn fetch_background_line(&self, pos: Vec2<usize>) -> ([u8; 32], [u8; 16]) {
        let Vec2(x, y) = pos;
        if y < 30 {
            let (name0, attribute0) = self.memory.background0.fetch_line(pos, 32 - x);
            let (name1, attribute1) = self.memory.background1.fetch_line(Vec2::new(0, y), x);
            (
                [name0, name1].concat().try_into().unwrap(),
                [attribute0, attribute1].concat().try_into().unwrap(),
            )
        } else {
            let pos = pos - Vec2::new(0, 30);
            let Vec2(x, y) = pos;
            let (name2, attribute2) = self.memory.background2.fetch_line(pos, 32 - x);
            let (name3, attribute3) = self.memory.background3.fetch_line(Vec2::new(0, y), x);
            (
                [name2, name3].concat().try_into().unwrap(),
                [attribute2, attribute3].concat().try_into().unwrap(),
            )
        }
    }
}

impl std::fmt::Display for PPU {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.register.borrow())
    }
}
