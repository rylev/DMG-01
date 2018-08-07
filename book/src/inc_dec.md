# Inc/Dec Instructions

The first instructions we'll be examining are inc and dec. These are simple instructions that increment or decrement a specific register by 1.

## Definition

We'll start by defining our instructions. We'll get into how the game code actually encode's these instructions and where the instructions come from later on. For now we're just focusing on the instruction itself and how it affects the CPU registers.

The first thing to do is to define an enum called `Instruction`. This enum will be the central place where all of our instructions will be defined. Our `Inc` and `Dec` instructions need to include information on which register they're targeting so we'll make sure to include it by associating the instruction with `IncDecTarget` enum that specifies the target register. `Inc` and `Dec` can target all of the 8 bit registers except f and all of the 16 bit registers except af.

```rust
enum Instruction {
  Inc(IncDecTarget),
  Dec(IncDecTarget)
}

enum IncDecTarget {
  A, B, C, D, E, H, L,
  BC, DE, HL
}
```

Ok, now that we have these two instructions, we'll need a way to execute. Let's create a method on CPU that takes an instruction and executes it. This method will take a mutable reference to the CPU since instructions most almost always mutate the CPU's state. The method will also take the instruction it will execute. We'll pattern match on the instruction and the target register, and then we'll do the appropriate action according to the instruction and the register:

```rust
impl CPU {
  ...
  fn execute(&mut self, instruction: Instruction) {
    match instruction {
      Instruction::Inc(target) => {
        match target {
          IncDecTarget::A => {
            // TODO: implement Inc on A register
          }
          _ => // TODO: support more targets
        }
      }
      _ => // TODO: support more instructions
    }
  }
  ...
}
```

We now have the boiler plate for figuring out which instruction and which target register. Let's see now what we have to do to the actual CPU. The steps for `Inc` for 8 bit target registers are the following:
* Read the current value from the target register
* Increment the value by one
* Update the flags register
* Write the updated value to the target register

Let's implement it with A as the target register:

```rust
...
    match instruction {
      Instruction::Inc(target) => {
        match target {
          IncDecTarget::A => {
            let value = self.registers.a;
            let new_value = self.inc_8bit(value);
            self.registers.a = new_value;
          }
          _ => // TODO: support more targets
        }
        _ => // TODO: support more instructions
      }
    }
...
  fn inc_8bit(&mut self, value: u8) {
    let new_value = value.wrapping_add(1);
    self.registers.f.zero = new_value == 0;
    self.registers.f.subtract = false;
    self.registers.f.half_carry = value & 0xF == 0xF;
    new_value
  }
...
```

There's a couple of things to point out in the `inc_8bit` method:

First, we use the `wrapping_add` method on our 8 bit value instead of `+`. This is because `+` panics in development when the result of the addition overflows. This is to prevent accidental overflows which can be a source of common bugs. Overflowing is what we want and is what `wrapping_add` gives us without panics.

Second, we set three flags as a result of the inc instruction: zero, subtract and half_carry. The zero flag is set to true if our new value is equal to 0.The subtract flag is always set to false since we know we're not subtracting.

The half_carry flag is a bit more involved. The half_carry is set to true if there is a carry from the lower nibble (a.k.a the lower four bits) to the upper nibble (a.k.a the upper four bits). Let's take a look at some examples of what this means. In the following diagram, we have the byte 143 in binary (0b1000_1111). We then add one to the number. Notice how the 1 from the lower nibble is carried to the upper nibble. This is a carry. You should already be familiar with carries from elemetry arithmetic. Whenever there's not enough room for a number in a particular digit's place, we carry over to the next digits place.
```
      lower nibble            lower nibble
         ┌--┐                    ┌--┐
    1000 1111    + 1   ==   1001 0000
    └--┘                    └--┘
upper nibble            upper nibble
```

If this happens when incrementing our value, we set the half_carry flag to true. This is very easy to test for since the only way this can happen is if the lower nibble of the original value was `0b1111`. We can just mask out the upper nibble and test to see if the number is equal to 0b1111 a.k.a. 0xF.

## How Do We Know?

Yout might be wondering, "how do we know what to do given a certain the instruction". The short answer is that this is just how the chip was specified  and manufactured to worked. We know this because people have either read the original user's manual for the Game Boy's CPU chip (known as a "data sheet"),or they've written test programs for the chip that call specific instructions and see what happens. Luckily you don't need to do this. You can find descriptions of all the instructions [in the instruction guide](TODO).

Ok so we know how to implement `Inc` for 8 bit target registers. We know how to look in the instruction guide for how `Inc` for 16 bit registers is implemented and how `Dec` is implemented for both 8 and 16 bit registers. Now we can move on to other instructions!
