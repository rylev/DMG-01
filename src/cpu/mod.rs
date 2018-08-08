pub mod flags_register;
pub mod registers;
pub mod instruction;

use self::registers::Registers;
use self::instruction::{Instruction,IncDecTarget,ArithmeticTarget};

pub struct CPU {
    registers: Registers
}

// Macro for reading the value of a 8 bit register through some CPU method
// Arguments:
// * self (a.k.a the CPU)
// * the name of the register to get,
// * a method for changing register's value,
//
// The macro gets the value from the register, and performs work on that value
macro_rules! read_8bit_register {
    ( $self:ident, $getter:ident => $work:ident) => {
        {
            let value = $self.registers.$getter;
            $self.$work(value)
        }
    };
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
            let result = read_8bit_register!($self, $getter => $work);
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
                }
            },
            Instruction::Sub(register) => {
                match register {
                    // 8 bit target
                    ArithmeticTarget::A => change_8bit_register!(self, a => sub_without_carry => a),
                    ArithmeticTarget::B => change_8bit_register!(self, b => sub_without_carry => a),
                    ArithmeticTarget::C => change_8bit_register!(self, c => sub_without_carry => a),
                    ArithmeticTarget::D => change_8bit_register!(self, d => sub_without_carry => a),
                    ArithmeticTarget::E => change_8bit_register!(self, e => sub_without_carry => a),
                    ArithmeticTarget::H => change_8bit_register!(self, h => sub_without_carry => a),
                    ArithmeticTarget::L => change_8bit_register!(self, l => sub_without_carry => a),
                }
            },
            Instruction::SubC(register) => {
                match register {
                    // 8 bit target
                    ArithmeticTarget::A => change_8bit_register!(self, a => sub_with_carry => a),
                    ArithmeticTarget::B => change_8bit_register!(self, b => sub_with_carry => a),
                    ArithmeticTarget::C => change_8bit_register!(self, c => sub_with_carry => a),
                    ArithmeticTarget::D => change_8bit_register!(self, d => sub_with_carry => a),
                    ArithmeticTarget::E => change_8bit_register!(self, e => sub_with_carry => a),
                    ArithmeticTarget::H => change_8bit_register!(self, h => sub_with_carry => a),
                    ArithmeticTarget::L => change_8bit_register!(self, l => sub_with_carry => a),
                }
            },
            Instruction::And(register) => {
                match register {
                    // 8 bit target
                    ArithmeticTarget::A => change_8bit_register!(self, a => and => a),
                    ArithmeticTarget::B => change_8bit_register!(self, b => and => a),
                    ArithmeticTarget::C => change_8bit_register!(self, c => and => a),
                    ArithmeticTarget::D => change_8bit_register!(self, d => and => a),
                    ArithmeticTarget::E => change_8bit_register!(self, e => and => a),
                    ArithmeticTarget::H => change_8bit_register!(self, h => and => a),
                    ArithmeticTarget::L => change_8bit_register!(self, l => and => a),
                }
            },
            Instruction::Or(register) => {
                match register {
                    // 8 bit target
                    ArithmeticTarget::A => change_8bit_register!(self, a => or => a),
                    ArithmeticTarget::B => change_8bit_register!(self, b => or => a),
                    ArithmeticTarget::C => change_8bit_register!(self, c => or => a),
                    ArithmeticTarget::D => change_8bit_register!(self, d => or => a),
                    ArithmeticTarget::E => change_8bit_register!(self, e => or => a),
                    ArithmeticTarget::H => change_8bit_register!(self, h => or => a),
                    ArithmeticTarget::L => change_8bit_register!(self, l => or => a),
                }
            },
            Instruction::Xor(register) => {
                match register {
                    // 8 bit target
                    ArithmeticTarget::A => change_8bit_register!(self, a => xor => a),
                    ArithmeticTarget::B => change_8bit_register!(self, b => xor => a),
                    ArithmeticTarget::C => change_8bit_register!(self, c => xor => a),
                    ArithmeticTarget::D => change_8bit_register!(self, d => xor => a),
                    ArithmeticTarget::E => change_8bit_register!(self, e => xor => a),
                    ArithmeticTarget::H => change_8bit_register!(self, h => xor => a),
                    ArithmeticTarget::L => change_8bit_register!(self, l => xor => a),
                }
            },
            Instruction::Cp(register) => {
                match register {
                    // 8 bit target
                    ArithmeticTarget::A => read_8bit_register!(self, a => cp),
                    ArithmeticTarget::B => read_8bit_register!(self, b => cp),
                    ArithmeticTarget::C => read_8bit_register!(self, c => cp),
                    ArithmeticTarget::D => read_8bit_register!(self, d => cp),
                    ArithmeticTarget::E => read_8bit_register!(self, e => cp),
                    ArithmeticTarget::H => read_8bit_register!(self, h => cp),
                    ArithmeticTarget::L => read_8bit_register!(self, l => cp),
                }
            },
            Instruction::CCF => {
                self.registers.f.carry = !self.registers.f.carry;
                self.registers.f.half_carry = false;
                self.registers.f.subtract = false;
            }
            Instruction::SCF => {
                self.registers.f.carry = true;
                self.registers.f.half_carry = false;
                self.registers.f.subtract = false;
            }
            Instruction::RRA => {
                change_8bit_register!(self, a => rr_retain_zero => a);
            }
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
        // Half Carry is set if adding the lower nibbles of the value and register a
        // together (plus the optional carry bit) result in a value bigger the 0xF.
        // If the result is larger than 0xF than the addition caused a carry from
        // the lower nibble to the upper nibble.
        self.registers.f.half_carry = ((self.registers.a & 0xF) + (value & 0xF) + additional_carry) > 0xF;
        add2
    }

    #[inline(always)]
    fn sub_without_carry(&mut self, value: u8) -> u8 {
        self.sub(value, false)
    }

    #[inline(always)]
    fn sub_with_carry(&mut self, value: u8) -> u8 {
        self.sub(value, true)
    }

    #[inline(always)]
    fn sub(&mut self, value: u8, sub_carry: bool) -> u8 {
        let additional_carry = if sub_carry && self.registers.f.carry { 1 } else { 0 };
        let (sub, carry) = self.registers.a.overflowing_sub(value);
        let (sub2, carry2) = sub.overflowing_sub(additional_carry);
        self.registers.f.zero = sub2 == 0;
        self.registers.f.subtract = true;
        self.registers.f.carry = carry || carry2;
        // Half Carry is set if subtracting the lower nibbles of the value (and the
        // optional carry bit) with register a will result in a value lower than 0x0.
        // To avoid underflowing in this test, we can check if the lower nibble of a
        // is less than the lower nibble of the value (with the additional carry)
        self.registers.f.half_carry = (self.registers.a & 0xF) < (value & 0xF) + additional_carry;
        sub2
    }

    #[inline(always)]
    fn and(&mut self, value: u8) -> u8 {
        let new_value = self.registers.a & value;
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = true;
        self.registers.f.carry = false;
        new_value
    }

    #[inline(always)]
    fn or(&mut self, value: u8) -> u8 {
        let new_value = self.registers.a | value;
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = false;
        new_value
    }

    #[inline(always)]
    fn xor(&mut self, value: u8) -> u8 {
        let new_value = self.registers.a ^ value;
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = false;
        new_value
    }

    #[inline(always)]
    fn cp(&mut self, value: u8) {
        self.registers.f.zero = self.registers.a == value;
        self.registers.f.subtract = true;
        // Half Carry is set if subtracting the lower nibbles of the value with register
        // a will result in a value lower than 0x0.  To avoid underflowing in this test,
        // we can check if the lower nibble of a is less than the lower nibble of the value
        self.registers.f.half_carry = (self.registers.a & 0xF) < (value & 0xF);
        self.registers.f.carry = self.registers.a < value;
    }

    #[inline(always)]
    fn rr_retain_zero(&mut self, value: u8) -> u8 {
        self.rr(value, false)
    }

    #[inline(always)]
    fn rr(&mut self, value: u8, set_zero: bool) -> u8 {
        let carry_bit = value & 0b1;
        let new_value = (if self.registers.f.carry { 1 } else { 0 }) << 7 | (value >> 1);
        self.registers.f.zero = set_zero && new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = carry_bit == 1;
        new_value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_instruction {
        ( $instruction:expr, $( $($register:ident).* => $value:expr ),* ) => {
            {
                let mut cpu = CPU::new();
                $(
                    cpu.registers$(.$register)* = $value;
                 )*
                cpu.execute($instruction);
                cpu
            }
        };
    }
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
        let cpu = test_instruction!(Instruction::Inc(IncDecTarget::A), a => 0x7);

        assert_eq!(cpu.registers.a, 0x8);
        check_flags!(cpu, zero => false, subtract => false, half_carry => false, carry => false);
    }

    #[test]
    fn execute_inc_8bit_half_carry() {
        let cpu = test_instruction!(Instruction::Inc(IncDecTarget::A), a => 0xF);

        assert_eq!(cpu.registers.a, 0x10);
        check_flags!(cpu, zero => false, subtract => false, half_carry => true, carry => false);
    }

    #[test]
    fn execute_inc_8bit_overflow() {
        let cpu = test_instruction!(Instruction::Inc(IncDecTarget::A), a => 0xFF);

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

    // Dec
    #[test]
    fn execute_dec_8bit_non_overflow() {
        let cpu = test_instruction!(Instruction::Dec(IncDecTarget::A), a => 0x7);

        assert_eq!(cpu.registers.a, 0x6);
        check_flags!(cpu, zero => false, subtract => true, half_carry => false, carry => false);
    }

    #[test]
    fn execute_dec_8bit_half_carry() {
        let cpu = test_instruction!(Instruction::Dec(IncDecTarget::A), a => 0x80);

        assert_eq!(cpu.registers.a, 0x7f);
        check_flags!(cpu, zero => false, subtract => true, half_carry => true, carry => false);
    }

    #[test]
    fn execute_dec_8bit_underflow() {
        let cpu = test_instruction!(Instruction::Dec(IncDecTarget::A), a => 0x0);

        assert_eq!(cpu.registers.a, 0xFF);
        check_flags!(cpu, zero => false, subtract => true, half_carry => true, carry => false);
    }

    #[test]
    fn execute_dec_16bit_underflow() {
        let instruction = Instruction::Dec(IncDecTarget::BC);
        let mut cpu = CPU::new();
        cpu.registers.set_bc(0x0000);
        cpu.execute(instruction);

        assert_eq!(cpu.registers.get_bc(), 0xFFFF);
        assert_eq!(cpu.registers.b, 0xFF);
        assert_eq!(cpu.registers.c, 0xFF);
        check_flags!(cpu, zero => false, subtract => false, half_carry => false, carry => false);
    }

    // Add
    #[test]
    fn execute_add_8bit_non_overflow_target_a() {
        let cpu = test_instruction!(Instruction::Add(ArithmeticTarget::A), a => 0x7);

        assert_eq!(cpu.registers.a, 0xe);
        check_flags!(cpu, zero => false, subtract => false, half_carry => false, carry => false);
    }

    #[test]
    fn execute_add_8bit_non_overflow_target_c() {
        let cpu = test_instruction!(Instruction::Add(ArithmeticTarget::C), a => 0x7, c => 0x3);

        assert_eq!(cpu.registers.a, 0xa);
        check_flags!(cpu, zero => false, subtract => false, half_carry => false, carry => false);
    }

    #[test]
    fn execute_add_8bit_non_overflow_target_c_with_carry() {
        let cpu = test_instruction!(Instruction::Add(ArithmeticTarget::C), a => 0x7, c => 0x3, f.carry => true);

        assert_eq!(cpu.registers.a, 0xa);
        check_flags!(cpu, zero => false, subtract => false, half_carry => false, carry => false);
    }

    #[test]
    fn execute_add_8bit_carry() {
        let cpu = test_instruction!(Instruction::Add(ArithmeticTarget::B), a => 0xFC, b => 0x9);

        assert_eq!(cpu.registers.a, 0x05);
        check_flags!(cpu, zero => false, subtract => false, half_carry => true, carry => true);
    }

    // Add with carry
    #[test]
    fn execute_addc_8bit_non_overflow_target_a_no_carry() {
        let cpu = test_instruction!(Instruction::Add(ArithmeticTarget::A), a => 0x7);

        assert_eq!(cpu.registers.a, 0xe);
        check_flags!(cpu, zero => false, subtract => false, half_carry => false, carry => false);
    }

    #[test]
    fn execute_addc_8bit_non_overflow_target_a_with_carry() {
        let cpu = test_instruction!(Instruction::AddC(ArithmeticTarget::A), a => 0x7, f.carry => true);

        assert_eq!(cpu.registers.a, 0xf);
        check_flags!(cpu, zero => false, subtract => false, half_carry => false, carry => false);
    }

    #[test]
    fn execute_addc_8bit_non_overflow_target_c_with_carry() {
        let cpu = test_instruction!(Instruction::AddC(ArithmeticTarget::C), a => 0x7, c => 0x3, f.carry => true);

        assert_eq!(cpu.registers.a, 0xb);
        check_flags!(cpu, zero => false, subtract => false, half_carry => false, carry => false);
    }

    #[test]
    fn execute_addc_8bit_carry_with_carry() {
        let cpu = test_instruction!(Instruction::AddC(ArithmeticTarget::B), a => 0xFC, b => 0x3, f.carry => true);

        assert_eq!(cpu.registers.a, 0x00);
        check_flags!(cpu, zero => true, subtract => false, half_carry => true, carry => true);
    }

    // Sub
    #[test]
    fn execute_sub_8bit_non_underflow_target_a() {
        let cpu = test_instruction!(Instruction::Sub(ArithmeticTarget::A), a => 0x7);

        assert_eq!(cpu.registers.a, 0x0);
        check_flags!(cpu, zero => true, subtract => true, half_carry => false, carry => false);
    }

    #[test]
    fn execute_sub_8bit_non_underflow_target_c() {
        let cpu = test_instruction!(Instruction::Sub(ArithmeticTarget::C), a => 0x7, c => 0x3);

        assert_eq!(cpu.registers.a, 0x4);
        check_flags!(cpu, zero => false, subtract => true, half_carry => false, carry => false);
    }

    #[test]
    fn execute_sub_8bit_non_overflow_target_c_with_carry() {
        let cpu = test_instruction!(Instruction::Sub(ArithmeticTarget::C), a => 0x7, c => 0x3, f.carry => true);

        assert_eq!(cpu.registers.a, 0x4);
        check_flags!(cpu, zero => false, subtract => true, half_carry => false, carry => false);
    }

    #[test]
    fn execute_sub_8bit_carry() {
        let cpu = test_instruction!(Instruction::Sub(ArithmeticTarget::B), a => 0x4, b => 0x9);

        assert_eq!(cpu.registers.a, 0xFB);
        check_flags!(cpu, zero => false, subtract => true, half_carry => true, carry => true);
    }

    // Sub with carry
    #[test]
    fn execute_subc_8bit_non_overflow_target_a_no_carry() {
        let cpu = test_instruction!(Instruction::SubC(ArithmeticTarget::A), a => 0x7);

        assert_eq!(cpu.registers.a, 0x0);
        check_flags!(cpu, zero => true, subtract => true, half_carry => false, carry => false);
    }

    #[test]
    fn execute_subc_8bit_non_overflow_target_a_with_carry() {
        let cpu = test_instruction!(Instruction::SubC(ArithmeticTarget::A), a => 0x7, f.carry => true);

        assert_eq!(cpu.registers.a, 0xFF);
        check_flags!(cpu, zero => false, subtract => true, half_carry => true, carry => true);
    }

    #[test]
    fn execute_subc_8bit_non_overflow_target_c_with_carry() {
        let cpu = test_instruction!(Instruction::SubC(ArithmeticTarget::C), a => 0x7, c => 0x3, f.carry => true);

        assert_eq!(cpu.registers.a, 0x3);
        check_flags!(cpu, zero => false, subtract => true, half_carry => false, carry => false);
    }

    // And
    #[test]
    fn execute_and_8bit() {
        let cpu = test_instruction!(Instruction::And(ArithmeticTarget::A), a => 0x7);

        assert_eq!(cpu.registers.a, 0x7);
        check_flags!(cpu, zero => false, subtract => false, half_carry => true, carry => false);
    }

    #[test]
    fn execute_and_8bit_with_zero() {
        let cpu = test_instruction!(Instruction::And(ArithmeticTarget::B), a => 0x8);

        assert_eq!(cpu.registers.a, 0x0);
        check_flags!(cpu, zero => true, subtract => false, half_carry => true, carry => false);
    }

    // Or
    #[test]
    fn execute_or_8bit() {
        let cpu = test_instruction!(Instruction::Or(ArithmeticTarget::A), a => 0x7);

        assert_eq!(cpu.registers.a, 0x7);
        check_flags!(cpu, zero => false, subtract => false, half_carry => false, carry => false);
    }

    #[test]
    fn execute_or_8bit_with_zero() {
        let cpu = test_instruction!(Instruction::Or(ArithmeticTarget::B), a => 0x8);

        assert_eq!(cpu.registers.a, 0x8);
        check_flags!(cpu, zero => false, subtract => false, half_carry => false, carry => false);
    }

    // Xor
    #[test]
    fn execute_xor_8bit() {
        let cpu = test_instruction!(Instruction::Xor(ArithmeticTarget::A), a => 0b0000_0111);

        assert_eq!(cpu.registers.a, 0x0);
        check_flags!(cpu, zero => true, subtract => false, half_carry => false, carry => false);
    }

    #[test]
    fn execute_xor_8bit_with_zero() {
        let cpu = test_instruction!(Instruction::Xor(ArithmeticTarget::B), a => 0x8);

        assert_eq!(cpu.registers.a, 0x8);
        check_flags!(cpu, zero => false, subtract => false, half_carry => false, carry => false);
    }

    // Cp
    #[test]
    fn execute_cp_8bit_non_underflow_target_a() {
        let cpu = test_instruction!(Instruction::Cp(ArithmeticTarget::A), a => 0x7);

        assert_eq!(cpu.registers.a, 0x7);
        check_flags!(cpu, zero => true, subtract => true, half_carry => false, carry => false);
    }

    #[test]
    fn execute_cp_8bit_non_underflow_target_c() {
        let cpu = test_instruction!(Instruction::Cp(ArithmeticTarget::C), a => 0x7, c => 0x3);

        assert_eq!(cpu.registers.a, 0x7);
        check_flags!(cpu, zero => false, subtract => true, half_carry => false, carry => false);
    }

    #[test]
    fn execute_cp_8bit_non_overflow_target_c_with_carry() {
        let cpu = test_instruction!(Instruction::Cp(ArithmeticTarget::C), a => 0x7, c => 0x3, f.carry => true);

        assert_eq!(cpu.registers.a, 0x7);
        check_flags!(cpu, zero => false, subtract => true, half_carry => false, carry => false);
    }

    #[test]
    fn execute_cp_8bit_carry() {
        let cpu = test_instruction!(Instruction::Cp(ArithmeticTarget::B), a => 0x4, b => 0x9);

        assert_eq!(cpu.registers.a, 0x4);
        check_flags!(cpu, zero => false, subtract => true, half_carry => true, carry => true);
    }

    // RRA
    #[test]
    fn execute_rra_8bit() {
        let cpu = test_instruction!(Instruction::RRA, a => 0b1);

        assert_eq!(cpu.registers.a, 0x0);
        check_flags!(cpu, zero => false, subtract => false, half_carry => false, carry => true);
    }
}
