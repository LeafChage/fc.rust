use crate::display::{H, W};

#[derive(Debug, Clone, Copy)]
pub struct Line(pub usize);

impl Line {
    pub fn is_last(&self) -> bool {
        let Line(v) = self;
        // because line is index of array
        *v == (H / 8) - 1
    }
}

/// 縦も同様に240列しかないが、20はHBlankが発生する。
/// この間PPURegisterの値をHBlank中に変更する。
/// http://pgate1.at-ninja.jp/NES_on_FPGA/nes_ppu.htm
#[derive(Debug, Default)]
pub struct PPUCycle {
    cycle: usize,
}

/// Line: 256 + HBlank = 341
const HBLANK: usize = 341 - W;
const H_CYCLE: usize = HBLANK + W;

const VBLANK: usize = 20;
const V_CYCLE: usize = H + VBLANK;

impl PPUCycle {
    pub fn add_cycle_with_next(&mut self, cycle: usize) -> Option<Line> {
        let target_cycles = self.next_target_cycle();
        self.cycle += cycle;
        if target_cycles > self.cycle {
            None
        } else if self.is_vblank() {
            None
        } else {
            Some(Line(self.cycle_sprite_line(target_cycles)))
        }
    }

    fn cycle_sprite_line(&self, cycle: usize) -> usize {
        cycle / H_CYCLE / 8
    }

    fn next_target_cycle(&self) -> usize {
        let drawed_sprite_line = self.cycle / H_CYCLE / 8;
        H_CYCLE * ((drawed_sprite_line + 1) * 8)
    }

    pub fn is_vblank(&self) -> bool {
        self.cycle >= H_CYCLE * (V_CYCLE - VBLANK)
    }

    pub fn has_drawed(&self) -> bool {
        self.cycle > H_CYCLE * V_CYCLE
    }

    pub fn rewind(&mut self) {
        self.cycle -= H_CYCLE * V_CYCLE;
    }
}

