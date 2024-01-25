mod opecode;
mod operand;
mod order;

pub use opecode::Opecode;
pub use operand::{IndexRegister, Operand};
pub use order::{CYCLES, ORDER_SET};

pub fn debug_program(rom: Vec<u8>) {
    let mut i = 0;
    while i < rom.len() {
        let a = rom[i];
        let (upper, lower) = binary::byte_to_4bit(a);
        let (opecode, operand) = ORDER_SET[upper as usize][lower as usize];
        let cycle = CYCLES[upper as usize][lower as usize];
        println!(
            "{:04X?}: [{:?},{:?}], ({}) {:02X?}",
            i,
            opecode,
            operand,
            cycle,
            &rom[i..=(i+operand.length())],
        );
        i += 1 + operand.length();
    }
}
