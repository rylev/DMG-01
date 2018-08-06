pub mod flags_register;
pub mod registers;
pub mod instruction;

use self::registers::Registers;
use self::instruction::{Instruction,IncDecRegister};

pub struct CPU {
    registers: Registers
}

// Macro for changing the value of a 8 bit register through some CPU method
// Arguments:
// * self (a.k.a the CPU)
// * the name of the register,
// * a method for changing register's value,
//
// The macro gets the value from the register, performs work on that value and then sets the value back in the
// register
macro_rules! change_8bit_register {
    ( $self:ident, $reg:ident, $work:ident) => {
        {
            let amount = $self.registers.$reg;
            let result = $self.$work(amount);
            $self.registers.$reg = result;
        }
    };
}

// Macro for changing the value of a 16 bit register through some CPU method
// Arguments:
// * self (a.k.a the CPU)
// * a method for getting a register,
// * a method for setting a register,
// * a method for changing register's value,
//
// The macro gets the value from the register, performs work on that value and then sets the value back in the
// register
macro_rules! change_16bit_register {
    ( $self:ident, $getter:ident, $setter:ident, $work:ident) => {
        {
            let amount = $self.registers.$getter();
            let result = $self.$work(amount);
            $self.registers.$setter(result);
        }
    };
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            registers: Registers::new()
        }
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Inc(register) => {
                match register {
                    // 8 bit target
                    IncDecRegister::A => change_8bit_register!(self, a, inc_8bit),
                    IncDecRegister::B => change_8bit_register!(self, b, inc_8bit),
                    IncDecRegister::C => change_8bit_register!(self, c, inc_8bit),
                    IncDecRegister::D => change_8bit_register!(self, d, inc_8bit),
                    IncDecRegister::E => change_8bit_register!(self, e, inc_8bit),
                    IncDecRegister::H => change_8bit_register!(self, h, inc_8bit),
                    IncDecRegister::L => change_8bit_register!(self, l, inc_8bit),
                    // 16 bit target
                    IncDecRegister::BC => change_16bit_register!(self, get_bc, set_bc, inc_16bit),
                    IncDecRegister::DE => change_16bit_register!(self, get_de, set_de, inc_16bit),
                    IncDecRegister::HL => change_16bit_register!(self, get_hl, set_hl, inc_16bit),
                }
            },
            Instruction::Dec(register) => {
                match register {
                    // 8 bit target
                    IncDecRegister::A => change_8bit_register!(self, a, dec_8bit),
                    IncDecRegister::B => change_8bit_register!(self, b, dec_8bit),
                    IncDecRegister::C => change_8bit_register!(self, c, dec_8bit),
                    IncDecRegister::D => change_8bit_register!(self, d, dec_8bit),
                    IncDecRegister::E => change_8bit_register!(self, e, dec_8bit),
                    IncDecRegister::H => change_8bit_register!(self, h, dec_8bit),
                    IncDecRegister::L => change_8bit_register!(self, l, dec_8bit),
                    // 16 bit target
                    IncDecRegister::BC => change_16bit_register!(self, get_bc, set_bc, dec_16bit),
                    IncDecRegister::DE => change_16bit_register!(self, get_de, set_de, dec_16bit),
                    IncDecRegister::HL => change_16bit_register!(self, get_hl, set_hl, dec_16bit),
                }
            },
        }
    }

    #[inline(always)]
    fn inc_8bit(&mut self, value: u8) -> u8 {
        let new_value = value.wrapping_add(1);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        // Half Carry is set if the lower nibble of the value is equal to 0xF.
        // If the nibble is equal to 0xF (0b1111) that means incrementing the value
        // by 1 would cause a carry from the lower nibble to the upper nibble.
        self.registers.f.half_carry = value & 0xF == 0xF;
        new_value
    }

    #[inline(always)]
    fn inc_16bit(&mut self, value: u16) -> u16 {
        value.wrapping_add(1)
    }

    #[inline(always)]
    fn dec_8bit(&mut self, value: u8) -> u8 {
        let new_value = value.wrapping_sub(1);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = true;
        self.registers.f.half_carry = value & 0xF == 0x0;
        new_value
    }

    #[inline(always)]
    fn dec_16bit(&mut self, value: u16) -> u16 {
        value.wrapping_sub(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! check_flags {
        ( $cpu:ident,  zero => $zero:ident, subtract => $subtract:ident, half_carry => $half_carry:ident, carry => $carry:ident ) => {
            {
                let flags = $cpu.registers.f;
                println!("Flags: {:?}", flags);
                assert_eq!(flags.zero, $zero);
                assert_eq!(flags.subtract, $subtract);
                assert_eq!(flags.half_carry, $half_carry);
                assert_eq!(flags.carry, $carry);
            }
        };
    }

    // Inc
    #[test]
    fn execute_inc_8bit_non_overflow() {
        let instruction = Instruction::Inc(IncDecRegister::A);
        let mut cpu = CPU::new();
        cpu.registers.a = 0x7;
        cpu.execute(instruction);

        assert_eq!(cpu.registers.a, 0x8);
        check_flags!(cpu, zero => false, subtract => false, half_carry => false, carry => false);
    }

    #[test]
    fn execute_inc_8bit_half_carry() {
        let instruction = Instruction::Inc(IncDecRegister::A);
        let mut cpu = CPU::new();
        cpu.registers.a = 0xf;
        cpu.execute(instruction);

        assert_eq!(cpu.registers.a, 0x10);
        check_flags!(cpu, zero => false, subtract => false, half_carry => true, carry => false);
    }

    #[test]
    fn execute_inc_8bit_overflow() {
        let instruction = Instruction::Inc(IncDecRegister::A);
        let mut cpu = CPU::new();
        cpu.registers.a = 0xFF;
        cpu.execute(instruction);

        assert_eq!(cpu.registers.a, 0x0);
        check_flags!(cpu, zero => true, subtract => false, half_carry => true, carry => false);
    }

    #[test]
    fn execute_inc_16bit_byte_overflow() {
        let instruction = Instruction::Inc(IncDecRegister::BC);
        let mut cpu = CPU::new();
        cpu.registers.set_bc(0xFF);
        cpu.execute(instruction);

        assert_eq!(cpu.registers.get_bc(), 0x0100);
        assert_eq!(cpu.registers.b, 0x01);
        assert_eq!(cpu.registers.c, 0x00);
        check_flags!(cpu, zero => false, subtract => false, half_carry => false, carry => false);
    }

    #[test]
    fn execute_inc_16bit_overflow() {
        let instruction = Instruction::Inc(IncDecRegister::BC);
        let mut cpu = CPU::new();
        cpu.registers.set_bc(0xFFFF);
        cpu.execute(instruction);

        assert_eq!(cpu.registers.get_bc(), 0x0);
        assert_eq!(cpu.registers.b, 0x00);
        assert_eq!(cpu.registers.c, 0x00);
        check_flags!(cpu, zero => false, subtract => false, half_carry => false, carry => false);
    }
}
