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

impl Instruction {
    pub fn from_byte(byte: u8, prefixed: bool) -> Option<Instruction> {
        if prefixed {
            Instruction::from_byte_prefixed(byte)
        } else {
            Instruction::from_byte_not_prefixed(byte)
        }
    }

    fn from_byte_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            0x00 => Some(Instruction::RLC(PrefixTarget::B)),
            0x01 => Some(Instruction::RLC(PrefixTarget::C)),
            0x02 => Some(Instruction::RLC(PrefixTarget::D)),
            0x03 => Some(Instruction::RLC(PrefixTarget::E)),
            0x04 => Some(Instruction::RLC(PrefixTarget::H)),
            0x05 => Some(Instruction::RLC(PrefixTarget::L)),
            0x07 => Some(Instruction::RLC(PrefixTarget::A)),

            0x08 => Some(Instruction::RRC(PrefixTarget::B)),
            0x09 => Some(Instruction::RRC(PrefixTarget::C)),
            0x0a => Some(Instruction::RRC(PrefixTarget::D)),
            0x0b => Some(Instruction::RRC(PrefixTarget::E)),
            0x0c => Some(Instruction::RRC(PrefixTarget::H)),
            0x0d => Some(Instruction::RRC(PrefixTarget::L)),
            0x0f => Some(Instruction::RRC(PrefixTarget::A)),

            0x10 => Some(Instruction::RL(PrefixTarget::B)),
            0x11 => Some(Instruction::RL(PrefixTarget::C)),
            0x12 => Some(Instruction::RL(PrefixTarget::D)),
            0x13 => Some(Instruction::RL(PrefixTarget::E)),
            0x14 => Some(Instruction::RL(PrefixTarget::H)),
            0x15 => Some(Instruction::RL(PrefixTarget::L)),
            0x17 => Some(Instruction::RL(PrefixTarget::A)),

            0x18 => Some(Instruction::RR(PrefixTarget::B)),
            0x19 => Some(Instruction::RR(PrefixTarget::C)),
            0x1a => Some(Instruction::RR(PrefixTarget::D)),
            0x1b => Some(Instruction::RR(PrefixTarget::E)),
            0x1c => Some(Instruction::RR(PrefixTarget::H)),
            0x1d => Some(Instruction::RR(PrefixTarget::L)),
            0x1f => Some(Instruction::RR(PrefixTarget::A)),

            0x20 => Some(Instruction::SLA(PrefixTarget::B)),
            0x21 => Some(Instruction::SLA(PrefixTarget::C)),
            0x22 => Some(Instruction::SLA(PrefixTarget::D)),
            0x23 => Some(Instruction::SLA(PrefixTarget::E)),
            0x24 => Some(Instruction::SLA(PrefixTarget::H)),
            0x25 => Some(Instruction::SLA(PrefixTarget::L)),
            0x27 => Some(Instruction::SLA(PrefixTarget::A)),

            0x28 => Some(Instruction::SRA(PrefixTarget::B)),
            0x29 => Some(Instruction::SRA(PrefixTarget::C)),
            0x2a => Some(Instruction::SRA(PrefixTarget::D)),
            0x2b => Some(Instruction::SRA(PrefixTarget::E)),
            0x2c => Some(Instruction::SRA(PrefixTarget::H)),
            0x2d => Some(Instruction::SRA(PrefixTarget::L)),
            0x2f => Some(Instruction::SRA(PrefixTarget::A)),

            0x30 => Some(Instruction::SWAP(PrefixTarget::B)),
            0x31 => Some(Instruction::SWAP(PrefixTarget::C)),
            0x32 => Some(Instruction::SWAP(PrefixTarget::D)),
            0x33 => Some(Instruction::SWAP(PrefixTarget::E)),
            0x34 => Some(Instruction::SWAP(PrefixTarget::H)),
            0x35 => Some(Instruction::SWAP(PrefixTarget::L)),
            0x37 => Some(Instruction::SWAP(PrefixTarget::A)),

            0x38 => Some(Instruction::SRL(PrefixTarget::B)),
            0x39 => Some(Instruction::SRL(PrefixTarget::C)),
            0x3a => Some(Instruction::SRL(PrefixTarget::D)),
            0x3b => Some(Instruction::SRL(PrefixTarget::E)),
            0x3c => Some(Instruction::SRL(PrefixTarget::H)),
            0x3d => Some(Instruction::SRL(PrefixTarget::L)),
            0x3f => Some(Instruction::SRL(PrefixTarget::A)),

            0x40 => Some(Instruction::BIT(PrefixTarget::B, BitPosition::B0)),
            0x41 => Some(Instruction::BIT(PrefixTarget::C, BitPosition::B0)),
            0x42 => Some(Instruction::BIT(PrefixTarget::D, BitPosition::B0)),
            0x43 => Some(Instruction::BIT(PrefixTarget::E, BitPosition::B0)),
            0x44 => Some(Instruction::BIT(PrefixTarget::H, BitPosition::B0)),
            0x45 => Some(Instruction::BIT(PrefixTarget::L, BitPosition::B0)),
            0x47 => Some(Instruction::BIT(PrefixTarget::A, BitPosition::B0)),
            0x48 => Some(Instruction::BIT(PrefixTarget::B, BitPosition::B1)),
            0x49 => Some(Instruction::BIT(PrefixTarget::C, BitPosition::B1)),
            0x4a => Some(Instruction::BIT(PrefixTarget::D, BitPosition::B1)),
            0x4b => Some(Instruction::BIT(PrefixTarget::E, BitPosition::B1)),
            0x4c => Some(Instruction::BIT(PrefixTarget::H, BitPosition::B1)),
            0x4d => Some(Instruction::BIT(PrefixTarget::L, BitPosition::B1)),
            0x4f => Some(Instruction::BIT(PrefixTarget::A, BitPosition::B1)),

            0x50 => Some(Instruction::BIT(PrefixTarget::B, BitPosition::B2)),
            0x51 => Some(Instruction::BIT(PrefixTarget::C, BitPosition::B2)),
            0x52 => Some(Instruction::BIT(PrefixTarget::D, BitPosition::B2)),
            0x53 => Some(Instruction::BIT(PrefixTarget::E, BitPosition::B2)),
            0x54 => Some(Instruction::BIT(PrefixTarget::H, BitPosition::B2)),
            0x55 => Some(Instruction::BIT(PrefixTarget::L, BitPosition::B2)),
            0x57 => Some(Instruction::BIT(PrefixTarget::A, BitPosition::B2)),
            0x58 => Some(Instruction::BIT(PrefixTarget::B, BitPosition::B3)),
            0x59 => Some(Instruction::BIT(PrefixTarget::C, BitPosition::B3)),
            0x5a => Some(Instruction::BIT(PrefixTarget::D, BitPosition::B3)),
            0x5b => Some(Instruction::BIT(PrefixTarget::E, BitPosition::B3)),
            0x5c => Some(Instruction::BIT(PrefixTarget::H, BitPosition::B3)),
            0x5d => Some(Instruction::BIT(PrefixTarget::L, BitPosition::B3)),
            0x5f => Some(Instruction::BIT(PrefixTarget::A, BitPosition::B3)),


            0x60 => Some(Instruction::BIT(PrefixTarget::B, BitPosition::B4)),
            0x61 => Some(Instruction::BIT(PrefixTarget::C, BitPosition::B4)),
            0x62 => Some(Instruction::BIT(PrefixTarget::D, BitPosition::B4)),
            0x63 => Some(Instruction::BIT(PrefixTarget::E, BitPosition::B4)),
            0x64 => Some(Instruction::BIT(PrefixTarget::H, BitPosition::B4)),
            0x65 => Some(Instruction::BIT(PrefixTarget::L, BitPosition::B4)),
            0x67 => Some(Instruction::BIT(PrefixTarget::A, BitPosition::B4)),
            0x68 => Some(Instruction::BIT(PrefixTarget::B, BitPosition::B5)),
            0x69 => Some(Instruction::BIT(PrefixTarget::C, BitPosition::B5)),
            0x6a => Some(Instruction::BIT(PrefixTarget::D, BitPosition::B5)),
            0x6b => Some(Instruction::BIT(PrefixTarget::E, BitPosition::B5)),
            0x6c => Some(Instruction::BIT(PrefixTarget::H, BitPosition::B5)),
            0x6d => Some(Instruction::BIT(PrefixTarget::L, BitPosition::B5)),
            0x6f => Some(Instruction::BIT(PrefixTarget::A, BitPosition::B5)),


            0x70 => Some(Instruction::BIT(PrefixTarget::B, BitPosition::B6)),
            0x71 => Some(Instruction::BIT(PrefixTarget::C, BitPosition::B6)),
            0x72 => Some(Instruction::BIT(PrefixTarget::D, BitPosition::B6)),
            0x73 => Some(Instruction::BIT(PrefixTarget::E, BitPosition::B6)),
            0x74 => Some(Instruction::BIT(PrefixTarget::H, BitPosition::B6)),
            0x75 => Some(Instruction::BIT(PrefixTarget::L, BitPosition::B6)),
            0x77 => Some(Instruction::BIT(PrefixTarget::A, BitPosition::B6)),
            0x78 => Some(Instruction::BIT(PrefixTarget::B, BitPosition::B7)),
            0x79 => Some(Instruction::BIT(PrefixTarget::C, BitPosition::B7)),
            0x7a => Some(Instruction::BIT(PrefixTarget::D, BitPosition::B7)),
            0x7b => Some(Instruction::BIT(PrefixTarget::E, BitPosition::B7)),
            0x7c => Some(Instruction::BIT(PrefixTarget::H, BitPosition::B7)),
            0x7d => Some(Instruction::BIT(PrefixTarget::L, BitPosition::B7)),
            0x7f => Some(Instruction::BIT(PrefixTarget::A, BitPosition::B7)),

            0x80 => Some(Instruction::RES(PrefixTarget::B, BitPosition::B0)),
            0x81 => Some(Instruction::RES(PrefixTarget::C, BitPosition::B0)),
            0x82 => Some(Instruction::RES(PrefixTarget::D, BitPosition::B0)),
            0x83 => Some(Instruction::RES(PrefixTarget::E, BitPosition::B0)),
            0x84 => Some(Instruction::RES(PrefixTarget::H, BitPosition::B0)),
            0x85 => Some(Instruction::RES(PrefixTarget::L, BitPosition::B0)),
            0x87 => Some(Instruction::RES(PrefixTarget::A, BitPosition::B0)),
            0x88 => Some(Instruction::RES(PrefixTarget::B, BitPosition::B1)),
            0x89 => Some(Instruction::RES(PrefixTarget::C, BitPosition::B1)),
            0x8a => Some(Instruction::RES(PrefixTarget::D, BitPosition::B1)),
            0x8b => Some(Instruction::RES(PrefixTarget::E, BitPosition::B1)),
            0x8c => Some(Instruction::RES(PrefixTarget::H, BitPosition::B1)),
            0x8d => Some(Instruction::RES(PrefixTarget::L, BitPosition::B1)),
            0x8f => Some(Instruction::RES(PrefixTarget::A, BitPosition::B1)),

            0x90 => Some(Instruction::RES(PrefixTarget::B, BitPosition::B2)),
            0x91 => Some(Instruction::RES(PrefixTarget::C, BitPosition::B2)),
            0x92 => Some(Instruction::RES(PrefixTarget::D, BitPosition::B2)),
            0x93 => Some(Instruction::RES(PrefixTarget::E, BitPosition::B2)),
            0x94 => Some(Instruction::RES(PrefixTarget::H, BitPosition::B2)),
            0x95 => Some(Instruction::RES(PrefixTarget::L, BitPosition::B2)),
            0x97 => Some(Instruction::RES(PrefixTarget::A, BitPosition::B2)),
            0x98 => Some(Instruction::RES(PrefixTarget::B, BitPosition::B3)),
            0x99 => Some(Instruction::RES(PrefixTarget::C, BitPosition::B3)),
            0x9a => Some(Instruction::RES(PrefixTarget::D, BitPosition::B3)),
            0x9b => Some(Instruction::RES(PrefixTarget::E, BitPosition::B3)),
            0x9c => Some(Instruction::RES(PrefixTarget::H, BitPosition::B3)),
            0x9d => Some(Instruction::RES(PrefixTarget::L, BitPosition::B3)),
            0x9f => Some(Instruction::RES(PrefixTarget::A, BitPosition::B3)),


            0xa0 => Some(Instruction::RES(PrefixTarget::B, BitPosition::B4)),
            0xa1 => Some(Instruction::RES(PrefixTarget::C, BitPosition::B4)),
            0xa2 => Some(Instruction::RES(PrefixTarget::D, BitPosition::B4)),
            0xa3 => Some(Instruction::RES(PrefixTarget::E, BitPosition::B4)),
            0xa4 => Some(Instruction::RES(PrefixTarget::H, BitPosition::B4)),
            0xa5 => Some(Instruction::RES(PrefixTarget::L, BitPosition::B4)),
            0xa7 => Some(Instruction::RES(PrefixTarget::A, BitPosition::B4)),
            0xa8 => Some(Instruction::RES(PrefixTarget::B, BitPosition::B5)),
            0xa9 => Some(Instruction::RES(PrefixTarget::C, BitPosition::B5)),
            0xaa => Some(Instruction::RES(PrefixTarget::D, BitPosition::B5)),
            0xab => Some(Instruction::RES(PrefixTarget::E, BitPosition::B5)),
            0xac => Some(Instruction::RES(PrefixTarget::H, BitPosition::B5)),
            0xad => Some(Instruction::RES(PrefixTarget::L, BitPosition::B5)),
            0xaf => Some(Instruction::RES(PrefixTarget::A, BitPosition::B5)),


            0xb0 => Some(Instruction::RES(PrefixTarget::B, BitPosition::B6)),
            0xb1 => Some(Instruction::RES(PrefixTarget::C, BitPosition::B6)),
            0xb2 => Some(Instruction::RES(PrefixTarget::D, BitPosition::B6)),
            0xb3 => Some(Instruction::RES(PrefixTarget::E, BitPosition::B6)),
            0xb4 => Some(Instruction::RES(PrefixTarget::H, BitPosition::B6)),
            0xb5 => Some(Instruction::RES(PrefixTarget::L, BitPosition::B6)),
            0xb7 => Some(Instruction::RES(PrefixTarget::A, BitPosition::B6)),
            0xb8 => Some(Instruction::RES(PrefixTarget::B, BitPosition::B7)),
            0xb9 => Some(Instruction::RES(PrefixTarget::C, BitPosition::B7)),
            0xba => Some(Instruction::RES(PrefixTarget::D, BitPosition::B7)),
            0xbb => Some(Instruction::RES(PrefixTarget::E, BitPosition::B7)),
            0xbc => Some(Instruction::RES(PrefixTarget::H, BitPosition::B7)),
            0xbd => Some(Instruction::RES(PrefixTarget::L, BitPosition::B7)),
            0xbf => Some(Instruction::RES(PrefixTarget::A, BitPosition::B7)),

            0xc0 => Some(Instruction::SET(PrefixTarget::B, BitPosition::B0)),
            0xc1 => Some(Instruction::SET(PrefixTarget::C, BitPosition::B0)),
            0xc2 => Some(Instruction::SET(PrefixTarget::D, BitPosition::B0)),
            0xc3 => Some(Instruction::SET(PrefixTarget::E, BitPosition::B0)),
            0xc4 => Some(Instruction::SET(PrefixTarget::H, BitPosition::B0)),
            0xc5 => Some(Instruction::SET(PrefixTarget::L, BitPosition::B0)),
            0xc7 => Some(Instruction::SET(PrefixTarget::A, BitPosition::B0)),
            0xc8 => Some(Instruction::SET(PrefixTarget::B, BitPosition::B1)),
            0xc9 => Some(Instruction::SET(PrefixTarget::C, BitPosition::B1)),
            0xca => Some(Instruction::SET(PrefixTarget::D, BitPosition::B1)),
            0xcb => Some(Instruction::SET(PrefixTarget::E, BitPosition::B1)),
            0xcc => Some(Instruction::SET(PrefixTarget::H, BitPosition::B1)),
            0xcd => Some(Instruction::SET(PrefixTarget::L, BitPosition::B1)),
            0xcf => Some(Instruction::SET(PrefixTarget::A, BitPosition::B1)),

            0xd0 => Some(Instruction::SET(PrefixTarget::B, BitPosition::B2)),
            0xd1 => Some(Instruction::SET(PrefixTarget::C, BitPosition::B2)),
            0xd2 => Some(Instruction::SET(PrefixTarget::D, BitPosition::B2)),
            0xd3 => Some(Instruction::SET(PrefixTarget::E, BitPosition::B2)),
            0xd4 => Some(Instruction::SET(PrefixTarget::H, BitPosition::B2)),
            0xd5 => Some(Instruction::SET(PrefixTarget::L, BitPosition::B2)),
            0xd7 => Some(Instruction::SET(PrefixTarget::A, BitPosition::B2)),
            0xd8 => Some(Instruction::SET(PrefixTarget::B, BitPosition::B3)),
            0xd9 => Some(Instruction::SET(PrefixTarget::C, BitPosition::B3)),
            0xda => Some(Instruction::SET(PrefixTarget::D, BitPosition::B3)),
            0xdb => Some(Instruction::SET(PrefixTarget::E, BitPosition::B3)),
            0xdc => Some(Instruction::SET(PrefixTarget::H, BitPosition::B3)),
            0xdd => Some(Instruction::SET(PrefixTarget::L, BitPosition::B3)),
            0xdf => Some(Instruction::SET(PrefixTarget::A, BitPosition::B3)),

            0xe0 => Some(Instruction::SET(PrefixTarget::B, BitPosition::B4)),
            0xe1 => Some(Instruction::SET(PrefixTarget::C, BitPosition::B4)),
            0xe2 => Some(Instruction::SET(PrefixTarget::D, BitPosition::B4)),
            0xe3 => Some(Instruction::SET(PrefixTarget::E, BitPosition::B4)),
            0xe4 => Some(Instruction::SET(PrefixTarget::H, BitPosition::B4)),
            0xe5 => Some(Instruction::SET(PrefixTarget::L, BitPosition::B4)),
            0xe7 => Some(Instruction::SET(PrefixTarget::A, BitPosition::B4)),
            0xe8 => Some(Instruction::SET(PrefixTarget::B, BitPosition::B5)),
            0xe9 => Some(Instruction::SET(PrefixTarget::C, BitPosition::B5)),
            0xea => Some(Instruction::SET(PrefixTarget::D, BitPosition::B5)),
            0xeb => Some(Instruction::SET(PrefixTarget::E, BitPosition::B5)),
            0xec => Some(Instruction::SET(PrefixTarget::H, BitPosition::B5)),
            0xed => Some(Instruction::SET(PrefixTarget::L, BitPosition::B5)),
            0xef => Some(Instruction::SET(PrefixTarget::A, BitPosition::B5)),

            0xf0 => Some(Instruction::SET(PrefixTarget::B, BitPosition::B6)),
            0xf1 => Some(Instruction::SET(PrefixTarget::C, BitPosition::B6)),
            0xf2 => Some(Instruction::SET(PrefixTarget::D, BitPosition::B6)),
            0xf3 => Some(Instruction::SET(PrefixTarget::E, BitPosition::B6)),
            0xf4 => Some(Instruction::SET(PrefixTarget::H, BitPosition::B6)),
            0xf5 => Some(Instruction::SET(PrefixTarget::L, BitPosition::B6)),
            0xf7 => Some(Instruction::SET(PrefixTarget::A, BitPosition::B6)),
            0xf8 => Some(Instruction::SET(PrefixTarget::B, BitPosition::B7)),
            0xf9 => Some(Instruction::SET(PrefixTarget::C, BitPosition::B7)),
            0xfa => Some(Instruction::SET(PrefixTarget::D, BitPosition::B7)),
            0xfb => Some(Instruction::SET(PrefixTarget::E, BitPosition::B7)),
            0xfc => Some(Instruction::SET(PrefixTarget::H, BitPosition::B7)),
            0xfd => Some(Instruction::SET(PrefixTarget::L, BitPosition::B7)),
            0xff => Some(Instruction::SET(PrefixTarget::A, BitPosition::B7)),
            _ => None
        }
    }

    fn from_byte_not_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            0x02 => Some(Instruction::INC(IncDecTarget::BC)),
            0x13 => Some(Instruction::INC(IncDecTarget::DE)),
            0x23 => Some(Instruction::INC(IncDecTarget::HL)),
            0x04 => Some(Instruction::INC(IncDecTarget::B)),
            0x14 => Some(Instruction::INC(IncDecTarget::D)),
            0x24 => Some(Instruction::INC(IncDecTarget::H)),
            0x0c => Some(Instruction::INC(IncDecTarget::C)),
            0x1c => Some(Instruction::INC(IncDecTarget::E)),
            0x2c => Some(Instruction::INC(IncDecTarget::L)),
            0x3c => Some(Instruction::INC(IncDecTarget::A)),

            0x3d => Some(Instruction::DEC(IncDecTarget::A)),
            0x05 => Some(Instruction::DEC(IncDecTarget::B)),
            0x0d => Some(Instruction::DEC(IncDecTarget::C)),
            0x15 => Some(Instruction::DEC(IncDecTarget::D)),
            0x1d => Some(Instruction::DEC(IncDecTarget::E)),
            0x25 => Some(Instruction::DEC(IncDecTarget::H)),
            0x2d => Some(Instruction::DEC(IncDecTarget::L)),
            0x0b => Some(Instruction::DEC(IncDecTarget::BC)),
            0x1b => Some(Instruction::DEC(IncDecTarget::DE)),
            0x2b => Some(Instruction::DEC(IncDecTarget::HL)),

            0x87 => Some(Instruction::ADD(ArithmeticTarget::A)),
            0x80 => Some(Instruction::ADD(ArithmeticTarget::B)),
            0x81 => Some(Instruction::ADD(ArithmeticTarget::C)),
            0x82 => Some(Instruction::ADD(ArithmeticTarget::D)),
            0x83 => Some(Instruction::ADD(ArithmeticTarget::E)),
            0x84 => Some(Instruction::ADD(ArithmeticTarget::H)),
            0x85 => Some(Instruction::ADD(ArithmeticTarget::L)),

            0x8f => Some(Instruction::ADC(ArithmeticTarget::A)),
            0x88 => Some(Instruction::ADC(ArithmeticTarget::B)),
            0x89 => Some(Instruction::ADC(ArithmeticTarget::C)),
            0x8a => Some(Instruction::ADC(ArithmeticTarget::D)),
            0x8b => Some(Instruction::ADC(ArithmeticTarget::E)),
            0x8c => Some(Instruction::ADC(ArithmeticTarget::H)),
            0x8d => Some(Instruction::ADC(ArithmeticTarget::L)),

            0x97 => Some(Instruction::SUB(ArithmeticTarget::A)),
            0x90 => Some(Instruction::SUB(ArithmeticTarget::B)),
            0x91 => Some(Instruction::SUB(ArithmeticTarget::C)),
            0x92 => Some(Instruction::SUB(ArithmeticTarget::D)),
            0x93 => Some(Instruction::SUB(ArithmeticTarget::E)),
            0x94 => Some(Instruction::SUB(ArithmeticTarget::H)),
            0x95 => Some(Instruction::SUB(ArithmeticTarget::L)),

            0x9f => Some(Instruction::SBC(ArithmeticTarget::A)),
            0x98 => Some(Instruction::SBC(ArithmeticTarget::B)),
            0x99 => Some(Instruction::SBC(ArithmeticTarget::C)),
            0x9a => Some(Instruction::SBC(ArithmeticTarget::D)),
            0x9b => Some(Instruction::SBC(ArithmeticTarget::E)),
            0x9c => Some(Instruction::SBC(ArithmeticTarget::H)),
            0x9d => Some(Instruction::SBC(ArithmeticTarget::L)),

            0xa7 => Some(Instruction::AND(ArithmeticTarget::A)),
            0xa0 => Some(Instruction::AND(ArithmeticTarget::B)),
            0xa1 => Some(Instruction::AND(ArithmeticTarget::C)),
            0xa2 => Some(Instruction::AND(ArithmeticTarget::D)),
            0xa3 => Some(Instruction::AND(ArithmeticTarget::E)),
            0xa4 => Some(Instruction::AND(ArithmeticTarget::H)),
            0xa5 => Some(Instruction::AND(ArithmeticTarget::L)),

            0xb7 => Some(Instruction::OR(ArithmeticTarget::A)),
            0xb0 => Some(Instruction::OR(ArithmeticTarget::B)),
            0xb1 => Some(Instruction::OR(ArithmeticTarget::C)),
            0xb2 => Some(Instruction::OR(ArithmeticTarget::D)),
            0xb3 => Some(Instruction::OR(ArithmeticTarget::E)),
            0xb4 => Some(Instruction::OR(ArithmeticTarget::H)),
            0xb5 => Some(Instruction::OR(ArithmeticTarget::L)),

            0xaf => Some(Instruction::XOR(ArithmeticTarget::A)),
            0xa8 => Some(Instruction::XOR(ArithmeticTarget::B)),
            0xa9 => Some(Instruction::XOR(ArithmeticTarget::C)),
            0xaa => Some(Instruction::XOR(ArithmeticTarget::D)),
            0xab => Some(Instruction::XOR(ArithmeticTarget::E)),
            0xac => Some(Instruction::XOR(ArithmeticTarget::H)),
            0xad => Some(Instruction::XOR(ArithmeticTarget::L)),

            0xbf => Some(Instruction::CP(ArithmeticTarget::A)),
            0xb8 => Some(Instruction::CP(ArithmeticTarget::B)),
            0xb9 => Some(Instruction::CP(ArithmeticTarget::C)),
            0xba => Some(Instruction::CP(ArithmeticTarget::D)),
            0xbb => Some(Instruction::CP(ArithmeticTarget::E)),
            0xbc => Some(Instruction::CP(ArithmeticTarget::H)),
            0xbd => Some(Instruction::CP(ArithmeticTarget::L)),

            0x3f => Some(Instruction::CCF),
            0x37 => Some(Instruction::SCF),
            0x1f => Some(Instruction::RRA),
            0x17 => Some(Instruction::RLA),
            0x0f => Some(Instruction::RRCA),
            0x07 => Some(Instruction::RLCA),
            0x2f => Some(Instruction::CPL),

            _ => None
        }
    }
}
