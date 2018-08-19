# Finishing Up the CPU

We've nearly reached the end of building our CPU. There are a few instructions left to implement, many of which we won't touch on here, since they're closely tied with other parts of the Game Boy that we've yet to talk about.

## Remaining Instructions

In this chapter, we'll look at two more instructions: `NOP` and `HALT`

### NOP

`NOP` is perhaps the simplest of the Game Boy's instructions. It stands for no-operation and it effectively does nothing except advance the program counter by 1.

### HALT

`HALT` is a big more complicated than `NOP`. When the Game Boy is running, it is constantly in a loop executing instructions. The `HALT` instruction gives the game the ability to stop the CPU from executing any more instructions. How the Game Boy eventually continues executing instructions will be discussed later in the book, but for now, we have the ability to stop the Game Boy dead in its tracks. Reasons a game might want to do this include saving battery. If the game doesn't have anything to do, it can halt the CPU and save a bit energy.

For now, we'll implement `HALT` by adding a `is_halted` boolean to the CPU. At the beginning of `execute` we can check if the CPU is halted. If it is, we simply return. The `HALT` instruction simply sets this field to true.

## Where We Are

So far we've learned about the CPU and the many different instructions that the CPU can execute. We learned that these instructions live in memory which is just a long array of 8-bit numbers. The CPU reads in these bytes, decodes them as instructions and executes them. Some instructions simply do different arithmetic operations on the contents of the CPU's registers. Some instructions can cause the CPU to change its program counters, effectively "jumping" it to a different place in the game code. Some instructions read from and write to memory including a special part of memory we call the stack which behaves like a stack data structure. Finally, we learned about two special instructions: `NOP` which does nothing and `HALT` which stops the CPU from executing more instructions.

In the next section of the book, we'll be leaving the comfort of the CPU and exploring memory more closely.
