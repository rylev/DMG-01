pub mod flags_register;
pub mod registers;
pub mod instruction;

use self::registers::Registers;
use self::instruction::{
    Instruction,
    IncDecTarget,
    ArithmeticTarget,
    PrefixTarget,
    BitPosition,
    ADDHLTarget
};

/// # Macros
///
/// The following are macros for generating repetitive code needed for processing CPU
/// instructions. For more information on macros read [the chapter in the Rust book](https://doc.rust-lang.org/book/second-edition/appendix-04-macros.html).

// Macro for changing the CPU based on the value of a 8 bit register
macro_rules! manipulate_8bit_register {
    // Macro pattern for getting a value from a register and doing some work on that value
    //
    // # Example Usage:
    // ``` rust
    // manipulate_8bit_register!(self, a => print_register)
    // ```
    //
    // This above reads register `a` and then calls the method `print_register` with the
    // value from `a`
    ( $self:ident : $getter:ident => $work:ident) => {
        {
            let value = $self.registers.$getter;
            $self.$work(value)
        }
    };
    // Macro pattern for getting a value from a register and doing some work on that value and
    // writting it back to the register
    //
    // # Example Usage:
    // ``` rust
    // manipulate_8bit_register!(self, a => increment => d)
    // ```
    //
    // This above reads register `a` and then calls the method `increment` with the
    // value from `a` and then writes the result of `increment` into register `d`
    ( $self:ident : $getter:ident => $work:ident => $setter:ident) => {
        {
            let result = manipulate_8bit_register!($self: $getter => $work);
            $self.registers.$setter = result;
        }
    };
    // Macro pattern for getting a value from a register and doing some work on that value at a
    // specific bit pattern
    //
    // # Example Usage:
    // ``` rust
    // manipulate_8bit_register!(self, a => increment @ BitPosition::B2)
    // ```
    //
    // This above reads register `a` and then calls the method `increment` with the
    // value from `a` and the bit position marker `B2`
    ( $self:ident : ( $register:ident @ $bit_position:ident ) => $work:ident ) => {
        {
            let value = $self.registers.$register;
            $self.$work(value, $bit_position)
        }
    };
    // Macro pattern for getting a value from a register and doing some work on that value at a
    // specific bit pattern and writting it back to the register
    //
    // # Example Usage:
    // ``` rust
    // manipulate_8bit_register!(self, a => increment @ BitPosition::B2 => c)
    // ```
    //
    // This above reads register `a` and then calls the method `increment` with the
    // value from `a` and the bit position marker `B2` and then writes the result of the
    // call to `increment` into the register `c`.
    ( $self:ident : ( $getter:ident @ $bit_position:ident ) => $work:ident => $setter:ident) => {
        {
            let result = manipulate_8bit_register!($self: ( $getter @ $bit_position ) => $work);
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
macro_rules! manipulate_16bit_register {
    ( $self:ident : $getter:ident => $work:ident => $setter:ident ) => {
        {
            let amount = $self.registers.$getter();
            let result = $self.$work(amount);
            $self.registers.$setter(result);
        }
    };
}

macro_rules! arithmetic_instruction {
    // Macro pattern for matching a register and then manipulating the register
    //
    // # Example Usage:
    // ``` rust
    // arithmetic_instruction!(register, self.foo)
    // ```
    //
    // The above matches a register and then calls the function `foo` to do work on the value
    // in that register.
    ( $register:ident, $self:ident.$work:ident) => {
        {
            match $register {
                ArithmeticTarget::A => manipulate_8bit_register!($self: a => $work),
                ArithmeticTarget::B => manipulate_8bit_register!($self: b => $work),
                ArithmeticTarget::C => manipulate_8bit_register!($self: c => $work),
                ArithmeticTarget::D => manipulate_8bit_register!($self: d => $work),
                ArithmeticTarget::E => manipulate_8bit_register!($self: e => $work),
                ArithmeticTarget::H => manipulate_8bit_register!($self: h => $work),
                ArithmeticTarget::L => manipulate_8bit_register!($self: l => $work),
            }
        }
    };
    // Macro pattern for matching a register and then manipulating the register and writing the
    // value back to the a register
    //
    // # Example Usage:
    // ``` rust
    // arithmetic_instruction!(register, self.foo => a)
    // ```
    //
    // The above matches a register and then calls the function `foo` to do work on the value
    // in that register and writes the result of `foo` into the a register.
    ( $register:ident, $self:ident.$work:ident => a) => {
        {
            match $register {
                ArithmeticTarget::A => manipulate_8bit_register!($self: a => $work => a),
                ArithmeticTarget::B => manipulate_8bit_register!($self: b => $work => a),
                ArithmeticTarget::C => manipulate_8bit_register!($self: c => $work => a),
                ArithmeticTarget::D => manipulate_8bit_register!($self: d => $work => a),
                ArithmeticTarget::E => manipulate_8bit_register!($self: e => $work => a),
                ArithmeticTarget::H => manipulate_8bit_register!($self: h => $work => a),
                ArithmeticTarget::L => manipulate_8bit_register!($self: l => $work => a),
            }
        }
    };
}

macro_rules! prefix_instruction {
    // Macro pattern for matching a register and then manipulating the register and writing the
    // value back to the a register
    //
    // # Example Usage:
    // ``` rust
    // prefix_instruction!(register, self.foo => a)
    // ```
    //
    // The above matches a register and then calls the function `foo` to do work on the value
    // in that register and writes the result of `foo` into the `a` register.
    ( $register:ident, $self:ident.$work:ident => reg) => {
        {
            match $register {
                PrefixTarget::A => manipulate_8bit_register!($self: a => $work => a),
                PrefixTarget::B => manipulate_8bit_register!($self: b => $work => b),
                PrefixTarget::C => manipulate_8bit_register!($self: c => $work => c),
                PrefixTarget::D => manipulate_8bit_register!($self: d => $work => d),
                PrefixTarget::E => manipulate_8bit_register!($self: e => $work => e),
                PrefixTarget::H => manipulate_8bit_register!($self: h => $work => h),
                PrefixTarget::L => manipulate_8bit_register!($self: l => $work => l),
            }
        }
    };
    // Macro pattern for matching a register and then manipulating the register at a specific bit
    // location and writing the value back to the a register
    //
    // # Example Usage:
    // ``` rust
    // prefix_instruction!(register, (self.foo @ bit_position) => a)
    // ```
    //
    // The above matches a register and then calls the function `foo` to do work on the value
    // in that register at the bit position `bit_position` and writes the result of `foo` into the `a` register.
    ( $register:ident, ( $self:ident.$work:ident @ $bit_position:ident ) => reg) => {
        {
            match $register {
                PrefixTarget::A => manipulate_8bit_register!($self: (a @ $bit_position) => $work => a),
                PrefixTarget::B => manipulate_8bit_register!($self: (b @ $bit_position) => $work => b),
                PrefixTarget::C => manipulate_8bit_register!($self: (c @ $bit_position) => $work => c),
                PrefixTarget::D => manipulate_8bit_register!($self: (d @ $bit_position) => $work => d),
                PrefixTarget::E => manipulate_8bit_register!($self: (e @ $bit_position) => $work => e),
                PrefixTarget::H => manipulate_8bit_register!($self: (h @ $bit_position) => $work => h),
                PrefixTarget::L => manipulate_8bit_register!($self: (l @ $bit_position) => $work => l),
            }
        }
    };
    // Macro pattern for matching a register and then manipulating the register at a specific bit
    // location
    //
    // # Example Usage:
    // ``` rust
    // prefix_instruction!(register, (self.foo @ bit_position))
    // ```
    //
    // The above matches a register and then calls the function `foo` to do work on the value
    // in that register at the bit position `bit_position`
    ( $register:ident, $self:ident.$work:ident @ $bit_position:ident ) => {
        {
            match $register {
                PrefixTarget::A => manipulate_8bit_register!($self: (a @ $bit_position) => $work),
                PrefixTarget::B => manipulate_8bit_register!($self: (b @ $bit_position) => $work),
                PrefixTarget::C => manipulate_8bit_register!($self: (c @ $bit_position) => $work),
                PrefixTarget::D => manipulate_8bit_register!($self: (d @ $bit_position) => $work),
                PrefixTarget::E => manipulate_8bit_register!($self: (e @ $bit_position) => $work),
                PrefixTarget::H => manipulate_8bit_register!($self: (h @ $bit_position) => $work),
                PrefixTarget::L => manipulate_8bit_register!($self: (l @ $bit_position) => $work),
            }
        }
    };
}

const ROM_BANK_0_SIZE: usize = 0x3fff;
const ROM_BANK_0_END: usize = 0x3fff;
struct MemoryBus {
    rom_bank_0: [u8; ROM_BANK_0_SIZE]
}

impl MemoryBus {
    pub fn new() -> MemoryBus {
        MemoryBus {
            // Note: instead of modeling memory as one array of length 0xFFFF, we'll
            // break memory up into it's logical parts.
            rom_bank_0: [0; ROM_BANK_0_SIZE]
        }
    }
    pub fn read_byte(&self, address: u16) -> u8 {
        let address = address as usize;
        if address < ROM_BANK_0_END {
            self.rom_bank_0[address]
        } else {
            panic!("Reading from unkown part of memory at address #{address}")
        }
    }
}

pub struct CPU {
    pub registers: Registers,
    pc: u16,
    bus: MemoryBus,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            registers: Registers::new(),
            pc: 0x0,
            bus: MemoryBus::new(),
        }
    }

    fn step(&mut self) {
        let mut instruction_byte = self.bus.read_byte(self.pc);

        let prefixed = instruction_byte == 0xCB;
        if prefixed {
            instruction_byte = self.bus.read_byte(self.pc + 1);
        }

        let next_pc = if let Some(instruction) = Instruction::from_byte(instruction_byte, prefixed) {
            self.execute(instruction)
        } else {
            let description = format!("0x{}{:x}", if prefixed { "cb" } else { "" }, instruction_byte);
            panic!("Unkown instruction found for: {}", description)
        };

        self.pc = next_pc;
    }

    pub fn execute(&mut self, instruction: Instruction) -> u16 {
        match instruction {
            Instruction::INC(register) => {
                // DESCRIPTION: (increment) - increment the value in a specific register by 1
                // WHEN: target is 16 bit register
                // PC: +1
                // Z:- S:- H:- C:-
                // ELSE:
                // PC: +1
                // Z:? S:0 H:? C:-
                match register {
                    // 8 bit target
                    IncDecTarget::A => manipulate_8bit_register!(self: a => inc_8bit => a),
                    IncDecTarget::B => manipulate_8bit_register!(self: b => inc_8bit => b),
                    IncDecTarget::C => manipulate_8bit_register!(self: c => inc_8bit => c),
                    IncDecTarget::D => manipulate_8bit_register!(self: d => inc_8bit => d),
                    IncDecTarget::E => manipulate_8bit_register!(self: e => inc_8bit => e),
                    IncDecTarget::H => manipulate_8bit_register!(self: h => inc_8bit => h),
                    IncDecTarget::L => manipulate_8bit_register!(self: l => inc_8bit => l),
                    // 16 bit target
                    IncDecTarget::BC => manipulate_16bit_register!(self: get_bc => inc_16bit => set_bc),
                    IncDecTarget::DE => manipulate_16bit_register!(self: get_de => inc_16bit => set_de),
                    IncDecTarget::HL => manipulate_16bit_register!(self: get_hl => inc_16bit => set_hl),
                }
                self.pc.wrapping_add(1)
            },
            Instruction::DEC(register) => {
                // DESCRIPTION: (decrement) - decrement the value in a specific register by 1
                // WHEN: target is 16 bit register
                // PC: +1
                // Z:- S:- H:- C:-
                // ELSE:
                // PC: +1
                // Z:? S:0 H:? C:-
                match register {
                    // 8 bit target
                    IncDecTarget::A => manipulate_8bit_register!(self: a => dec_8bit => a),
                    IncDecTarget::B => manipulate_8bit_register!(self: b => dec_8bit => b),
                    IncDecTarget::C => manipulate_8bit_register!(self: c => dec_8bit => c),
                    IncDecTarget::D => manipulate_8bit_register!(self: d => dec_8bit => d),
                    IncDecTarget::E => manipulate_8bit_register!(self: e => dec_8bit => e),
                    IncDecTarget::H => manipulate_8bit_register!(self: h => dec_8bit => h),
                    IncDecTarget::L => manipulate_8bit_register!(self: l => dec_8bit => l),
                    // 16 bit target
                    IncDecTarget::BC => manipulate_16bit_register!(self: get_bc => dec_16bit => set_bc),
                    IncDecTarget::DE => manipulate_16bit_register!(self: get_de => dec_16bit => set_de),
                    IncDecTarget::HL => manipulate_16bit_register!(self: get_hl => dec_16bit => set_hl),
                }
                self.pc.wrapping_add(1)
            },
            Instruction::ADD(register) => {
                // DESCRIPTION: (add) - add the value stored in a specific register
                // with the value in the A register
                // PC:+1
                // Z:? S:0 H:? C:?
                arithmetic_instruction!(register, self.add_without_carry => a);
                self.pc.wrapping_add(1)
            },
            Instruction::ADDHL(register) => {
                // DESCRIPTION: (add) - add the value stored in a specific register
                // with the value in the HL register
                // PC:+1
                // Z:- S:0 H:? C:?
                let value = match register {
                    ADDHLTarget::BC => {
                        let value = self.registers.get_bc();
                        self.add_hl(value)
                    }
                    ADDHLTarget::DE => {
                        let value = self.registers.get_de();
                        self.add_hl(value)
                    }
                    ADDHLTarget::HL => {
                        let value = self.registers.get_hl();
                        self.add_hl(value)
                    }
                };
                self.registers.set_hl(value);
                self.pc.wrapping_add(1)
            },
            Instruction::ADC(register) => {
                // DESCRIPTION: (add with carry) - add the value stored in a specific
                // register with the value in the A register and the value in the carry flag
                // PC:+1
                // Z:? S:0 H:? C:?
                arithmetic_instruction!(register, self.add_with_carry => a);
                self.pc.wrapping_add(1)
            },
            Instruction::SUB(register) => {
                // DESCRIPTION: (subtract) - subtract the value stored in a specific register
                // with the value in the A register
                // PC:+1
                // Z:? S:1 H:? C:?
                arithmetic_instruction!(register, self.sub_without_carry => a);
                self.pc.wrapping_add(1)
            },
            Instruction::SBC(register) => {
                // DESCRIPTION: (subtract) - subtract the value stored in a specific register
                // with the value in the A register and the value in the carry flag
                // PC:+1
                // Z:? S:1 H:? C:?
                arithmetic_instruction!(register, self.sub_with_carry => a);
                self.pc.wrapping_add(1)
            },
            Instruction::AND(register) => {
                // DESCRIPTION: (AND) - do a bitwise and on the value in a specific
                // register and the value in the A register
                // PC:+1
                // Z:? S:0 H:1 C:0
                arithmetic_instruction!(register, self.and => a);
                self.pc.wrapping_add(1)
            },
            Instruction::OR(register) => {
                // DESCRIPTION: (OR) - do a bitwise or on the value in a specific
                // register and the value in the A register
                // PC:+1
                // Z:? S:0 H:0 C:0
                arithmetic_instruction!(register, self.or => a);
                self.pc.wrapping_add(1)
            },
            Instruction::XOR(register) => {
                // DESCRIPTION: (XOR) - do a bitwise xor on the value in a specific
                // register and the value in the A register
                // PC:+1
                // Z:? S:0 H:0 C:0
                arithmetic_instruction!(register, self.xor => a);
                self.pc.wrapping_add(1)
            },
            Instruction::CP(register) => {
                // DESCRIPTION: (compare) - just like SUB except the result of the
                // subtraction is not stored back into A
                // PC:+1
                // Z:? S:1 H:? C:?
                arithmetic_instruction!(register, self.compare);
                self.pc.wrapping_add(1)
            },
            Instruction::CCF => {
                // DESCRIPTION: (complement carry flag) - toggle the value of the carry flag
                // PC:+1
                // Z:- S:0 H:0 C:?
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                self.registers.f.carry = !self.registers.f.carry;
                self.pc.wrapping_add(1)
            }
            Instruction::SCF => {
                // DESCRIPTION: (set carry flag) - set the carry flag to true
                // PC:+1
                // Z:- S:0 H:0 C:1
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                self.registers.f.carry = true;
                self.pc.wrapping_add(1)
            }
            Instruction::RRA => {
                // DESCRIPTION: (rotate right A register) - bit rotate A register right through the carry flag
                // PC:+1
                // Z:0 S:0 H:0 C:?
                manipulate_8bit_register!(self: a => rotate_right_through_carry_retain_zero => a);
                self.pc.wrapping_add(1)
            }
            Instruction::RLA => {
                // DESCRIPTION: (rotate left A register) - bit rotate A register left through the carry flag
                // PC:+1
                // Z:0 S:0 H:0 C:?
                manipulate_8bit_register!(self: a => rotate_left_through_carry_retain_zero => a);
                self.pc.wrapping_add(1)
            }
            Instruction::RRCA => {
                // DESCRIPTION: (rotate right A register) - bit rotate A register right (not through the carry flag)
                // PC:+1
                // Z:0 S:0 H:0 C:?
                manipulate_8bit_register!(self: a => rotate_right_retain_zero => a);
                self.pc.wrapping_add(1)
            }
            Instruction::RLCA => {
                // DESCRIPTION: (rotate left A register) - bit rotate A register left (not through the carry flag)
                // PC:+1
                // Z:0 S:0 H:0 C:?
                manipulate_8bit_register!(self: a => rotate_left_retain_zero => a);
                self.pc.wrapping_add(1)
            }
            Instruction::CPL => {
                // DESCRIPTION: (complement) - toggle every bit of the A register
                // PC:+1
                // Z:- S:1 H:1 C:-
                manipulate_8bit_register!(self: a => complement => a);
                self.pc.wrapping_add(1)
            }
            Instruction::BIT(register, bit_position) => {
                // DESCRIPTION: (bit test) - test to see if a specific bit of a specific register is set
                // PC:+2
                // Z:? S:0 H:1 C:-
                prefix_instruction!(register, self.bit_test @ bit_position);
                self.pc.wrapping_add(2)
            }
            Instruction::RES(register, bit_position) => {
                // DESCRIPTION: (bit reset) - set a specific bit of a specific register to 0
                // PC:+2
                // Z:- S:- H:- C:-
                prefix_instruction!(register, (self.reset_bit @ bit_position) => reg);
                self.pc.wrapping_add(2)
            }
            Instruction::SET(register, bit_position) => {
                // DESCRIPTION: (bit set) - set a specific bit of a specific register to 1
                // PC:+2
                // Z:- S:- H:- C:-
                prefix_instruction!(register, (self.set_bit @ bit_position) => reg);
                self.pc.wrapping_add(2)
            }
            Instruction::SRL(register) => {
                // DESCRIPTION: (shift right logical) - bit shift a specific register right by 1
                // PC:+2
                // Z:? S:0 H:0 C:?
                prefix_instruction!(register, self.shift_right_logical => reg);
                self.pc.wrapping_add(2)
            }
            Instruction::RR(register) => {
                // DESCRIPTION: (rotate right) - bit rotate a specific register right by 1 through the carry flag
                // PC:+2
                // Z:? S:0 H:0 C:?
                prefix_instruction!(register, self.rotate_right_through_carry_set_zero => reg);
                self.pc.wrapping_add(2)
            }
            Instruction::RL(register) => {
                // DESCRIPTION: (rotate left) - bit rotate a specific register left by 1 through the carry flag
                // PC:+2
                // Z:? S:0 H:0 C:?
                prefix_instruction!(register, self.rotate_left_through_carry_set_zero => reg);
                self.pc.wrapping_add(2)
            }
            Instruction::RRC(register) => {
                // DESCRIPTION: (rotate right) - bit rotate a specific register right by 1 (not through the carry flag)
                // PC:+2
                // Z:? S:0 H:0 C:?
                prefix_instruction!(register, self.rotate_right_set_zero => reg);
                self.pc.wrapping_add(2)
            }
            Instruction::RLC(register) => {
                // DESCRIPTION: (rotate left) - bit rotate a specific register left by 1 (not through the carry flag)
                // PC:+2
                // Z:? S:0 H:0 C:?
                prefix_instruction!(register, self.rotate_left_set_zero => reg);
                self.pc.wrapping_add(2)
            }
            Instruction::SRA(register) => {
                // DESCRIPTION: (shift right arithmetic) - arithmetic shift a specific register right by 1
                // PC:+2
                // Z:? S:0 H:0 C:?
                prefix_instruction!(register, self.shift_right_arithmetic => reg);
                self.pc.wrapping_add(2)
            }
            Instruction::SLA(register) => {
                // DESCRIPTION: (shift left arithmetic) - arithmetic shift a specific register left by 1
                // PC:+2
                // Z:? S:0 H:0 C:?
                prefix_instruction!(register, self.shift_left_arithmetic => reg);
                self.pc.wrapping_add(2)
            }
            Instruction::SWAP(register) => {
                // DESCRIPTION: switch upper and lower nibble of a specific register
                // PC:+2
                // Z:? S:0 H:0 C:0
                prefix_instruction!(register, self.swap_nibbles => reg);
                self.pc.wrapping_add(2)
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
        // Half Carry is set if adding the lower nibbles of the value and register A
        // together (plus the optional carry bit) result in a value bigger the 0xF.
        // If the result is larger than 0xF than the addition caused a carry from
        // the lower nibble to the upper nibble.
        self.registers.f.half_carry = ((self.registers.a & 0xF) + (value & 0xF) + additional_carry) > 0xF;
        add2
    }

    #[inline(always)]
    fn add_hl(&mut self, value: u16) -> u16 {
        let hl = self.registers.get_hl();
        let (result, carry) = hl.overflowing_add(value);
        self.registers.f.carry = carry;
        self.registers.f.subtract = false;
        // Half carry tests if we flow over the 11th bit i.e. does adding the two
        // numbers together cause the 11th bit to flip
        let mask = 0b111_1111_1111; // mask out bits 11-15
        self.registers.f.half_carry = (value & mask) + (hl & mask) > mask;

        result
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
    fn compare(&mut self, value: u8) {
        self.registers.f.zero = self.registers.a == value;
        self.registers.f.subtract = true;
        // Half Carry is set if subtracting the lower nibbles of the value with register
        // a will result in a value lower than 0x0.  To avoid underflowing in this test,
        // we can check if the lower nibble of a is less than the lower nibble of the value
        self.registers.f.half_carry = (self.registers.a & 0xF) < (value & 0xF);
        self.registers.f.carry = self.registers.a < value;
    }

    #[inline(always)]
    fn rotate_right_through_carry_retain_zero(&mut self, value: u8) -> u8 {
        self.rotate_right_through_carry(value, false)
    }

    #[inline(always)]
    fn rotate_right_through_carry_set_zero(&mut self, value: u8) -> u8 {
        self.rotate_right_through_carry(value, true)
    }

    #[inline(always)]
    fn rotate_right_through_carry(&mut self, value: u8, set_zero: bool) -> u8 {
        let carry_bit = if self.registers.f.carry { 1 } else { 0 } << 7;
        let new_value = carry_bit | (value >> 1);
        self.registers.f.zero = set_zero && new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = value & 0b1 == 0b1;
        new_value
    }

    #[inline(always)]
    fn rotate_left_through_carry_retain_zero(&mut self, value: u8) -> u8 {
        self.rotate_left_through_carry(value, false)
    }

    #[inline(always)]
    fn rotate_left_through_carry_set_zero(&mut self, value: u8) -> u8 {
        self.rotate_left_through_carry(value, true)
    }

    #[inline(always)]
    fn rotate_left_through_carry(&mut self, value: u8, set_zero: bool) -> u8 {
        let carry_bit = if self.registers.f.carry { 1 } else { 0 };
        let new_value = (value << 1) | carry_bit;
        self.registers.f.zero = set_zero && new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = (value & 0x80) == 0x80;
        new_value
    }

    #[inline(always)]
    fn rotate_right_set_zero(&mut self, value: u8) -> u8 {
        self.rotate_right(value, true)
    }

    #[inline(always)]
    fn rotate_right_retain_zero(&mut self, value: u8) -> u8 {
        self.rotate_right(value, false)
    }

    #[inline(always)]
    fn rotate_left_set_zero(&mut self, value: u8) -> u8 {
        self.rotate_left(value, true)
    }

    #[inline(always)]
    fn rotate_left_retain_zero(&mut self, value: u8) -> u8 {
        self.rotate_left(value, false)
    }

    #[inline(always)]
    fn rotate_left(&mut self, value: u8, set_zero: bool) -> u8 {
        let new_value = value.rotate_left(1);
        self.registers.f.zero = set_zero && new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = value & 0x80 == 0x80;
        new_value
    }

    #[inline(always)]
    fn rotate_right(&mut self, value: u8, set_zero: bool) -> u8 {
        let new_value = value.rotate_right(1);
        self.registers.f.zero = set_zero && new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = value & 0b1 == 0b1;
        new_value
    }

    #[inline(always)]
    fn complement(&mut self, value: u8) -> u8 {
        let new_value = !value;
        self.registers.f.subtract = true;
        self.registers.f.half_carry = true;
        new_value
    }

    #[inline(always)]
    fn bit_test(&mut self, value: u8, bit_position: BitPosition) {
        let bit_position: u8 = bit_position.into();
        let result = (value >> bit_position) & 0b1;
        self.registers.f.zero = result == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = true;

    }

    #[inline(always)]
    fn reset_bit(&mut self, value: u8, bit_position: BitPosition) -> u8 {
        let bit_position: u8 = bit_position.into();
        value & !(1 << bit_position)
    }

    #[inline(always)]
    fn set_bit(&mut self, value: u8, bit_position: BitPosition) -> u8 {
        let bit_position: u8 = bit_position.into();
        value | (1 << bit_position)
    }

    #[inline(always)]
    fn shift_right_logical(&mut self, value: u8) -> u8 {
        let new_value = value >> 1;
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = value & 0b1 == 0b1;
        new_value
    }

    #[inline(always)]
    fn shift_right_arithmetic(&mut self, value: u8) -> u8 {
        let msb = value & 0x80;
        let new_value = msb | (value >> 1);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = value & 0b1 == 0b1;
        new_value
    }

    #[inline(always)]
    fn shift_left_arithmetic(&mut self, value: u8) -> u8 {
        let new_value = value << 1;
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = value & 0x80 == 0x80;
        new_value
    }

    #[inline(always)]
    fn swap_nibbles(&mut self, value: u8) -> u8 {
        let new_value = ((value & 0xf) << 4) | ((value & 0xf0) >> 4);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = false;
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

    // INC
    #[test]
    fn execute_inc_8bit_non_overflow() {
        let cpu = test_instruction!(Instruction::INC(IncDecTarget::A), a => 0x7);

        assert_eq!(cpu.registers.a, 0x8);
        check_flags!(cpu, zero => false, subtract => false, half_carry => false, carry => false);
    }

    #[test]
    fn execute_inc_8bit_half_carry() {
        let cpu = test_instruction!(Instruction::INC(IncDecTarget::A), a => 0xF);

        assert_eq!(cpu.registers.a, 0x10);
        check_flags!(cpu, zero => false, subtract => false, half_carry => true, carry => false);
    }

    #[test]
    fn execute_inc_8bit_overflow() {
        let cpu = test_instruction!(Instruction::INC(IncDecTarget::A), a => 0xFF);

        assert_eq!(cpu.registers.a, 0x0);
        check_flags!(cpu, zero => true, subtract => false, half_carry => true, carry => false);
    }

    #[test]
    fn execute_inc_16bit_byte_overflow() {
        let instruction = Instruction::INC(IncDecTarget::BC);
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
        let instruction = Instruction::INC(IncDecTarget::BC);
        let mut cpu = CPU::new();
        cpu.registers.set_bc(0xFFFF);
        cpu.execute(instruction);

        assert_eq!(cpu.registers.get_bc(), 0x0);
        assert_eq!(cpu.registers.b, 0x00);
        assert_eq!(cpu.registers.c, 0x00);
        check_flags!(cpu, zero => false, subtract => false, half_carry => false, carry => false);
    }

    // DEC
    #[test]
    fn execute_dec_8bit_non_overflow() {
        let cpu = test_instruction!(Instruction::DEC(IncDecTarget::A), a => 0x7);

        assert_eq!(cpu.registers.a, 0x6);
        check_flags!(cpu, zero => false, subtract => true, half_carry => false, carry => false);
    }

    #[test]
    fn execute_dec_8bit_half_carry() {
        let cpu = test_instruction!(Instruction::DEC(IncDecTarget::A), a => 0x80);

        assert_eq!(cpu.registers.a, 0x7f);
        check_flags!(cpu, zero => false, subtract => true, half_carry => true, carry => false);
    }

    #[test]
    fn execute_dec_8bit_underflow() {
        let cpu = test_instruction!(Instruction::DEC(IncDecTarget::A), a => 0x0);

        assert_eq!(cpu.registers.a, 0xFF);
        check_flags!(cpu, zero => false, subtract => true, half_carry => true, carry => false);
    }

    #[test]
    fn execute_dec_16bit_underflow() {
        let instruction = Instruction::DEC(IncDecTarget::BC);
        let mut cpu = CPU::new();
        cpu.registers.set_bc(0x0000);
        cpu.execute(instruction);

        assert_eq!(cpu.registers.get_bc(), 0xFFFF);
        assert_eq!(cpu.registers.b, 0xFF);
        assert_eq!(cpu.registers.c, 0xFF);
        check_flags!(cpu, zero => false, subtract => false, half_carry => false, carry => false);
    }

    // ADD
    #[test]
    fn execute_add_8bit_non_overflow_target_a() {
        let cpu = test_instruction!(Instruction::ADD(ArithmeticTarget::A), a => 0x7);

        assert_eq!(cpu.registers.a, 0xe);
        check_flags!(cpu, zero => false, subtract => false, half_carry => false, carry => false);
    }

    #[test]
    fn execute_add_8bit_non_overflow_target_c() {
        let cpu = test_instruction!(Instruction::ADD(ArithmeticTarget::C), a => 0x7, c => 0x3);

        assert_eq!(cpu.registers.a, 0xa);
        check_flags!(cpu, zero => false, subtract => false, half_carry => false, carry => false);
    }

    #[test]
    fn execute_add_8bit_non_overflow_target_c_with_carry() {
        let cpu = test_instruction!(Instruction::ADD(ArithmeticTarget::C), a => 0x7, c => 0x3, f.carry => true);

        assert_eq!(cpu.registers.a, 0xa);
        check_flags!(cpu, zero => false, subtract => false, half_carry => false, carry => false);
    }

    #[test]
    fn execute_add_8bit_carry() {
        let cpu = test_instruction!(Instruction::ADD(ArithmeticTarget::B), a => 0xFC, b => 0x9);

        assert_eq!(cpu.registers.a, 0x05);
        check_flags!(cpu, zero => false, subtract => false, half_carry => true, carry => true);
    }

    // ADC
    #[test]
    fn execute_addc_8bit_non_overflow_target_a_no_carry() {
        let cpu = test_instruction!(Instruction::ADD(ArithmeticTarget::A), a => 0x7);

        assert_eq!(cpu.registers.a, 0xe);
        check_flags!(cpu, zero => false, subtract => false, half_carry => false, carry => false);
    }

    #[test]
    fn execute_addc_8bit_non_overflow_target_a_with_carry() {
        let cpu = test_instruction!(Instruction::ADC(ArithmeticTarget::A), a => 0x7, f.carry => true);

        assert_eq!(cpu.registers.a, 0xf);
        check_flags!(cpu, zero => false, subtract => false, half_carry => false, carry => false);
    }

    #[test]
    fn execute_addc_8bit_non_overflow_target_c_with_carry() {
        let cpu = test_instruction!(Instruction::ADC(ArithmeticTarget::C), a => 0x7, c => 0x3, f.carry => true);

        assert_eq!(cpu.registers.a, 0xb);
        check_flags!(cpu, zero => false, subtract => false, half_carry => false, carry => false);
    }

    #[test]
    fn execute_addc_8bit_carry_with_carry() {
        let cpu = test_instruction!(Instruction::ADC(ArithmeticTarget::B), a => 0xFC, b => 0x3, f.carry => true);

        assert_eq!(cpu.registers.a, 0x00);
        check_flags!(cpu, zero => true, subtract => false, half_carry => true, carry => true);
    }

    // SUB
    #[test]
    fn execute_sub_8bit_non_underflow_target_a() {
        let cpu = test_instruction!(Instruction::SUB(ArithmeticTarget::A), a => 0x7);

        assert_eq!(cpu.registers.a, 0x0);
        check_flags!(cpu, zero => true, subtract => true, half_carry => false, carry => false);
    }

    #[test]
    fn execute_sub_8bit_non_underflow_target_c() {
        let cpu = test_instruction!(Instruction::SUB(ArithmeticTarget::C), a => 0x7, c => 0x3);

        assert_eq!(cpu.registers.a, 0x4);
        check_flags!(cpu, zero => false, subtract => true, half_carry => false, carry => false);
    }

    #[test]
    fn execute_sub_8bit_non_overflow_target_c_with_carry() {
        let cpu = test_instruction!(Instruction::SUB(ArithmeticTarget::C), a => 0x7, c => 0x3, f.carry => true);

        assert_eq!(cpu.registers.a, 0x4);
        check_flags!(cpu, zero => false, subtract => true, half_carry => false, carry => false);
    }

    #[test]
    fn execute_sub_8bit_carry() {
        let cpu = test_instruction!(Instruction::SUB(ArithmeticTarget::B), a => 0x4, b => 0x9);

        assert_eq!(cpu.registers.a, 0xFB);
        check_flags!(cpu, zero => false, subtract => true, half_carry => true, carry => true);
    }

    // SBC
    #[test]
    fn execute_subc_8bit_non_overflow_target_a_no_carry() {
        let cpu = test_instruction!(Instruction::SBC(ArithmeticTarget::A), a => 0x7);

        assert_eq!(cpu.registers.a, 0x0);
        check_flags!(cpu, zero => true, subtract => true, half_carry => false, carry => false);
    }

    #[test]
    fn execute_subc_8bit_non_overflow_target_a_with_carry() {
        let cpu = test_instruction!(Instruction::SBC(ArithmeticTarget::A), a => 0x7, f.carry => true);

        assert_eq!(cpu.registers.a, 0xFF);
        check_flags!(cpu, zero => false, subtract => true, half_carry => true, carry => true);
    }

    #[test]
    fn execute_subc_8bit_non_overflow_target_c_with_carry() {
        let cpu = test_instruction!(Instruction::SBC(ArithmeticTarget::C), a => 0x7, c => 0x3, f.carry => true);

        assert_eq!(cpu.registers.a, 0x3);
        check_flags!(cpu, zero => false, subtract => true, half_carry => false, carry => false);
    }

    // AND
    #[test]
    fn execute_and_8bit() {
        let cpu = test_instruction!(Instruction::AND(ArithmeticTarget::A), a => 0x7);

        assert_eq!(cpu.registers.a, 0x7);
        check_flags!(cpu, zero => false, subtract => false, half_carry => true, carry => false);
    }

    #[test]
    fn execute_and_8bit_with_zero() {
        let cpu = test_instruction!(Instruction::AND(ArithmeticTarget::B), a => 0x8);

        assert_eq!(cpu.registers.a, 0x0);
        check_flags!(cpu, zero => true, subtract => false, half_carry => true, carry => false);
    }

    // OR
    #[test]
    fn execute_or_8bit() {
        let cpu = test_instruction!(Instruction::OR(ArithmeticTarget::A), a => 0x7);

        assert_eq!(cpu.registers.a, 0x7);
        check_flags!(cpu, zero => false, subtract => false, half_carry => false, carry => false);
    }

    #[test]
    fn execute_or_8bit_with_zero() {
        let cpu = test_instruction!(Instruction::OR(ArithmeticTarget::B), a => 0x8);

        assert_eq!(cpu.registers.a, 0x8);
        check_flags!(cpu, zero => false, subtract => false, half_carry => false, carry => false);
    }

    // XOR
    #[test]
    fn execute_xor_8bit() {
        let cpu = test_instruction!(Instruction::XOR(ArithmeticTarget::A), a => 0b0000_0111);

        assert_eq!(cpu.registers.a, 0x0);
        check_flags!(cpu, zero => true, subtract => false, half_carry => false, carry => false);
    }

    #[test]
    fn execute_xor_8bit_with_zero() {
        let cpu = test_instruction!(Instruction::XOR(ArithmeticTarget::B), a => 0x8);

        assert_eq!(cpu.registers.a, 0x8);
        check_flags!(cpu, zero => false, subtract => false, half_carry => false, carry => false);
    }

    // CP
    #[test]
    fn execute_cp_8bit_non_underflow_target_a() {
        let cpu = test_instruction!(Instruction::CP(ArithmeticTarget::A), a => 0x7);

        assert_eq!(cpu.registers.a, 0x7);
        check_flags!(cpu, zero => true, subtract => true, half_carry => false, carry => false);
    }

    #[test]
    fn execute_cp_8bit_non_underflow_target_c() {
        let cpu = test_instruction!(Instruction::CP(ArithmeticTarget::C), a => 0x7, c => 0x3);

        assert_eq!(cpu.registers.a, 0x7);
        check_flags!(cpu, zero => false, subtract => true, half_carry => false, carry => false);
    }

    #[test]
    fn execute_cp_8bit_non_overflow_target_c_with_carry() {
        let cpu = test_instruction!(Instruction::CP(ArithmeticTarget::C), a => 0x7, c => 0x3, f.carry => true);

        assert_eq!(cpu.registers.a, 0x7);
        check_flags!(cpu, zero => false, subtract => true, half_carry => false, carry => false);
    }

    #[test]
    fn execute_cp_8bit_carry() {
        let cpu = test_instruction!(Instruction::CP(ArithmeticTarget::B), a => 0x4, b => 0x9);

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

    // RLA
    #[test]
    fn execute_rla_8bit() {
        let cpu = test_instruction!(Instruction::RLA, a => 0x80);

        assert_eq!(cpu.registers.a, 0x0);
        check_flags!(cpu, zero => false, subtract => false, half_carry => false, carry => true);
    }

    // RRCA
    #[test]
    fn execute_rrca_8bit() {
        let cpu = test_instruction!(Instruction::RRCA, a => 0b1, f.carry => true);

        assert_eq!(cpu.registers.a, 0x80);
        check_flags!(cpu, zero => false, subtract => false, half_carry => false, carry => true);
    }

    // RLCA
    #[test]
    fn execute_rlca_8bit() {
        let cpu = test_instruction!(Instruction::RLCA, a => 0x80, f.carry => true);

        assert_eq!(cpu.registers.a, 0x1);
        check_flags!(cpu, zero => false, subtract => false, half_carry => false, carry => true);
    }

    // CPL
    #[test]
    fn execute_cpl_8bit() {
        let cpu = test_instruction!(Instruction::CPL, a => 0b1011_0100);

        assert_eq!(cpu.registers.a, 0b0100_1011);
        check_flags!(cpu, zero => false, subtract => true, half_carry => true, carry => false);
    }

    // BIT
    #[test]
    fn execute_bit_8bit() {
        let cpu = test_instruction!(Instruction::BIT(PrefixTarget::A, BitPosition::B2), a => 0b1011_0100);

        assert_eq!(cpu.registers.a, 0b1011_0100);
        check_flags!(cpu, zero => false, subtract => false, half_carry => true, carry => false);

        let cpu = test_instruction!(Instruction::BIT(PrefixTarget::A, BitPosition::B1), a => 0b1011_0100);
        assert_eq!(cpu.registers.a, 0b1011_0100);
        check_flags!(cpu, zero => true, subtract => false, half_carry => true, carry => false);
    }

    // RES
    #[test]
    fn execute_res_8bit() {
        let cpu = test_instruction!(Instruction::RES(PrefixTarget::A, BitPosition::B2), a => 0b1011_0100);

        assert_eq!(cpu.registers.a, 0b1011_0000);
        check_flags!(cpu, zero => false, subtract => false, half_carry => false, carry => false);

        let cpu = test_instruction!(Instruction::RES(PrefixTarget::A, BitPosition::B1), a => 0b1011_0100);
        assert_eq!(cpu.registers.a, 0b1011_0100);
        check_flags!(cpu, zero => false, subtract => false, half_carry => false, carry => false);
    }

    // SET
    #[test]
    fn execute_set_8bit() {
        let cpu = test_instruction!(Instruction::SET(PrefixTarget::A, BitPosition::B2), a => 0b1011_0100);

        assert_eq!(cpu.registers.a, 0b1011_0100);
        check_flags!(cpu, zero => false, subtract => false, half_carry => false, carry => false);

        let cpu = test_instruction!(Instruction::SET(PrefixTarget::A, BitPosition::B1), a => 0b1011_0100);
        assert_eq!(cpu.registers.a, 0b1011_0110);
        check_flags!(cpu, zero => false, subtract => false, half_carry => false, carry => false);
    }

    // SRL
    #[test]
    fn execute_srl_8bit() {
        let cpu = test_instruction!(Instruction::SRL(PrefixTarget::A), a => 0b1011_0101);

        assert_eq!(cpu.registers.a, 0b0101_1010);
        check_flags!(cpu, zero => false, subtract => false, half_carry => false, carry => true);
    }

    // RR
    #[test]
    fn execute_rr() {
        let cpu = test_instruction!(Instruction::RR(PrefixTarget::A), a => 0b1011_0101);

        assert_eq!(cpu.registers.a, 0b0101_1010);
        check_flags!(cpu, zero => false, subtract => false, half_carry => false, carry => true);

        let cpu = test_instruction!(Instruction::RR(PrefixTarget::A), a => 0b1011_0101, f.carry => true);

        assert_eq!(cpu.registers.a, 0b1101_1010);
        check_flags!(cpu, zero => false, subtract => false, half_carry => false, carry => true);
    }

    // RL
    #[test]
    fn execute_rl() {
        let cpu = test_instruction!(Instruction::RL(PrefixTarget::A), a => 0b1011_0101);

        assert_eq!(cpu.registers.a, 0b0110_1010);
        check_flags!(cpu, zero => false, subtract => false, half_carry => false, carry => true);

        let cpu = test_instruction!(Instruction::RL(PrefixTarget::A), a => 0b1011_0101, f.carry => true);

        assert_eq!(cpu.registers.a, 0b0110_1011);
        check_flags!(cpu, zero => false, subtract => false, half_carry => false, carry => true);
    }

    // SRA
    #[test]
    fn execute_sra() {
        let cpu = test_instruction!(Instruction::SRA(PrefixTarget::A), a => 0b1011_0101);

        assert_eq!(cpu.registers.a, 0b1101_1010);
        check_flags!(cpu, zero => false, subtract => false, half_carry => false, carry => true);
    }

    // SLA
    #[test]
    fn execute_sla() {
        let cpu = test_instruction!(Instruction::SLA(PrefixTarget::A), a => 0b1011_0101);

        assert_eq!(cpu.registers.a, 0b0110_1010);
        check_flags!(cpu, zero => false, subtract => false, half_carry => false, carry => true);
    }

    // SWAP
    #[test]
    fn execute_swap() {
        let cpu = test_instruction!(Instruction::SWAP(PrefixTarget::A), a => 0b1011_0101);

        assert_eq!(cpu.registers.a, 0b0101_1011);
        check_flags!(cpu, zero => false, subtract => false, half_carry => false, carry => false);
    }

    // -----------------------------------------------------------------------------

    // Step
    #[test]
    fn test_step() {
        let mut cpu = CPU::new();
        cpu.bus.rom_bank_0[0] = 0x23; //INC(HL)
        cpu.bus.rom_bank_0[1] = 0xB5; //OR(L)
        cpu.bus.rom_bank_0[2] = 0xCB; //PREFIX
        cpu.bus.rom_bank_0[3] = 0xe8; //SET(B, 5)
        for _ in 0..3 {
            cpu.step();
        }

        assert_eq!(cpu.registers.h, 0b0);
        assert_eq!(cpu.registers.l, 0b1);
        assert_eq!(cpu.registers.a, 0b1);
        assert_eq!(cpu.registers.b, 0b0010_0000);
    }
}
