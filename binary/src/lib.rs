mod bits;
mod showable;

pub use bits::Byte;
pub use showable::{show_binary, show_binary_with, showable, showable_with};

pub fn byte_to_4bit(v: u8) -> (u8, u8) {
    (v >> 4, 0x0f & v)
}

#[test]
fn it_byte_to_4bit() {
    assert_eq!(byte_to_4bit(0xf2u8), (0xf, 0x2));
    assert_eq!(byte_to_4bit(0x4Fu8), (0x4, 0xf));
}

#[test]
fn it_byte_to_4bit() {
    assert_eq!(byte_to_4bit(0xf2u8), (0xf, 0x2));
    assert_eq!(byte_to_4bit(0x4Fu8), (0x4, 0xf));
}

pub fn u16_to_u8(v: u16) -> (u8, u8) {
    ((v >> 8) as u8, (0x00FF & v) as u8)
}


