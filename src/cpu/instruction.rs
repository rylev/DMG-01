#[derive(Copy, Clone, Debug, PartialEq)]
pub enum IncDecRegister {
    A, B, C, D, E, H, L,
    BC, DE, HL,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Instruction {
    Inc(IncDecRegister),
    Dec(IncDecRegister),
}
