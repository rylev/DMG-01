pub mod flags_register;
pub mod registers;
pub mod instruction;

use self::registers::Registers;
use self::instruction::{Instruction,IncDecTarget,ArithmeticTarget};

pub struct CPU {
    registers: Registers
}

// Macro for changing the value of a 8 bit register through some CPU method
// Arguments:
// * self (a.k.a the CPU)
// * the name of the register to get,
// * a method for changing register's value,
// * the name of the register to set,
//
// The macro gets the value from the register, performs work on that value and then sets the value back in the
// register
macro_rules! change_8bit_register {
    ( $self:ident, $getter:ident => $work:ident => $setter:ident) => {
        {
            let value = $self.registers.$getter;
            let result = $self.$work(value);
            $self.registers.$setter = result;
        }
    };
}

// Macro for changing the value of a 16 bit register through some CPU method
// Arguments:
// * self (a.k.a the CPU)
// * a method for getting a register,
// * a method for changing register's value,
// * a method for setting a register,
//
// The macro gets the value from the register, performs work on that value and then sets the value back in the
// register
macro_rules! change_16bit_register {
    ( $self:ident, $getter:ident => $work:ident => $setter:ident ) => {
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
                    IncDecTarget::A => change_8bit_register!(self, a => inc_8bit => a),
                    IncDecTarget::B => change_8bit_register!(self, b => inc_8bit => b),
                    IncDecTarget::C => change_8bit_register!(self, c => inc_8bit => c),
                    IncDecTarget::D => change_8bit_register!(self, d => inc_8bit => d),
                    IncDecTarget::E => change_8bit_register!(self, e => inc_8bit => e),
                    IncDecTarget::H => change_8bit_register!(self, h => inc_8bit => h),
                    IncDecTarget::L => change_8bit_register!(self, l => inc_8bit => l),
                    // 16 bit target
                    IncDecTarget::BC => change_16bit_register!(self, get_bc => inc_16bit => set_bc),
                    IncDecTarget::DE => change_16bit_register!(self, get_de => inc_16bit => set_de),
                    IncDecTarget::HL => change_16bit_register!(self, get_hl => inc_16bit => set_hl),
                }
            },
            Instruction::Dec(register) => {
                match register {
                    // 8 bit target
                    IncDecTarget::A => change_8bit_register!(self, a => dec_8bit => a),
                    IncDecTarget::B => change_8bit_register!(self, b => dec_8bit => b),
                    IncDecTarget::C => change_8bit_register!(self, c => dec_8bit => c),
                    IncDecTarget::D => change_8bit_register!(self, d => dec_8bit => d),
                    IncDecTarget::E => change_8bit_register!(self, e => dec_8bit => e),
                    IncDecTarget::H => change_8bit_register!(self, h => dec_8bit => h),
                    IncDecTarget::L => change_8bit_register!(self, l => dec_8bit => l),
                    // 16 bit target
                    IncDecTarget::BC => change_16bit_register!(self, get_bc => dec_16bit => set_bc),
                    IncDecTarget::DE => change_16bit_register!(self, get_de => dec_16bit => set_de),
                    IncDecTarget::HL => change_16bit_register!(self, get_hl => dec_16bit => set_hl),
                }
            },
            Instruction::Add(register) => {
                match register {
                    // 8 bit target
                    ArithmeticTarget::A => change_8bit_register!(self, a => add_without_carry => a),
                    ArithmeticTarget::B => change_8bit_register!(self, b => add_without_carry => a),
                    ArithmeticTarget::C => change_8bit_register!(self, c => add_without_carry => a),
                    ArithmeticTarget::D => change_8bit_register!(self, d => add_without_carry => a),
                    ArithmeticTarget::E => change_8bit_register!(self, e => add_without_carry => a),
                    ArithmeticTarget::H => change_8bit_register!(self, h => add_without_carry => a),
                    ArithmeticTarget::L => change_8bit_register!(self, l => add_without_carry => a),
                    // Direct target
                    ArithmeticTarget::D8(amount) => {
                        let result = self.add_without_carry(amount);
                        self.registers.a = result;
                    }
                }
            },
            Instruction::AddC(register) => {
                match register {
                    // 8 bit target
                    ArithmeticTarget::A => change_8bit_register!(self, a => add_with_carry => a),
                    ArithmeticTarget::B => change_8bit_register!(self, b => add_with_carry => a),
                    ArithmeticTarget::C => change_8bit_register!(self, c => add_with_carry => a),
                    ArithmeticTarget::D => change_8bit_register!(self, d => add_with_carry => a),
                    ArithmeticTarget::E => change_8bit_register!(self, e => add_with_carry => a),
                    ArithmeticTarget::H => change_8bit_register!(self, h => add_with_carry => a),
                    ArithmeticTarget::L => change_8bit_register!(self, l => add_with_carry => a),
                    // Direct target
                    ArithmeticTarget::D8(amount) => {
                        let result = self.add_with_carry(amount);
                        self.registers.a = result;
                    }
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
        // Half Carry is set if the lower nibble of the value is equal to 0x0.
        // If the nibble is equal to 0x0 (0b0000) that means decrementing the value
        // by 1 would cause a carry from the upper nibble to the lower nibble.
        self.registers.f.half_carry = value & 0xF == 0x0;
        new_value
    }

    #[inline(always)]
    fn dec_16bit(&mut self, value: u16) -> u16 {
        value.wrapping_sub(1)
    }

    #[inline(always)]
    fn add_without_carry(&mut self, value: u8) -> u8 {
        self.add(value, false)
    }

    #[inline(always)]
    fn add_with_carry(&mut self, value: u8) -> u8 {
        self.add(value, true)
    }

    #[inline(always)]
    fn add(&mut self, value: u8, add_carry: bool) -> u8 {
        let additional_carry = if add_carry && self.registers.f.carry { 1 } else { 0 };
        let (add, carry) = self.registers.a.overflowing_add(value);
        let (add2, carry2) = add.overflowing_add(additional_carry);
        self.registers.f.zero = add2 == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = carry || carry2;
        self.registers.f.half_carry = ((self.registers.a & 0xf) + (value & 0xf) + additional_carry) > 0xf;
        add2
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
        let instruction = Instruction::Inc(IncDecTarget::A);
        let mut cpu = CPU::new();
        cpu.registers.a = 0x7;
        cpu.execute(instruction);

        assert_eq!(cpu.registers.a, 0x8);
        check_flags!(cpu, zero => false, subtract => false, half_carry => false, carry => false);
    }

    #[test]
    fn execute_inc_8bit_half_carry() {
        let instruction = Instruction::Inc(IncDecTarget::A);
        let mut cpu = CPU::new();
        cpu.registers.a = 0xf;
        cpu.execute(instruction);

        assert_eq!(cpu.registers.a, 0x10);
        check_flags!(cpu, zero => false, subtract => false, half_carry => true, carry => false);
    }

    #[test]
    fn execute_inc_8bit_overflow() {
        let instruction = Instruction::Inc(IncDecTarget::A);
        let mut cpu = CPU::new();
        cpu.registers.a = 0xFF;
        cpu.execute(instruction);

        assert_eq!(cpu.registers.a, 0x0);
        check_flags!(cpu, zero => true, subtract => false, half_carry => true, carry => false);
    }

    #[test]
    fn execute_inc_16bit_byte_overflow() {
        let instruction = Instruction::Inc(IncDecTarget::BC);
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
        let instruction = Instruction::Inc(IncDecTarget::BC);
        let mut cpu = CPU::new();
        cpu.registers.set_bc(0xFFFF);
        cpu.execute(instruction);

        assert_eq!(cpu.registers.get_bc(), 0x0);
        assert_eq!(cpu.registers.b, 0x00);
        assert_eq!(cpu.registers.c, 0x00);
        check_flags!(cpu, zero => false, subtract => false, half_carry => false, carry => false);
    }
}
