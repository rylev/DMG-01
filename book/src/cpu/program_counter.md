# Program Counter

So far we've seen instructions that can operate on register data. But how does the CPU which instruction to execute? To understand this, we'll first need to understand where our instructions are stored.

## Game ROM

So far we know that the Game Boy has a CPU that executes instructions and it has memory. Memory can be thought of as one very large array of 8-bit numbers.

At the beginning of this very long array are 255 bytes (from index 0x0000 to index 0x00FF) that are hard coded into the Game Boy's circuitry. These 255 bytes are instructions that tell the Game Boy how to "bootstrap" itself (i.e. get itself ready to execute a game) as well as display the [iconic splash screen](https://www.youtube.com/watch?v=ClJWTR_lCL4). Later in the book we'll be looking at specifically what these instructions do, but for now just imagine them as a collection of instructions many of which we learned in the previous chapter and the rest of which we'll learn in this chapter and the next few to come.

When the user of a Game Boy inserts a game cartridge, the contents of that cartridge become available to the CPU right after these 255 bytes. We'll talk later about where other things such as the contents of the screen and graphics data live in memory later in the book. For now we just need to know that the contents of memory starting at index 0x100 until index 0x3FFF include the contents of the cartridge.

So our memory is simply an long array of 8-bit numbers (0xFFFF or 65,536 of them to be exact). Each of these numbers can be decoded as an instruction that our CPU knows how to run. But how does the CPU know which of these to execute?

## The Program Counter

Along with the register data, our CPU also holds on to a 16-bit number called the progam counter (often abbreviated as PC) which tells us which instruction the Game Boy is currently executing. This 16-bit number is capable of addressing of the of 0xFFFF numbers that live in memory. In fact, when we talk about the memory array we don't usually use the term "index", but instead the term "address".

Let's add a program counter to our CPU as well as memory that we can address from the CPU.

```rust,noplayground
# struct Registers {}
struct CPU {
  registers: Registers,
  pc: u16,
  bus: MemoryBus,
}

struct MemoryBus {
  memory: [u8; 0xFFFF]
}

impl MemoryBus {
  fn read_byte(&self, address: u16) -> u8 {
    self.memory[address as usize]
  }
}
```

Ok, so we now have a program counter that can tell us at which address in memory the currently executing instruction is. Now we'll need to actually add the method to the CPU that uses the program counter to read the instruction from memory and execute it.

The full set of steps is as follows:
* Use the program counter to read the instruction byte from memory.
* Translate the byte to one of the instances of the `Instruction` enum
* If we can successfully translate the instruction call our `execute` method else panic which now returns the next program counter
* Set this next program counter on our CPU

```rust,noplayground
# enum Instruction { }
# struct CPU { pc: u16, bus: Bus }
# struct Bus {}
# impl Bus {
#   fn read_byte(&self, a: u16) -> u8 { 0 }
# }
# impl CPU {
#   fn execute(&self, i: Instruction) -> u16 { 0 }
# }
# impl Instruction {
#   fn from_byte(b: u8) -> Option<Instruction> { None }
# }
impl CPU {
  fn step(&mut self) {
    let mut instruction_byte = self.bus.read_byte(self.pc);

    let next_pc = if let Some(instruction) = Instruction::from_byte(instruction_byte) {
      self.execute(instruction)
    } else {
      panic!("Unkown instruction found for: 0x{:x}", instruction_byte);
    };

    self.pc = next_pc;
  }
}
```

So there's two things we'll need to add for the above to work. We'll need to change our execute method to return the next program counter, and we'll need to add a function that takes a byte and returns an `Instruction`. Let's start with latter. Decoding our instruction byte as an `Instruction` is very straight forward. Instructions are uniquely identified by the byte number. For instance, a logical `OR` with the `A` register as its target is identified by the byte 0x87. Want to do an `OR` with the `H` register as the target? That's the number 0xB4. The `SCF` (or Set Carry Flag) instruction is identified by the byte 0x37. We can use our [instruction guide](../appendix/instruction_guide/index.html) to find out which byte value corresponds to which `Instruction`.

```rust,noplayground
# enum IncDecTarget { BC, DE }
# enum Instruction { INC(IncDecTarget) }
impl Instruction {
  fn from_byte(byte: u8) -> Option<Instruction> {
    match byte {
      0x02 => Some(Instruction::INC(IncDecTarget::BC)),
      0x13 => Some(Instruction::INC(IncDecTarget::DE)),
      _ => /* TODO: Add mapping for rest of instructions */ None
    }
  }
}
```

And now let's change our `execute` method so that it now returns the next program counter:

```rust,noplayground
# struct Registers { a:u8, c: u8 }
# struct CPU { pc: u16, registers: Registers }
# enum Instruction { ADD(ArithmeticTarget), }
# enum ArithmeticTarget { A, B, C, D, E, H, L, }
impl CPU {
  fn execute(&mut self, instruction: Instruction) -> u16 {
    match instruction {
      Instruction::ADD(target) => {
        match target {
          ArithmeticTarget::C => {
            let value = self.registers.c;
            let new_value = self.add(value);
            self.registers.a = new_value;
            self.pc.wrapping_add(1)
          }
          _ => { /* TODO: support more targets */ self.pc }
        }
      }
      _ => { /* TODO: support more instructions */ self.pc }
    }
  }
  # fn add(&self, value: u8) -> u8 {
  #   0
  # }
}
```

Now we have the ability to read the instruction byte from memory that's pointed to by our program counter, decode that instruction byte as one of the variants of our `Instruction` enum, execute that instruction and get back the new program counter and finally set the new program counter on our CPU. This is how all instructions in the Game Boy get executed! Well, except...

## Prefix Instructions

The process we've laid out for how instructions get executed is true for roughly half of the total instructions the Game Boy can perform. The other half of instructions work the same way except that instead of being identified by a single byte they're first indentified by a prefix byte. This prefix byte tells the CPU, "Hey! The next instruction byte you read shouldn't be interpreted as a normal instruction, but rather as a prefix instruction".

This prefix byte is the number "0xCB". So, we'll need to add logic that first checks to see if the byte we read from memory is 0xCB. If it is, we then need to read one more byte and interpret this byte as an "prefix instruction". For example, if we read 0xCB from memory, we know that we're going to be decoding a prefix instruction. We then read another byte. If that byte is, say, 0xB4, we should *not* interpret this as `OR` with `H` as the target like we normally would but rather as a `RES` instruction with the 6th bit of the `H` register as the target. Again we can use the [instruction guide](../appendix/instruction_guide/index.html) to help us know what a given byte should decode as.

Let's put it in code!

```rust,noplayground
# enum IncDecTarget { BC }
# enum PrefixTarget { B }
# enum Instruction { INC(IncDecTarget), RLC(PrefixTarget) }
# struct CPU { pc: u16, bus: Bus }
# struct Bus {}
# impl Bus {
#   fn read_byte(&self, a: u16) -> u8 { 0 }
# }
# impl CPU {
#   fn execute(&self, i: Instruction) -> u16 { 0 }
# }
impl CPU {
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
}

impl Instruction {
  fn from_byte(byte: u8, prefixed: bool) -> Option<Instruction> {
    if prefixed {
      Instruction::from_byte_prefixed(byte)
    } else {
      Instruction::from_byte_not_prefixed(byte)
    }
  }

  fn from_byte_prefixed(byte: u8) -> Option<Instruction> {
    match byte {
      0x00 => Some(Instruction::RLC(PrefixTarget::B)),
      _ => /* TODO: Add mapping for rest of instructions */ None
    }
  }

  fn from_byte_not_prefixed(byte: u8) -> Option<Instruction> {
    match byte {
      0x02 => Some(Instruction::INC(IncDecTarget::BC)),
      _ => /* TODO: Add mapping for rest of instructions */ None
    }
  }
}
```

And there we have it. We're now succesfully executing instructions that are stored in memory! We even have the ability to tell the difference between "prefixed" instructions and non-"prefixed" instructions. Next we'll look at bit closer at instructions that read and write to memory.
