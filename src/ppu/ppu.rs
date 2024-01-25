use super::memory::MemoryMap;
use super::register::Register;
use crate::display::Display;
use crate::memory::{RAM, ROM};
use crate::rect::Rect;
use crate::result::Result;
use crate::vec2::Vec2;
use std::cell::RefCell;
use std::rc::Rc;

pub struct PPU {
    cycle: usize,
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
            cycle: 0,
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

    pub fn exec(&mut self, cycle: usize) -> Result<usize> {
        self.cycle += cycle;
        if self.cycle < 341 * 240 {
            return Ok(self.cycle);
        }
        self.cycle = 0;

        let v = self.register.borrow().scroll_offset.older() as usize;
        let h = self.register.borrow().scroll_offset.later() as usize;
        let (name, attribute) = self.fetch_background(Rect::from_pos_size(
            Vec2::new(h, v),
            Vec2::new(32 + h, 30 + v),
        ));
        for y in 0..30 {
            for x in 0..32 {
                let sprite = self.memory.sprite(name[y][x] as usize);
                if !sprite.zero() {
                    let palette = &self
                        .memory
                        .palette
                        .background_palette(attribute[y / 2][x / 2] as usize);
                    self.display.borrow_mut().put_image(x, y, &sprite, &palette);
                } else {
                    self.display.borrow_mut().put_plane(
                        x,
                        y,
                        &self.memory.palette.background_color(),
                    );
                }
            }
        }
        Ok(0)
    }

    /// | rect0 | rect1 |
    /// | rect2 | rect3 |
    fn fetch_background(&self, rect: Rect) -> ([[u8; 32]; 30], [[u8; 16]; 15]) {
        let pos = rect.pos();
        let Vec2(x, y) = pos;
        let rect0 = Rect::from_pos2(pos, Vec2::new(32, 30));
        let rect1 = Rect::from_pos2(Vec2::new(0, y), Vec2::new(x, 30));
        let rect2 = Rect::from_pos2(Vec2::new(x, 0), Vec2::new(32, y));
        let rect3 = Rect::from_pos2(Vec2::zero(), pos);

        let (name0, attribute0) = self.memory.background0.fetch(rect0);
        let (name1, attribute1) = self.memory.background1.fetch(rect1);
        let (name2, attribute2) = self.memory.background2.fetch(rect2);
        let (name3, attribute3) = self.memory.background3.fetch(rect3);

        let mut name = [[0u8; 32]; 30];
        for y in 0..30 {
            for x in 0..32 {
                if name0.len() > y {
                    if name0[y].len() > x {
                        name[y][x] = name0[y][x];
                    } else {
                        name[y][x] = name1[y][x - name[y].len()];
                    }
                } else {
                    if name2[y].len() > x {
                        name[y][x] = name2[y - name0.len()][x];
                    } else {
                        name[y][x] = name3[y - name0.len()][x - name2[y].len()];
                    }
                }
            }
        }

        let mut attribute = [[0u8; 16]; 15];
        for y in 0..15 {
            for x in 0..16 {
                if name0.len() > y {
                    if name0[y].len() > x {
                        name[y][x] = name0[y][x];
                    } else {
                        name[y][x] = name1[y][x - name[y].len()];
                    }
                } else {
                    if name2[y].len() > x {
                        name[y][x] = name2[y - name0.len()][x];
                    } else {
                        name[y][x] = name3[y - name0.len()][x - name2[y].len()];
                    }
                }
            }
        }
        (name, attribute)
    }
}

impl std::fmt::Display for PPU {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.register.borrow())
    }
}
