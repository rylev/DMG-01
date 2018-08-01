# Architecture Overview

Before we can begin we need to have a basic understanding of what is needed to
build a Game Boy emulator. This starts by having a basic understanding of the
Game Boy computer architecture.

## At a High Level

The Gameboy can be thought to contain the following pieces:

#### CPU (Central Processing Unit)

The CPU is the "brains" of a computer. We'll be going into detail of what a CPU
does in the next chapter, but for now know that it takes care of executing the
game's instructions, talks with memory and interfaces with the I/O
(input/output) hardware such as the screen and the gamepad controls, and keeps
track of very small pieces of data in "registers" that it acts on when executing instructions.

The Game Boy's CPU is a custom chip similar to the [Intel 8080](https://en.wikipedia.org/wiki/Intel_8080),
itself similar to the [Zilog Z80](https://en.wikipedia.org/wiki/Zilog_Z80).

#### RAM (Random Access Memory)

This is the piece of hardware that allows the Game Boy to "remember" things
while it is running. Without the RAM, the Gameboy's CPU could still execute
instructions and keep track of small pieces of data in its registers, but if
the data it needs to remember doesn't fit in its registers, it would have to
throw away that data.

### ROM (Read Only Memory)

This is memory that has been "hardcoded" into the machine (hence it being read
only). This memory is used to tell the machine how to set itself up to be able
to begin accepting instructions as well as to play the [iconic splash screen](https://www.youtube.com/watch?v=ClJWTR_lCL4) on
boot.

### I/O (Input/Output)

The Gameboy contains a screen, hardware dedicated to playing
sounds, and a gamepad that the player uses to interact with the game through
pushing various buttons. We'll discuss later how the CPU can interface with this hardware much
later in the book.
