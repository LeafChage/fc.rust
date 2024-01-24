#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum IndexRegister {
    X,
    Y,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Operand {
    Accumulator,
    Immediate,
    Absolute,
    AbsoluteIndirect,
    AbsoluteIndex(IndexRegister),
    ZeroPage,
    ZeroPageIndex(IndexRegister),
    Implied,
    Relative,
    IndirectIndex(IndexRegister),
    Nope,
}

impl Operand {
    pub fn length(&self) -> usize {
        match self {
            Operand::Immediate => 1,
            Operand::Absolute => 2,
            Operand::AbsoluteIndirect => 2,
            Operand::AbsoluteIndex(_) => 2,
            Operand::ZeroPage => 1,
            Operand::ZeroPageIndex(_) => 1,
            Operand::Relative => 1,
            Operand::IndirectIndex(_) => 1,
            Operand::Accumulator => 0,
            Operand::Implied => 0,
            Operand::Nope => 0,
        }
    }
}
