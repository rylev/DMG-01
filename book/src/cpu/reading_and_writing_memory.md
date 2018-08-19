# Instructions for Reading and Writting to Memory

Now that we've seen how instructions get executed and the very basics of reading from memory in order to fetch the instructions to be read, we'll look now at instructions that are used to read and write from different parts of memory.

## Loading Memory

First, when we talk about reading and writing memory, we usually use the term "load". We'll be loading data from some place to some place - for example, loading the contents of register A into memory at location 0xFF0A or loading register C with the contents from memory location 0x0040. Loading doesn't have to be between a register and a place in memory, it can also be between two registers or even two places in memory.

All of the instructions we'll be looking are called `LD` instructions. We'll be differentiating between the types of loads with the `LoadType` enum. The enum will describe what kind of load we're doing.

Let's take a look at the implementation of the `LD` instruction with the `LoadType` of `Byte` which loads a byte from one place to another.

```rust,noplaypen
# struct Bus {}
# struct Registers { a: u8 }
# struct CPU { registers: Registers, bus: Bus, pc: u16 }
# impl Registers { fn get_hl(&self) -> u16 { 0 } }
# impl CPU { fn read_next_byte(&self) -> u8 { 0 } }
# impl Bus { fn read_byte(&self, addr: u16) -> u8 { 0 }
             fn write_byte(&self, addr: u16, byte: u8) {} }
enum LoadByteTarget {
    A, B, C, D, E, H, L, HLI
}
enum LoadByteSource {
    A, B, C, D, E, H, L, D8, HLI
}
enum LoadType {
  Byte(LoadByteTarget, LoadByteSource),
}
enum Instruction {
  LD(LoadType),
}

impl CPU {
  fn execute(&mut self, instruction: Instruction) -> u16 {
    match instruction {
      Instruction::LD(load_type) => {
        match load_type {
          LoadType::Byte(target, source) => {
            let source_value = match source {
              LoadByteSource::A => self.registers.a,
              LoadByteSource::D8 => self.read_next_byte(),
              LoadByteSource::HLI => self.bus.read_byte(self.registers.get_hl()),
              _ => { panic!("TODO: implement other sources") }
            };
            match target {
              LoadByteTarget::A => self.registers.a = source_value,
              LoadByteTarget::HLI => self.bus.write_byte(self.registers.get_hl(), source_value),
              _ => { panic!("TODO: implement other targets") }
            };
            match source {
              LoadByteSource::D8  => self.pc.wrapping_add(2),
              _                   => self.pc.wrapping_add(1),
            }
          }
          _ => { panic!("TODO: implement other load types") }
        }
      }
      _ => { panic!("TODO: support more instructions") }
    }
  }
}
```

For loads with a register as a source, we simply read the register's value. If the source is a `D8` (meaning "direct 8 bit value"), the value is stored directly after the instruction, so we can simply call `read_next_byte` which reads the byte directly after the byte the program counter is currently pointing to. Lastly, if the source is `HLI` we use the value inside of the `HL` register as an address from which we read an 8 bit value from memory.

The target is merely the reverse of the source (except that we can't have `D8` as a target). If the target is a register, we write the source value into that register, and if the target is `HLI` we write to the address that is stored inside of the `HL` register.

The use of the 16-bit registers `BC`, `DE`, and `HL` to store addresses is very common.

Let's take a look at the other types of loads that there are:
* `Word`: just like the `Byte` type except with 16-bit values
* `AFromIndirect`: load the A register with the contents from a value from a memory location whose address is stored in some location
* `IndirectFromA`: load a memory location whose address is stored in some location with the contents of the A register
* `AFromByteAddress`: Just like `AFromIndirect` except the memory address is some address in the very last byte of memory.
* `ByteAddressFromA`: Just like `IndirectFromA` except the memory address is some address in the very last byte of memory.

For more detail on these instructions checkout the [instruction guide](../appendix/instruction_guide/index.html).

These instructions have been for writing and writing to anywhere in memory, but there are a set of instructions that deal with a specific piece of memory called the stack. Let's take a look at what the stack is and the instructions that are used to manipulate the stack.

## The Stack

Before we can look at the piece of memory in the Game Boy known as the stack, we need to have a good understanding of what a stack is more generally. A stack is a simple data structure that allows you to add values to it (a.k.a "push" values) and then get these values back (a.k.a pop them off the stack). The key thing to remember with a stack is that you pop items off the stack in reverse order from which you pushed the items on - i.e., if you pushed three items "A", "B", "C" on to a stack in that order, the order you will get them back when poping them off is "C", "B", "A".

The Game Boy CPU has built in support for a stack like data structure in memory. This stack lives somewhere in memory (we'll talk about how it's location in memory is set in just a minute), and it holds on to 16 bit values. How is it built?

First, the CPU has an additional 16-bit register on it that indicates the top of the stack. This register is called `SP` or stack pointer because it "points" to where the top of the stack is. Let's add this register to our CPU:

```rust, noplayground
# struct Registers {}
# struct MemoryBus {}
struct CPU {
  registers: Registers,
  pc: u16,
  sp: u16,
  bus: MemoryBus,
}
```

We have a stack pointer now so we know where our stack is, but how do we push and pop from this stack?

The Game Boy's CPU understands two insructions for doing just that. `PUSH` will write the contents of any 16-bit register into the stack and `POP` writes the head of stack into any 16-bit register.

Here's what's actually happening when a `PUSH` is performed:
* _Decrease_ the stack pointer by 1.
* Write the most significant byte of the 16 bit value into memory at the location the stack pointer is now pointing to
* _Decrease_ the stack pointer by 1 again.
* Write the least significant byte of the 16 bit value into memory at the location the stack pointer is now pointing to

Notice that the stack pointer is decresed by 1 and not increased. This is because the stack grows downward in memory. This is extra helpful since the normal place for the stack to live is at the very end of memory. In a later chapter we'll see that it's actually the Game Boy's boot ROM that sets the stack pointer to the very end of memory. Thus, when the stack grows it grows away from the end of memory towards the beginning of memory.

Let's implement `PUSH`:

```rust,noplayground
# struct Registers { }
# impl Registers { fn get_bc(&self) -> u16 { 0 } }
# struct CPU { pc: u16, bus: Bus, sp: u16, registers: Registers }
# struct Bus {}
# impl Bus { fn write_byte(&self, addr: u16, value: u8) { } }
# enum Instruction { PUSH(StackTarget), }
# enum StackTarget { BC, DE }
impl CPU {
  fn execute(&mut self, instruction: Instruction) -> u16 {
    match instruction {
      Instruction::PUSH(target) => {
        let value = match target {
          StackTarget::BC => self.registers.get_bc(),
          _ => { panic!("TODO: support more targets") }
        };
        self.push(value);
        self.pc.wrapping_add(1)
      }
      _ => { panic!("TODO: support more instructions") }
    }
  }

  fn push(&mut self, value: u16) {
    self.sp = self.sp.wrapping_sub(1);
    self.bus.write_byte(self.sp, ((value & 0xFF00) >> 8) as u8);

    self.sp = self.sp.wrapping_sub(1);
    self.bus.write_byte(self.sp, (value & 0xFF) as u8);
  }
}
```

We can now push elements on to the stack. Here's what's actually happening when a `PUSH` is performed:
* Read the least significant byte of the 16 bit value from memory at the location the stack pointer is pointing to
* _Increase_ the stack pointer by 1.
* Read the most significant byte of the 16 bit value from memory at the location the stack pointer is now pointing to
* _Increase_ the stack pointer by 1 again.
* Return the value with the most and least significant byte combined together

Let's write `POP`:

```rust,noplayground
# struct Registers { }
# impl Registers { fn set_bc(&self, value: u16) { } }
# struct CPU { pc: u16, bus: Bus, sp: u16, registers: Registers }
# struct Bus {}
# impl Bus { fn read_byte(&self, addr: u16) -> u8 { 0 } }
# enum Instruction { POP(StackTarget), }
# enum StackTarget { BC, DE }
impl CPU {
  fn execute(&mut self, instruction: Instruction) -> u16 {
    match instruction {
      Instruction::POP(target) => {
        let result = self.pop();
        match target {
            StackTarget::BC => self.registers.set_bc(result),
            _ => { panic!("TODO: support more targets") }
        };
        self.pc.wrapping_add(1)
      }
      _ => { panic!("TODO: support more instructions") }
    }
  }

  fn pop(&mut self) -> u16 {
    let lsb = self.bus.read_byte(self.sp) as u16;
    self.sp = self.sp.wrapping_add(1);

    let msb = self.bus.read_byte(self.sp) as u16;
    self.sp = self.sp.wrapping_add(1);

    (msb << 8) | lsb
  }
}
```

And there we have it! We have a working stack that we can used. But what sort of things is the stack used for? One built in use for the stack is creating a "call" stack that allows the game to "call" functions and return from them. Let's see how that works.

## Calling Functions
