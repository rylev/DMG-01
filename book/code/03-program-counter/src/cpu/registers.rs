use super::flags_register::FlagsRegister;

#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: FlagsRegister,
    pub h: u8,
    pub l: u8
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: FlagsRegister::new(),
            h: 0,
            l: 0
        }
    }

    pub fn get_af(&self) -> u16 {
        (self.a as u16) << 8
        | u8::from(self.f) as u16
    }
    pub fn set_af(&mut self, value: u16) {
        self.a = ((value & 0xFF00) >> 8) as u8;
        self.f = FlagsRegister::from((value & 0xF) as u8);
    }

    pub fn get_bc(&self) -> u16 {
        (self.b as u16) << 8
        | self.c as u16
    }
    pub fn set_bc(&mut self, value: u16) {
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }

    pub fn get_de(&self) -> u16 {
        (self.d as u16) << 8
        | self.e as u16
    }
    pub fn set_de(&mut self, value: u16) {
        self.d = ((value & 0xFF00) >> 8) as u8;
        self.e = (value & 0xFF) as u8;
    }

    pub fn get_hl(&self) -> u16 {
        (self.h as u16) << 8
        | self.l as u16
    }
    pub fn set_hl(&mut self, value: u16) {
        self.h = ((value & 0xFF00) >> 8) as u8;
        self.l = (value & 0xFF) as u8;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_set_bc() {
        let mut registers = Registers::new();
        registers.set_bc(0b1010_1111_1100_1100);
        assert_eq!(registers.b, 0b1010_1111u8);
        assert_eq!(registers.c, 0b1100_1100u8);
    }

    #[test]
    fn can_set_f_as_u8() {
        let mut registers = Registers::new();
        let value = 0b1100_0000;
        registers.f = value.into();
        let result: u8 = registers.f.into();
        assert_eq!(result, value);
    }

    #[test]
    fn can_set_f_as_flags_struct() {
        let mut registers = Registers::new();
        let value: FlagsRegister = 0b1100_0000u8.into();
        registers.f = value;
        assert_eq!(registers.f, value);
    }
}
