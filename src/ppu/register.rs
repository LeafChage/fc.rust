use super::buf_byte::Bufu8;
use binary::Byte;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Register {
    pub control1: u8,
    pub control2: u8,
    pub status: u8,
    pub sprite_addr: u8,
    pub sprite_data: u8,
    /// lower, upper
    ppu_addr: Bufu8,
    pub scroll_offset: Bufu8,
}

impl Register {
    pub fn ppu_addr(&self) -> u16 {
        self.ppu_addr.u16_be_bytes()
    }

    pub fn put_addr(&mut self, v: u8) {
        self.ppu_addr.add(v);
    }

    pub fn increment_ppu_addr(&mut self) {
        // TODO change it by flag.
        let addr = self.ppu_addr() + 1;
        self.ppu_addr = Bufu8::from_16_be_bytes(addr);
    }

    pub fn put_scroll_offset(&mut self, v: u8) {
        self.scroll_offset.add(v);
    }
}

impl Default for Register {
    fn default() -> Self {
        Self {
            control1: 0,
            control2: 0,
            status: 0,
            sprite_addr: 0,
            sprite_data: 9,
            ppu_addr: Bufu8::default(),
            scroll_offset: Bufu8::default(),
        }
    }
}

impl std::fmt::Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PPUADDR: {}\n", self.ppu_addr)?;
        write!(f, "ctr1:\n")?;
        write!(f, "NMI {} /", self.control1.bit(7))?;
        write!(f, "PPUMaster {} /", self.control1.bit(6))?;
        write!(f, "SS {} /", self.control1.bit(5))?;
        write!(f, "BGP {} /", self.control1.bit(4))?;
        write!(f, "SP {} /", self.control1.bit(3))?;
        write!(f, "PPUINC {} ", self.control1.bit(2))
    }
}
