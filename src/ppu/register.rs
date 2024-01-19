#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct BufU16 {
    pub upper: u8,
    pub lower: u8,
}

impl Into<u16> for BufU16 {
    fn into(self) -> u16 {
        let BufU16 { lower, upper } = self;
        u16::from_le_bytes([lower, upper])
    }
}

impl BufU16 {
    fn new(upper: u8, lower: u8) -> Self {
        Self { upper, lower }
    }
}

impl From<u16> for BufU16 {
    fn from(value: u16) -> Self {
        let lower = (value & 0x00FF) as u8;
        let upper = (value >> 8) as u8;
        Self { lower, upper }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Register {
    // lower, upper
    ppu_addr: BufU16,
}

impl Register {
    pub fn ppu_addr(&self) -> u16 {
        self.ppu_addr.into()
    }

    pub fn put_addr(&mut self, v: u8) {
        let BufU16 {
            lower,
            upper: _,
        } = self.ppu_addr;
        self.ppu_addr = BufU16::new(lower, v);
    }

    pub fn increment_ppu_addr(&mut self) {
        // TODO change it by flag.
        let v = self.ppu_addr() + 1;
        self.ppu_addr = BufU16::from(v);
    }
}

impl Default for Register {
    fn default() -> Self {
        Self {
            ppu_addr: BufU16::new(0, 0),
        }
    }
}

impl std::fmt::Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PPUADDR: {:02X?}", self.ppu_addr())
    }
}
