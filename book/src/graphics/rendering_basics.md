# Rendering Basics

Previously we saw that writting to memory from 0x8000 through 0x97FFF changes the contents of a tile map which describes how 8 by 8 pixel tiles are supposed to be displayed to the screen. We also know that writting to memory from 0x9800 through 0x9FFF changes which tiles are placed at certain spots on the screen, but we haven't yet saw any code that makes this work.

In this chapter we'll emulate how the Game Boy renders tiles to the screen. We'll see that the Game Boy actually renders on a line by line basis. At the end of the chapter, we'll have a big array buffer that holds actual RGBA values that can be given to a rendering library to be rendered in a window.

But before we can discuss this we first need to expand our knowledge of how long certain things that the Game Boy does take to do. We'll need to look into CPU cycles.

## CPU Cycles

When the Game Boy's CPU runs an instruction it takes a certain amount of time to complete that instruction. Of course, we can ask ourselves how many nanoseconds it takes for the Game Boy to do an `ADD` or `JP` instruction, but there's a better way to think about time: CPU cycles.

A CPU cycle is the only unit of time the CPU understands. If the CPU is a clock than the cycle is the tick. Each instruction takes a distinct number of cycles and everytime the CPU executes that instruction it takes the same number of cycles to complete. The Game Boy has a distinct "clock rate" or "clock speed" that specifies how many cycles the CPU can do in a second. Every computer has a set clock rate. If you have a computer with a 3.1 GHz CPU than your computer can execute 3,100,000,000 cycles a second! The Game Boy is not quite this fast. Instead it has a clock speed of 4.19 MHz (4,190,000 cycles a second). Doing an `ADD` instruction on 8 bit registers takes 4 cycles. This means the Game Boy can do 1,047,500 of these instructions every single second. Wow!

All of the other things that the Game Boy does from displaying images on the screen, to running timers, to polling for user input are all done based on the speed set by this clock speed. This means we're going to need to expose this clock speed to the rest of the system.

First, let's expand our `execute` function to return not only the next program counter value but to also return the number of cycles the instruction took to execute.

```rust,noplayground
# struct Registers { }
# struct CPU { pc: u16, registers: Registers }
# enum Instruction { ADD(ArithmeticTarget), }
# enum ArithmeticTarget { C }
impl CPU {
  fn execute(&mut self, instruction: Instruction) -> (u16, u8) {
    match instruction {
      Instruction::ADD(target) => {
        match target {
          ArithmeticTarget::C => {
            // TODO: Do the actual work here
            (self.pc.wrapping_add(1), 4)
          }
          _ => { panic!("TODO: support more targets") }
        }
      }
      _ => { panic!("TODO: support more instructions") }
    }
  }
}
```

We can use the [instruction guide](../appendix/instruction_guide/index.html) to figure out how many CPU cycles a given instruction can take.

Now that we know how many cycles an instruction takes we can use that information to advance the rest of the Game Boy forward in time. We'll create a method on our Bus called `step` that moves the subsystems of the Game Boy forward by a certain number of cycles. At first, this step function will just move our GPU forward in time, but as we add more subsystems like audio, this method will do more:

```rust,noplayground
# struct GPU {}
# struct MemoryBus { gpu: GPU }

impl MemoryBus {
  fn step(&mut self, cycles: u8) {
    self.gpu.step(cycles);
  }
}
```

We now can move our Memory Bus and all the related subsystems like graphics forward in time by a certain number of cycles. Before we can look into the step method on the GPU, we first need to talk at a very high level how the Game Boy hardware draws pixels to the screen.

## Drawing Pixels to the Screen

The way that the Game Boy's video screen works is conceptually very similar to how cathode ray televisions worked. In such televisions there was a vaccum tube with electron gun that pointed towards a phosphorescent screen and sweeped across this screen from left to right and top to bottom esentially painting the image on the screen. The Game Boy's screen follows the same conceptual pattern where it's screen is a large grid of pixels, and behind this grid is a "pixel gun" that points to every pixel in the grid one by one (left to right and top to bottom) and tells it what color it should make (one of the four colors we've described before).

Just like cathode ray televisions, the "pixel gun" doesn't start at the top left most pixel on the screen, but rather it starts to the left of the screen. Before every line that the "pixel gun" will paint to the screen, there is a period of time when it is off screen. This time period is known as the horizontal blank (HBlank) time period. Once the "pixel gun" has painted all the pixels on the screen, it does not immeadiately go back to the top left and start again. There is a time period when the contents on the screen will remain static. This is known as the vertical blank (VBlank) time period.

So how long in CPU cycles does all of this take to do?


## Graphics Life Cycle

The graphics subsystem of the Game Boy (often referred to as the LCD Controller) is, at the very end of the day a simple state machine that has four different states. As the Game Boy progresses cycle by cycle, the LCD controller sometimes switch state (usually called mode). The 4 different modes each tell us about what the graphics subsystem is doing and what the game code itself is allowed to do. Let's take a look:
* Vertical Blank: the Game Boy has w
* Horizontal Blank
* OAM Access
* VRAM Access

