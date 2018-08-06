#[derive(Copy, Clone, Debug, PartialEq)]
pub enum IncDecTarget {
    A, B, C, D, E, H, L,
    BC, DE, HL,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ArithmeticTarget {
    A, B, C, D, E, H, L,
    D8(u8)
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Instruction {
    Inc(IncDecTarget),
    Dec(IncDecTarget),

    Add(ArithmeticTarget),
    AddC(ArithmeticTarget),
}
