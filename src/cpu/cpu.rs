use super::memory::MemoryMap;
use super::register::{Flag, Register};
use crate::memory::{RAM, ROM};
use crate::program::{IndexRegister, Opecode, Operand, ORDER_SET};
use crate::result::Result;
use binary;
use binary::Byte;

pub struct CPU<'a> {
    register: Register,
    memory: MemoryMap<'a>,
}

impl<'a> std::fmt::Display for CPU<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.register)?;
        write!(f, "{}", self.memory)
    }
}

enum Value {
    Ref(u16),
    Immediate(u8),
    Null,
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Ref(n) => write!(f, "0x{:02X?}", n),
            Value::Immediate(n) => write!(f, "#0x{:02X?}", n),
            Value::Null => write!(f, "_"),
        }
    }
}

/// for order to register
enum R {
    A,
    X,
    Y,
    S,
    P,
    PC,
}

impl<'a> CPU<'a> {
    pub fn new(register: Register, memory: MemoryMap<'a>) -> Self {
        Self { register, memory }
    }

    fn addr(&self, operand: Operand, pc: usize) -> Result<Value> {
        Ok(match operand {
            Operand::Absolute => Value::Ref(u16::from_le_bytes([
                self.memory.get(pc)?,
                self.memory.get(pc + 1)?,
            ])),
            Operand::AbsoluteIndirect => Value::Ref({
                let addr =
                    u16::from_le_bytes([self.memory.get(pc)?, self.memory.get(pc + 1)?]) as usize;
                u16::from_le_bytes([self.memory.get(addr)?, self.memory.get(addr + 1)?])
            }),
            Operand::AbsoluteIndex(IndexRegister::X) => Value::Ref(
                u16::from_le_bytes([self.memory.get(pc)?, self.memory.get(pc + 1)?])
                    + self.register.x as u16,
            ),
            Operand::AbsoluteIndex(IndexRegister::Y) => Value::Ref(
                u16::from_le_bytes([self.memory.get(pc)?, self.memory.get(pc + 1)?])
                    + self.register.y as u16,
            ),
            Operand::ZeroPage => Value::Ref(u16::from_le_bytes([self.memory.get(pc)?, 0x00])),
            Operand::ZeroPageIndex(IndexRegister::X) => Value::Ref(u16::from_le_bytes([
                self.memory.get(pc)? + self.register.x,
                0x00,
            ])),
            Operand::ZeroPageIndex(IndexRegister::Y) => Value::Ref(u16::from_le_bytes([
                self.memory.get(pc)? + self.register.y,
                0x00,
            ])),
            Operand::Relative => Value::Ref(
                // TODO:
                (self.register.pc as i32 + 1 + i8::from_le_bytes([self.memory.get(pc)?]) as i32)
                    as u16,
            ),
            Operand::IndirectIndex(_) => todo!(),
            Operand::Immediate => Value::Immediate(self.memory.get(pc)?),
            Operand::Accumulator | Operand::Implied | Operand::Nope => Value::Null,
        })
    }

    fn read_program(&self, pc: usize) -> Result<(Opecode, Operand)> {
        let b = self.memory.get(pc)?;
        let (upper, lower) = binary::byte_to_4bit(b);
        Ok(ORDER_SET[upper as usize][lower as usize])
    }

    pub fn exec(&mut self) -> Result<()> {
        let pc = self.register.pc as usize;
        let (opecode, operand) = self.read_program(self.register.pc as usize)?;
        self.register.pc += 1;

        let value = self.addr(operand, self.register.pc as usize)?;
        self.register.pc += operand.length() as u16;

        println!(
            "{:?} {:?} {:?} {:02X?}",
            opecode,
            operand,
            value,
            [
                self.memory.get(pc)?,
                self.memory.get(pc + 1)?,
                self.memory.get(pc + 2)?,
                self.memory.get(pc + 3)?,
            ]
        );
        self.order(opecode, value)?;
        Ok(())
    }

    fn order(&mut self, opecode: Opecode, value: Value) -> Result<()> {
        Ok(match opecode {
            Opecode::ADC => self.adc(value)?,
            Opecode::SBC => self.sbc(value)?,
            Opecode::AND => self.and(value)?,
            Opecode::ORA => self.ora(value)?,
            Opecode::EOR => self.eor(value)?,
            Opecode::ASL => self.asl(value)?,
            Opecode::LSR => self.lsr(value)?,
            Opecode::ROL => self.rol(value)?,
            Opecode::ROR => self.ror(value)?,
            Opecode::BCC => self.bcc(value)?,
            Opecode::BCS => self.bcs(value)?,
            Opecode::BEQ => self.beq(value)?,
            Opecode::BNE => self.bne(value)?,
            Opecode::BVC => self.bvc(value)?,
            Opecode::BVS => self.bvs(value)?,
            Opecode::BPL => self.bpl(value)?,
            Opecode::BMI => self.bmi(value)?,
            Opecode::BIT => self.bit(value)?,
            Opecode::JMP => self.jmp(value)?,
            Opecode::JSR => self.jsr(value)?,
            Opecode::RTS => self.rts(value)?,
            Opecode::BRK => self.brk(value)?,
            Opecode::RTI => self.rti(value)?,
            Opecode::CMP => self.cmp(value)?,
            Opecode::CPX => self.cpx(value)?,
            Opecode::CPY => self.cpy(value)?,
            Opecode::INC => self.inc(value)?,
            Opecode::DEC => self.dec(value)?,
            Opecode::INX => self.inx(value)?,
            Opecode::DEX => self.dex(value)?,
            Opecode::INY => self.iny(value)?,
            Opecode::DEY => self.dey(value)?,
            Opecode::CLC => self.clc(value)?,
            Opecode::SEC => self.sec(value)?,
            Opecode::CLI => self.cli(value)?,
            Opecode::SEI => self.sei(value)?,
            Opecode::CLD => self.cld(value)?,
            Opecode::SED => self.sed(value)?,
            Opecode::CLV => self.clv(value)?,
            Opecode::LDA => self.lda(value)?,
            Opecode::LDX => self.ldx(value)?,
            Opecode::LDY => self.ldy(value)?,
            Opecode::STA => self.sta(value)?,
            Opecode::STX => self.stx(value)?,
            Opecode::STY => self.sty(value)?,
            Opecode::TAX => self.tax(value)?,
            Opecode::TXA => self.txa(value)?,
            Opecode::TAY => self.tay(value)?,
            Opecode::TYA => self.tya(value)?,
            Opecode::TSX => self.tsx(value)?,
            Opecode::TXS => self.txs(value)?,
            Opecode::PHA => self.pha(value)?,
            Opecode::PLA => self.pla(value)?,
            Opecode::PHP => self.php(value)?,
            Opecode::PLP => self.plp(value)?,
            Opecode::NOP => self.nop(value)?,
        })
    }

    fn update_flag(&mut self, flags: Vec<Flag>, result: u8) {
        for flag in flags.iter() {
            match flag {
                Flag::N => self.register.p.update_negative(result),
                Flag::V => self.register.p.update_overflow(result),
                Flag::R => self.register.p.update_reserved(result),
                Flag::B => self.register.p.update_break(result),
                Flag::D => self.register.p.update_decimal(result),
                Flag::I => self.register.p.update_interrupt(result),
                Flag::Z => self.register.p.update_zero(result),
                Flag::C => self.register.p.update_carry(result),
            }
        }
    }

    fn adc(&mut self, value: Value) -> Result<()> {
        todo!();
    }

    fn sbc(&mut self, value: Value) -> Result<()> {
        todo!();
    }

    fn and(&mut self, value: Value) -> Result<()> {
        let value = match value {
            Value::Ref(value) => self.memory.get(value as usize)?,
            Value::Immediate(v) => v,
            _ => unimplemented!(),
        };
        self.register.a = self.register.a & value;
        self.update_flag(vec![Flag::N, Flag::Z], self.register.a);
        Ok(())
    }

    fn ora(&mut self, value: Value) -> Result<()> {
        let value = match value {
            Value::Ref(value) => self.memory.get(value as usize)?,
            Value::Immediate(v) => v,
            _ => unimplemented!(),
        };
        self.register.a = self.register.a | value;
        self.update_flag(vec![Flag::N, Flag::Z], self.register.a);
        Ok(())
    }

    fn eor(&mut self, value: Value) -> Result<()> {
        let value = match value {
            Value::Ref(value) => self.memory.get(value as usize)?,
            Value::Immediate(v) => v,
            _ => unimplemented!(),
        };
        self.register.a = self.register.a ^ value;
        self.update_flag(vec![Flag::N, Flag::Z], self.register.a);
        Ok(())
    }

    fn asl(&mut self, _: Value) -> Result<()> {
        self.register.a = self.register.a << 1;
        self.register.p.toggle(Flag::C, self.register.a.bit(7));
        self.update_flag(vec![Flag::N, Flag::Z], self.register.a);
        Ok(())
    }

    fn lsr(&mut self, _: Value) -> Result<()> {
        self.register.a = self.register.a >> 1;
        self.register.p.toggle(Flag::C, self.register.a.bit(0));
        self.update_flag(vec![Flag::N, Flag::Z], self.register.a);
        Ok(())
    }

    fn rol(&mut self, _: Value) -> Result<()> {
        self.register.a = (self.register.a << 1).set(0, self.register.p.c());
        self.register.p.toggle(Flag::C, self.register.a.bit(7));
        self.update_flag(vec![Flag::N, Flag::Z], self.register.a);
        Ok(())
    }

    fn ror(&mut self, _: Value) -> Result<()> {
        self.register.a = (self.register.a >> 1).set(7, self.register.p.c());
        self.register.p.toggle(Flag::C, self.register.a.bit(0));
        self.update_flag(vec![Flag::N, Flag::Z], self.register.a);
        Ok(())
    }

    ///
    /// breanch
    ///
    fn branch(&mut self, value: Value, flag: bool) -> Result<()> {
        if flag {
            let value = match value {
                Value::Ref(value) => value,
                _ => unreachable!(),
            };
            self.register.pc = value;
        }
        Ok(())
    }

    fn bcc(&mut self, value: Value) -> Result<()> {
        self.branch(value, !self.register.p.c())
    }

    fn bcs(&mut self, value: Value) -> Result<()> {
        self.branch(value, self.register.p.c())
    }

    fn beq(&mut self, value: Value) -> Result<()> {
        self.branch(value, !self.register.p.z())
    }

    fn bne(&mut self, value: Value) -> Result<()> {
        self.branch(value, self.register.p.z())
    }

    fn bvc(&mut self, value: Value) -> Result<()> {
        self.branch(value, !self.register.p.v())
    }

    fn bvs(&mut self, value: Value) -> Result<()> {
        self.branch(value, self.register.p.v())
    }

    fn bpl(&mut self, value: Value) -> Result<()> {
        self.branch(value, !self.register.p.n())
    }

    fn bmi(&mut self, value: Value) -> Result<()> {
        self.branch(value, self.register.p.n())
    }

    fn bit(&mut self, _: Value) -> Result<()> {
        todo!();
    }

    fn jmp(&mut self, value: Value) -> Result<()> {
        let value = match value {
            Value::Ref(value) => value,
            _ => unreachable!(),
        };
        self.register.pc = value;
        Ok(())
    }

    fn jsr(&mut self, value: Value) -> Result<()> {
        todo!();
    }
    fn rts(&mut self, value: Value) -> Result<()> {
        todo!();
    }

    fn brk(&mut self, _: Value) -> Result<()> {
        if self.register.p.i() {
            // ignore
            return Ok(());
        } else {
            todo!();
        }
    }
    fn rti(&mut self, value: Value) -> Result<()> {
        todo!();
    }

    fn cmp(&mut self, value: Value) -> Result<()> {
        todo!();
    }
    fn cpx(&mut self, value: Value) -> Result<()> {
        todo!();
    }
    fn cpy(&mut self, value: Value) -> Result<()> {
        todo!();
    }

    fn inc(&mut self, value: Value) -> Result<()> {
        let value = match value {
            Value::Ref(value) => value,
            _ => unreachable!(),
        } as usize;

        let result = self.memory.get(value)? + 1;
        self.memory.put(value, result)?;
        self.update_flag(vec![Flag::N, Flag::Z], result);
        Ok(())
    }
    fn dec(&mut self, value: Value) -> Result<()> {
        let value = match value {
            Value::Ref(value) => value,
            _ => unreachable!(),
        } as usize;

        let result = self.memory.get(value)? - 1;
        self.memory.put(value, result)?;
        self.update_flag(vec![Flag::N, Flag::Z], result);
        Ok(())
    }
    fn inx(&mut self, _: Value) -> Result<()> {
        self.register.x += 1;
        self.update_flag(vec![Flag::N, Flag::Z], self.register.x);
        Ok(())
    }
    fn dex(&mut self, _: Value) -> Result<()> {
        self.register.x -= 1;
        self.update_flag(vec![Flag::N, Flag::Z], self.register.x);
        Ok(())
    }
    fn iny(&mut self, _: Value) -> Result<()> {
        self.register.y += 1;
        self.update_flag(vec![Flag::N, Flag::Z], self.register.x);
        Ok(())
    }
    fn dey(&mut self, _: Value) -> Result<()> {
        self.register.y -= 1;
        self.update_flag(vec![Flag::N, Flag::Z], self.register.x);
        Ok(())
    }

    fn clc(&mut self, _: Value) -> Result<()> {
        self.register.p.off(Flag::C);
        Ok(())
    }
    fn sec(&mut self, _: Value) -> Result<()> {
        self.register.p.on(Flag::C);
        Ok(())
    }
    fn cli(&mut self, _: Value) -> Result<()> {
        self.register.p.off(Flag::I);
        Ok(())
    }
    fn sei(&mut self, _: Value) -> Result<()> {
        self.register.p.on(Flag::I);
        Ok(())
    }
    fn cld(&mut self, _: Value) -> Result<()> {
        self.register.p.off(Flag::D);
        Ok(())
    }
    fn sed(&mut self, _: Value) -> Result<()> {
        self.register.p.on(Flag::D);
        Ok(())
    }
    fn clv(&mut self, _: Value) -> Result<()> {
        self.register.p.off(Flag::V);
        Ok(())
    }

    ///
    /// load to register
    ///
    fn load_to_register(&mut self, value: Value, r: R) -> Result<()> {
        let v = match value {
            Value::Ref(value) => self.memory.get(value as usize)?,
            Value::Immediate(v) => v,
            _ => unreachable!(),
        };
        match r {
            R::A => {
                self.register.a = v;
            }
            R::X => {
                self.register.x = v;
            }
            R::Y => {
                self.register.y = v;
            }
            _ => unreachable!(),
        };
        self.update_flag(vec![Flag::N, Flag::Z], v);
        Ok(())
    }

    fn lda(&mut self, value: Value) -> Result<()> {
        self.load_to_register(value, R::A)
    }
    fn ldx(&mut self, value: Value) -> Result<()> {
        self.load_to_register(value, R::X)
    }
    fn ldy(&mut self, value: Value) -> Result<()> {
        self.load_to_register(value, R::Y)
    }

    ///
    /// load to memory
    ///
    fn load_to_memory(&mut self, value: Value, r: R) -> Result<()> {
        let value = match value {
            Value::Ref(value) => value,
            _ => unreachable!(),
        };

        self.memory.put(
            value as usize,
            match r {
                R::A => self.register.a,
                R::X => self.register.x,
                R::Y => self.register.y,
                _ => unreachable!(),
            },
        )
    }
    fn sta(&mut self, value: Value) -> Result<()> {
        self.load_to_memory(value, R::A)
    }
    fn stx(&mut self, value: Value) -> Result<()> {
        self.load_to_memory(value, R::X)
    }
    fn sty(&mut self, value: Value) -> Result<()> {
        self.load_to_memory(value, R::Y)
    }

    ///
    /// transfer from register to register
    ///
    fn tax(&mut self, _: Value) -> Result<()> {
        self.register.x = self.register.a;
        self.update_flag(vec![Flag::N, Flag::Z], self.register.a);
        Ok(())
    }
    fn txa(&mut self, _: Value) -> Result<()> {
        self.register.a = self.register.x;
        self.update_flag(vec![Flag::N, Flag::Z], self.register.a);
        Ok(())
    }
    fn tay(&mut self, _: Value) -> Result<()> {
        self.register.y = self.register.a;
        self.update_flag(vec![Flag::N, Flag::Z], self.register.a);
        Ok(())
    }
    fn tya(&mut self, _: Value) -> Result<()> {
        self.register.a = self.register.y;
        self.update_flag(vec![Flag::N, Flag::Z], self.register.a);
        Ok(())
    }
    fn tsx(&mut self, _: Value) -> Result<()> {
        self.register.x = self.register.s;
        self.update_flag(vec![Flag::N, Flag::Z], self.register.x);
        Ok(())
    }
    fn txs(&mut self, _: Value) -> Result<()> {
        self.register.s = self.register.x;
        Ok(())
    }

    fn pha(&mut self, value: Value) -> Result<()> {
        todo!();
    }
    fn pla(&mut self, value: Value) -> Result<()> {
        todo!();
    }
    fn php(&mut self, value: Value) -> Result<()> {
        todo!();
    }
    fn plp(&mut self, value: Value) -> Result<()> {
        todo!();
    }

    fn nop(&self, _: Value) -> Result<()> {
        Ok(())
    }
}
