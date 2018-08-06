use std;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FlagsRegister {
    // set if the the last operation produced a result of 0
    pub zero: bool,
    // set if the last operation was a subtraction
    pub subtract: bool,
    // set if lower half of the result overflowed
    pub half_carry: bool,
    // set if the result overflowed
    pub carry: bool
}
const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

impl FlagsRegister {
    pub fn new() -> FlagsRegister {
        FlagsRegister {
            zero: false,
            subtract: false,
            half_carry: false,
            carry: false
        }
    }
}

impl std::convert::From<FlagsRegister> for u8  {
    fn from(flag: FlagsRegister) -> u8 {
        (if flag.zero       { 1 } else { 0 }) << ZERO_FLAG_BYTE_POSITION |
        (if flag.subtract   { 1 } else { 0 }) << SUBTRACT_FLAG_BYTE_POSITION |
        (if flag.half_carry { 1 } else { 0 }) << HALF_CARRY_FLAG_BYTE_POSITION |
        (if flag.carry      { 1 } else { 0 }) << CARRY_FLAG_BYTE_POSITION
    }
}

impl std::convert::From<u8> for FlagsRegister {
    fn from(byte: u8) -> Self {
        let zero = ((byte >> ZERO_FLAG_BYTE_POSITION) & 0b1) != 0;
        let subtract = ((byte >> SUBTRACT_FLAG_BYTE_POSITION) & 0b1) != 0;
        let half_carry = ((byte >> HALF_CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;
        let carry = ((byte >> CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;

        FlagsRegister {
            zero,
            subtract,
            half_carry,
            carry
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_be_converted_to_u8() {
        let mut flags = FlagsRegister::new();
        flags.zero = true;
        flags.carry = true;
        let result: u8 = flags.into();
        assert_eq!(result, 0b1001_0000u8);
    }

    #[test]
    fn can_be_converted_from_u8() {
        let result: FlagsRegister = 0b1001_0000.into();
        assert_eq!(result.zero, true);
        assert_eq!(result.carry, true);
        assert_eq!(result.half_carry, false);
        assert_eq!(result.subtract, false);
    }
}
