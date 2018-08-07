#[derive(Copy, Clone, Debug, PartialEq)]
pub enum IncDecTarget {
    A, B, C, D, E, H, L,
    BC, DE, HL,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Instruction {
    Inc(IncDecTarget),
    Dec(IncDecTarget),
}
