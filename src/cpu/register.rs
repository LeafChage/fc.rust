use super::status_register::StatusRegister;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Register {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub s: u8,
    pub p: StatusRegister,
    pub sp: u16,
    pub pc: u16,
}

impl Default for Register {
    fn default() -> Self {
        Register {
            a: 0,
            x: 0,
            y: 0,
            s: 0,
            sp: 0x01FF,
            p: StatusRegister::default(),
            pc: 0x8000, // TODO
        }
    }
}

impl std::fmt::Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "a: 0x{:02X} / x: 0x{:02X} / y: 0x{:02X}, s: 0x{:02X}, sp: 0x{:04X}, pc: 0x{:04X}",
            self.a, self.x, self.y, self.s, self.sp, self.pc,
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
