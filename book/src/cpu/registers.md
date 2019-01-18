# CPU Registers

In the previous chapter we outlined some of things that the CPU is responsible for. In this chapter we'll be focusing on just one of them: saving small amounts of data into registers.

## Overview

The Game Boy's CPU is a custom chip made just for the Game Boy. The chip is extremely similar to the [Intel 8080](https://en.wikipedia.org/wiki/Intel_8080) which is itself similar to the [Zilog Z80](https://en.wikipedia.org/wiki/Zilog_Z80). While the Intel 8080 and Zilog Z80 were used in many different computers in the 70s and 80s, the chip inside the Game Boy was just used for the Game Boy. Most of what's true about how the 8080 and Z80 work is also true of the Game Boy's chip. We won't go into details on how exactly they differ, but it's important to be aware that while they're similar to the Game Boy's chip, they're not exactly the same.

## Registers

The CPU is composed of 8 different "registers". Registers are responsible for holding on to little pieces of data that the CPU can manipulate when it executes various instructions. The Game Boy's CPU is an 8-Bit CPU, meaning that each of its registers can hold 8 bits (a.k.a 1 byte) of data. The CPU has 8 different registers labled as "a", "b", "c", "d", "e", "f", "h", "l".

Let's get started building our CPU by specing out the registers in code:

```rust,noplaypen
struct Registers {
  a: u8,
  b: u8,
  c: u8,
  d: u8,
  e: u8,
  f: u8,
  h: u8,
  l: u8,
}
```

We use the type `u8` for our registers. `u8` are 8-bit unsigned integers. For a refresher on how numbers are stored in computers, checkout the [guide on numbers](../appendix/numbers.md).

While the CPU only has 8 bit registers, there are instructions that allow the game to read and write 16 bits (i.e. 2 bytes) at the same time (denoted as `u16` in Rust - a 16 bit unsigned integer). Therefore, we'll need the ability to read an write these "virtual" 16 bit registers. These registers are refered to as "af" ("a" and "f" combined), "bc" ("b" and "c" combined), "de" ("d" and "e" combinded), and finally "hl" ("h" and "l" combined). Let's implement "bc":

```rust,noplaypen
# struct Registers { a: u8, b: u8, c: u8, d: u8, e: u8, f: u8, h: u8, l: u8, }
impl Registers {
  fn get_bc(&self) -> u16 {
    (self.b as u16) << 8
    | self.c as u16
  }

  fn set_bc(&mut self, value: u16) {
    self.b = ((value & 0xFF00) >> 8) as u8;
    self.c = (value & 0xFF) as u8;
  }
}
```

Here we see our first instance of "bit manipulation" through the use of four bitwise operators: ">>", "<<", "&", and "|". If you're unfamiliar with or feel a bit rusty using these types of operators, check out the [guide on bit manipulation](../appendix/bit_manipulation.md).

For reading the "bc" register we first treat the "b" register as a `u16` (this effectively just adds a byte of all 0s to the most significant position of the number). We then shift the "b" register 8 positions so that it's occupying the most significant byte position. Finally, we bitwise OR the "c" register. The result is a two byte number with the contents of "b" in the most significant byte position and the contents of "c" in the least significant byte position.

## Flags Register

We're almost done with our registers, but there's one thing we way we can improve our registers for use later. The "f" register is a special register called the "flags" register. The lower four bits of the register are _always_ 0s and the CPU automatically writes to the upper four bits when certain things happen. In other words, the CPU "flags" certain states. We won't go into the specific meanings of the flags just yet, but for now just know that they have the following names and positions:
* Bit 7: "zero"
* Bit 6: "subtraction"
* Bit 5: "half carry"
* Bit 4: "carry"

Here's a diagram of the flags register:
```ignore
   ┌-> Carry
 ┌-+> Subtraction
 | |
1111 0000
| |
└-+> Zero
  └-> Half Carry
```

So while we could continue modeling our flags register as a simple 8-bit number (after all, that's all it is in reality), it might be less error prone to explicitly model the fact that the upper 4 bits (a.k.a the upper "nibble") has specific meaning and the lower 4 bits (a.k.a the lower "nibble") must always be zeros.

For this reason we'll make a struct called the `FlagsRegister`:

```rust,noplaypen
struct FlagsRegister {
    zero: bool,
    subtract: bool,
    half_carry: bool,
    carry: bool
}
```

Since we might need to look at this register as an 8-bit number, we can implement some traits from the standard library that make this easy:

```rust,noplaypen
# struct FlagsRegister {
#    zero: bool,
#    subtract: bool,
#    half_carry: bool,
#    carry: bool
# }
const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

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
```

The `std::convert::From` trait allows us to easily convert our FlagsRegister from a `u8` and back.

Now that we have our special `FlagsRegister`, we can replace the `u8` in our `Registers` struct's `f` field.

And that's it! We have all the functionality we need for our registers. Next we'll be looking at different instructions for manipulating the registers.
