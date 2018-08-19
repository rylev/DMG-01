use std;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum IncDecTarget {
    A, B, C, D, E, H, L,
    BC, DE, HL,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ArithmeticTarget {
    A, B, C, D, E, H, L,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ADDHLTarget {
    BC, DE, HL
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PrefixTarget {
    A, B, C, D, E, H, L,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BitPosition {
    B0, B1, B2, B3, B4, B5, B6, B7
}
impl std::convert::From<BitPosition> for u8  {
    fn from(position: BitPosition) -> u8 {
        match position {
            BitPosition::B0 => 0,
            BitPosition::B1 => 1,
            BitPosition::B2 => 2,
            BitPosition::B3 => 3,
            BitPosition::B4 => 4,
            BitPosition::B5 => 5,
            BitPosition::B6 => 6,
            BitPosition::B7 => 7
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Instruction {
    INC(IncDecTarget),
    DEC(IncDecTarget),

    ADD(ArithmeticTarget),
    ADDHL(ADDHLTarget),
    ADC(ArithmeticTarget),
    SUB(ArithmeticTarget),
    SBC(ArithmeticTarget),
    AND(ArithmeticTarget),
    OR(ArithmeticTarget),
    XOR(ArithmeticTarget),
    CP(ArithmeticTarget),

    CCF,
    SCF,

    RRA,
    RLA,
    RRCA,
    RLCA,
    CPL,

    BIT(PrefixTarget, BitPosition),
    RES(PrefixTarget, BitPosition),
    SET(PrefixTarget, BitPosition),
    SRL(PrefixTarget),
    RR(PrefixTarget),
    RL(PrefixTarget),
    RRC(PrefixTarget),
    RLC(PrefixTarget),
    SRA(PrefixTarget),
    SLA(PrefixTarget),
    SWAP(PrefixTarget)
}
