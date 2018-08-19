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
