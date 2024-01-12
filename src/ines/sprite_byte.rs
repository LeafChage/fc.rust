#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct SpriteByte {
    b0: u8,
    b1: u8,
    b2: u8,
    b3: u8,
    b4: u8,
    b5: u8,
    b6: u8,
    b7: u8,
}

impl SpriteByte {
    fn new(b0: u8, b1: u8, b2: u8, b3: u8, b4: u8, b5: u8, b6: u8, b7: u8) -> Self {
        Self {
            b0,
            b1,
            b2,
            b3,
            b4,
            b5,
            b6,
            b7,
        }
    }
}

impl std::fmt::Display for SpriteByte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            b0,
            b1,
            b2,
            b3,
            b4,
            b5,
            b6,
            b7,
        } = self;
        write!(f, "{}{}{}{}{}{}{}{}", b0, b1, b2, b3, b4, b5, b6, b7,)
    }
}

impl std::ops::Add for SpriteByte {
    type Output = SpriteByte;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            self.b0 + rhs.b0,
            self.b1 + rhs.b1,
            self.b2 + rhs.b2,
            self.b3 + rhs.b3,
            self.b4 + rhs.b4,
            self.b5 + rhs.b5,
            self.b6 + rhs.b6,
            self.b7 + rhs.b7,
        )
    }
}

#[test]
fn it_byte_add() {
    assert_eq!(
        SpriteByte::from(0b1000_1000) + SpriteByte::from(0b1000_1000),
        SpriteByte::new(2, 0, 0, 0, 2, 0, 0, 0)
    )
}

impl From<u8> for SpriteByte {
    fn from(value: u8) -> Self {
        SpriteByte {
            b0: (0b1000_0000 & value) >> 7,
            b1: (0b0100_0000 & value) >> 6,
            b2: (0b0010_0000 & value) >> 5,
            b3: (0b0001_0000 & value) >> 4,
            b4: (0b0000_1000 & value) >> 3,
            b5: (0b0000_0100 & value) >> 2,
            b6: (0b0000_0010 & value) >> 1,
            b7: (0b0000_0001 & value) >> 0,
        }
    }
}

#[test]
fn it_byte() {
    assert_eq!(
        SpriteByte::from(0b1000_1000),
        SpriteByte::new(1, 0, 0, 0, 1, 0, 0, 0)
    )
}

impl From<SpriteByte> for [u8; 8] {
    fn from(value: SpriteByte) -> Self {
        [
            value.b0, value.b1, value.b2, value.b3, value.b4, value.b5, value.b6, value.b7,
        ]
    }
}

#[test]
fn it_bytes_from_sprite_byte() {
    assert_eq!(
        <[u8; 8]>::from(SpriteByte::new(1, 0, 0, 0, 1, 0, 0, 0)),
        [1, 0, 0, 0, 1, 0, 0, 0]
    )
}
