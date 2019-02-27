# Instructions on Register Data

The first instructions we'll be examining are instructions that just operate on and manipulate register data.

## ADD

We'll start by looking closely at how the `ADD` instructions work. This is a simple instruction that adds specific register's contents to the A register's contents. Once we know how this instruction works, it won't be much work to extend the CPU to support all other instructions that just operate on register data.

### Definition

First we need to define the instruction. We'll get into how the game code actually encodes instructions and where the instructions come from later on. For now we're just focusing on the instruction itself and how it affects the CPU registers.

The first thing to do is to define an enum called `Instruction`. This enum will be the central place where all of our instructions will be defined. Our `ADD` instruction needs to include information on which register they're targeting so we'll make sure to include it by associating the instruction with `ArithmeticTarget` enum that specifies the target register. `ADD` can target all of the 8 bit registers except f.

```rust,noplaypen
enum Instruction {
  ADD(ArithmeticTarget),
}

enum ArithmeticTarget {
  A, B, C, D, E, H, L,
}
```

### Executing the Instruction

Ok, now that we have ths instruction, we'll need a way to execute it. Let's create a method on CPU that takes an instruction and executes it. This method will take a mutable reference to the CPU since instructions always mutate the CPU's state. The method will also take the instruction it will execute. We'll pattern match on the instruction and the target register, and then we'll do the appropriate action according to the instruction and the register:

```rust,noplaypen
# struct CPU {}
# enum Instruction { ADD(ArithmeticTarget), }
# enum ArithmeticTarget { A, B, C, D, E, H, L, }
impl CPU {
  fn execute(&mut self, instruction: Instruction) {
    match instruction {
      Instruction::ADD(target) => {
        match target {
          ArithmeticTarget::C => {
            // TODO: implement ADD on register C
          }
          _ => { /* TODO: support more targets */ }
        }
      }
      _ => { /* TODO: support more instructions */ }
    }
  }
}
```

We now have the boiler plate for figuring out which instruction and which target register. Let's see now what we have to do to the actual CPU. The steps for `ADD` for 8 bit target registers are the following:
* Read the current value from the target register
* Add the value to the value in the A register making sure to handle overflow properly
* Update the flags register
* Write the updated value to the A register

Let's implement it with C as the target register:

```rust,noplaypen
# struct Registers { a:u8, c: u8 }
# struct CPU { registers: Registers }
# enum Instruction { ADD(ArithmeticTarget), }
# enum ArithmeticTarget { A, B, C, D, E, H, L, }
impl CPU {
  fn execute(&mut self, instruction: Instruction) {
    match instruction {
      Instruction::ADD(target) => {
        match target {
          ArithmeticTarget::C => {
            let value = self.registers.c;
            let new_value = self.add(value);
            self.registers.a = new_value;
          }
          _ => { /* TODO: support more targets */ }
        }
      }
      _ => { /* TODO: support more instructions */ }
    }
  }

  fn add(&mut self, value: u8) -> u8 {
    let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
    // TODO: set flags
    new_value
  }
}
```

Notice that we use the `overflowing_add` method on our 8 bit value instead of `+`. This is because `+` panics in development when the result of the addition overflows. Rust forces us to be explicit about the behaivor we want, we chose `overflowing_add` because it properly overflows the value, and it informs us if the addition actually resulted in an overflow or not. This will be important information for when we update the flags register.

### Setting Flags

There are four flags defined on the flags register:
* Zero: set to true if the result of the operation is equal to 0.
* Subtract: set to true if the operation was a subtraction.
* Carry: set to true if the operation resulted in an overflow.
* Half Carry: set to true if there is an overflow from the lower nibble (a.k.a the lower four bits) to the upper nibble (a.k.a the upper four bits). Let's take a look at some examples of what this means. In the following diagram, we have the byte 143 in binary (0b1000_1111). We then add 0b1 to the number. Notice how the 1 from the lower nibble is carried to the upper nibble. You should already be familiar with carries from elemetry arithmetic. Whenever there's not enough room for a number in a particular digit's place, we carry over to the next digits place.
  ```ignore
        lower nibble            lower nibble
           ┌--┐                    ┌--┐
      1000 1111  +   1   ==   1001 0000
      └--┘                    └--┘
  upper nibble            upper nibble
  ```

  If this happens when adding our values, we set the half_carry flag to true. We can test for this by masking out the upper nibble of both the A register and the value we're adding and testing if this value is greater than 0xF.

So let's take a look at the code:

```rust,noplaypen
# struct FlagsRegister { zero: bool, subtract: bool, half_carry: bool, carry: bool }
# struct Registers { a: u8, f: FlagsRegister }
# struct CPU { registers: Registers }
impl CPU {
  fn add(&mut self, value: u8) -> u8 {
    let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
    self.registers.f.zero = new_value == 0;
    self.registers.f.subtract = false;
    self.registers.f.carry = did_overflow;
    // Half Carry is set if adding the lower nibbles of the value and register A
    // together result in a value bigger than 0xF. If the result is larger than 0xF
    // than the addition caused a carry from the lower nibble to the upper nibble.
    self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
    new_value
  }
}
```

## How Do We Know?

Yout might be wondering, "how do we know what to do given a certain the instruction". The short answer is that this is just how the chip was specified  and manufactured to worked. We know this because people have either read the original user's manual for the Game Boy's CPU chip (known as a "data sheet"),or they've written test programs for the chip that call specific instructions and see what happens. Luckily you don't need to do this. You can find descriptions of all the instructions [in the instruction guide](../appendix/instruction_guide/index.html).

> *Side Note*
>
> Most CPU instructions that deal with register data manipulate that data through various bitwise operations. If the likes of logical shifts and bitwise ands aren't super clear to you, check out the [guide on bit manipulation](./appendix/bit_manipulation.md).

What are the other types of instructions that act on register data?

* **ADDHL** (add to HL) - just like ADD except that the target is added to the HL register
* **ADC** (add with carry) - just like ADD except that the value of the carry flag is also added to the number
* **SUB** (subtract) - subtract the value stored in a specific register with the value in the A register
* **SBC** (subtract with carry) - just like ADD except that the value of the carry flag is also subtracted from the number
* **AND** (logical and) - do a bitwise and on the value in a specific register and the value in the A register
* **OR** (logical or) - do a bitwise or on the value in a specific register and the value in the A register
* **XOR** (logical xor) - do a bitwise xor on the value in a specific register and the value in the A register
* **CP** (compare) - just like SUB except the result of the subtraction is not stored back into A
* **INC** (increment) - increment the value in a specific register by 1
* **DEC** (decrement) - decrement the value in a specific register by 1
* **CCF** (complement carry flag) - toggle the value of the carry flag
* **SCF** (set carry flag) - set the carry flag to true
* **RRA** (rotate right A register) - bit rotate A register right through the carry flag
* **RLA** (rotate left A register) - bit rotate A register left through the carry flag
* **RRCA** (rotate right A register) - bit rotate A register right (not through the carry flag)
* **RLCA** (rotate left A register) - bit rotate A register left (not through the carry flag)
* **CPL** (complement) - toggle every bit of the A register
* **BIT** (bit test) - test to see if a specific bit of a specific register is set
* **RESET** (bit reset) - set a specific bit of a specific register to 0
* **SET** (bit set) - set a specific bit of a specific register to 1
* **SRL** (shift right logical) - bit shift a specific register right by 1
* **RR** (rotate right) - bit rotate a specific register right by 1 through the carry flag
* **RL** (rotate left) - bit rotate a specific register left by 1 through the carry flag
* **RRC** (rotate right) - bit rotate a specific register right by 1 (not through the carry flag)
* **RLC** (rotate left) - bit rotate a specific register left by 1 (not through the carry flag)
* **SRA** (shift right arithmetic) - arithmetic shift a specific register right by 1
* **SLA** (shift left arithmetic) - arithmetic shift a specific register left by 1
* **SWAP** (swap nibbles) - switch upper and lower nibble of a specific register

Reading through the guide on instructions, should give you enough information to implement all the instructions yourself.

Next, we'll be looking at how the CPU keeps track of which instructions to execute as well as different types of instructions that can change where we are in a particular program.
