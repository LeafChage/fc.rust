#[rustfmt::skip]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Opecode {
            ADC, SBC,

            AND, ORA, EOR,

            ASL, LSR, ROL, ROR,

            BCC, BCS, BEQ, BNE, BVC, BVS, BPL, BMI,

            BIT,

            JMP, JSR, RTS,

            BRK, RTI,

            CMP, CPX, CPY,

            INC, DEC, INX, DEX, INY, DEY,

            CLC, SEC, CLI, SEI, CLD, SED, CLV,

            LDA, LDX, LDY,

            STA, STX, STY,

            TAX, TXA, TAY, TYA, TSX, TXS, PHA, PLA, PHP, PLP,

            NOP,
}

