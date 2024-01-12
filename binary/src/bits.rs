pub trait Byte {
    fn bit(&self, n: usize) -> bool;
    fn set(&self, n: usize, v: bool) -> Self;
}

impl Byte for u8 {
    fn bit(&self, n: usize) -> bool {
        let n = 1 << n;
        self & n == n
    }

    fn set(&self, n: usize, v: bool) -> Self {
        if v {
            self | 1 << n
        } else {
            self & !(1 << n)
        }
    }
}

#[test]
fn it_get() {
    assert!(0b0000_0001.bit(0));
    assert!(0b0000_0010.bit(1));
    assert!(!0b0000_0001.bit(1));
}

#[test]
fn it_set() {
    assert_eq!(0b0000_1000.set(6, true), 0b0100_1000);
    assert_eq!(0b1111_1111.set(6, false), 0b1011_1111);
}


