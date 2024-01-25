use binary::u16_to_u8;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Bufu8 {
    buf: [u8; 2],
}

impl Copy for Bufu8 {}

impl Default for Bufu8 {
    fn default() -> Self {
        Self { buf: [0; 2] }
    }
}

impl std::fmt::Display for Bufu8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "le:0x{:02X?}({},{})",
            self.u16_le_bytes(),
            self.buf[0],
            self.buf[1]
        )
    }
}

impl Bufu8 {
    pub fn add(&mut self, v: u8) {
        self.buf[0] = self.buf[1];
        self.buf[1] = v;
    }

    pub fn older(&self) -> u8 {
        self.buf[0]
    }

    pub fn later(&self) -> u8 {
        self.buf[1]
    }

    pub fn from_16_le_bytes(v: u16) -> Self {
        let (upper, lower) = u16_to_u8(v);
        let mut buf = Self::default();
        buf.add(lower);
        buf.add(upper);
        buf
    }

    pub fn from_16_be_bytes(v: u16) -> Self {
        let (upper, lower) = u16_to_u8(v);
        let mut buf = Self::default();
        buf.add(upper);
        buf.add(lower);
        buf
    }

    pub fn clear(&mut self) {
        self.buf = [0; 2];
    }

    pub fn u16_le_bytes(&self) -> u16 {
        u16::from_le_bytes([self.buf[0], self.buf[1]])
    }

    pub fn u16_be_bytes(&self) -> u16 {
        u16::from_be_bytes([self.buf[0], self.buf[1]])
    }
}
