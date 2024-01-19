use binary::Byte;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Register {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub s: u8,
    pub p: StatusRegister,
    pub pc: u16,
}

impl Default for Register {
    fn default() -> Self {
        Register {
            a: 0,
            x: 0,
            y: 0,
            s: 0,
            p: StatusRegister::default(),
            pc: 0x8000, // TODO
        }
    }
}

impl std::fmt::Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "a: 0x{:02X} / x: 0x{:02X} / y: 0x{:02X}, s: 0x{:02X}, pc: 0x{:02X}",
            self.a, self.x, self.y, self.s, self.pc,
        )?;
        writeln!(f, "flags: ")?;
        writeln!(
            f,
            "  n: {} / o: {} / r: {} / b: {} / d: {} / i: {} / z: {} / c: {}",
            if self.p.n() { 1 } else { 0 },
            if self.p.v() { 1 } else { 0 },
            if self.p.r() { 1 } else { 0 },
            if self.p.b() { 1 } else { 0 },
            if self.p.d() { 1 } else { 0 },
            if self.p.i() { 1 } else { 0 },
            if self.p.z() { 1 } else { 0 },
            if self.p.c() { 1 } else { 0 }
        )
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct StatusRegister {
    negative: bool,
    overflow: bool,
    reserved: bool,
    breaked: bool,
    decimal: bool,
    interrupt: bool,
    zero: bool,
    carry: bool,
}

pub enum Flag {
    N,
    V,
    R,
    B,
    D,
    I,
    Z,
    C,
}

impl StatusRegister {
    pub fn n(&self) -> bool {
        self.negative
    }
    pub fn v(&self) -> bool {
        self.overflow
    }
    pub fn r(&self) -> bool {
        self.reserved
    }
    pub fn b(&self) -> bool {
        self.breaked
    }
    pub fn d(&self) -> bool {
        self.decimal
    }
    pub fn i(&self) -> bool {
        self.interrupt
    }
    pub fn z(&self) -> bool {
        self.zero
    }
    pub fn c(&self) -> bool {
        self.carry
    }

    pub fn toggle(&mut self, flag: Flag, v: bool) {
        match flag {
            Flag::N => self.negative = v,
            Flag::V => self.overflow = v,
            Flag::R => self.reserved = v,
            Flag::B => self.breaked = v,
            Flag::D => self.decimal = v,
            Flag::I => self.interrupt = v,
            Flag::Z => self.zero = v,
            Flag::C => self.carry = v,
        }
    }
    pub fn on(&mut self, flag: Flag) {
        self.toggle(flag, true);
    }

    pub fn off(&mut self, flag: Flag) {
        self.toggle(flag, false);
    }

    pub fn update_negative(&mut self, result: u8) {
        self.negative = result.bit(7);
    }

    pub fn update_overflow(&mut self, result: u8) {
        todo!();
    }

    pub fn update_reserved(&mut self, result: u8) {
        todo!();
    }

    pub fn update_break(&mut self, result: u8) {
        todo!();
    }

    pub fn update_decimal(&mut self, result: u8) {
        todo!();
    }

    pub fn update_interrupt(&mut self, result: u8) {
        todo!();
    }

    pub fn update_zero(&mut self, result: u8) {
        self.zero = result == 0;
    }

    pub fn update_carry(&mut self, result: u8) {
        todo!();
    }
}
