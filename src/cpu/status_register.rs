use binary::Byte;
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

pub enum SFlag {
    N,
    V,
    R,
    B,
    D,
    I,
    Z,
    C,
}

impl From<StatusRegister> for u8 {
    fn from(value: StatusRegister) -> Self {
        let result = 0u8;
        result.set(7, value.n());
        result.set(6, value.v());
        result.set(6, value.r());
        result.set(6, value.b());
        result.set(6, value.d());
        result.set(6, value.i());
        result.set(6, value.z());
        result.set(6, value.c());
        result
    }
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

    pub fn toggle(&mut self, flag: SFlag, v: bool) {
        match flag {
            SFlag::N => self.negative = v,
            SFlag::V => self.overflow = v,
            SFlag::R => self.reserved = v,
            SFlag::B => self.breaked = v,
            SFlag::D => self.decimal = v,
            SFlag::I => self.interrupt = v,
            SFlag::Z => self.zero = v,
            SFlag::C => self.carry = v,
        }
    }
    pub fn on(&mut self, flag: SFlag) {
        self.toggle(flag, true);
    }

    pub fn off(&mut self, flag: SFlag) {
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
