use std;

use super::flags_register::FlagsRegister;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: FlagsRegister,
    h: u8,
    l: u8
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

    pub fn get_a(&self) -> u8 {
        self.a
    }
    pub fn set_a(&mut self, value: u8) {
        self.a = value;
    }

    pub fn get_b(&self) -> u8 {
        self.b
    }
    pub fn set_b(&mut self, value: u8) {
        self.b = value;
    }

    pub fn get_c(&self) -> u8 {
        self.c
    }
    pub fn set_c(&mut self, value: u8) {
        self.c = value;
    }

    pub fn get_d(&self) -> u8 {
        self.d
    }
    pub fn set_d(&mut self, value: u8) {
        self.d = value;
    }

    pub fn get_e(&self) -> u8 {
        self.e
    }
    pub fn set_e(&mut self, value: u8) {
        self.e = value;
    }

    pub fn get_f(&self) -> FlagsRegister {
        self.f
    }
    pub fn set_f<T: std::convert::Into<FlagsRegister>>(&mut self, value: T) {
        self.f = value.into();
    }

    pub fn get_h(&self) -> u8 {
        self.h
    }
    pub fn set_h(&mut self, value: u8) {
        self.h = value;
    }

    pub fn get_l(&self) -> u8 {
        self.l
    }
    pub fn set_l(&mut self, value: u8) {
        self.l = value;
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
        assert_eq!(registers.get_b(), 0b1010_1111u8);
        assert_eq!(registers.get_c(), 0b1100_1100u8);
    }

    #[test]
    fn can_set_f_as_u8() {
        let mut registers = Registers::new();
        let value = 0b1100_0000;
        registers.set_f(value);
        let result: u8 = registers.get_f().into();
        assert_eq!(result, value);
    }

    #[test]
    fn can_set_f_as_flags_struct() {
        let mut registers = Registers::new();
        let value: FlagsRegister = 0b1100_0000u8.into();
        registers.set_f(value);
        let result = registers.get_f();
        assert_eq!(result, value);
    }
}
