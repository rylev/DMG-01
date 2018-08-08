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
pub enum Instruction {
    Inc(IncDecTarget),
    Dec(IncDecTarget),

    Add(ArithmeticTarget),
    AddC(ArithmeticTarget),
    Sub(ArithmeticTarget),
    SubC(ArithmeticTarget),
    And(ArithmeticTarget),
    Or(ArithmeticTarget),
    Xor(ArithmeticTarget),
    Cp(ArithmeticTarget),

    CCF,
    SCF,

    RRA,
    // RLA,
    // RRCA,
    // RLCA,
    // CPL,
}
