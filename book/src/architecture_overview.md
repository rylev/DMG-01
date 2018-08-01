# Architecture Overview

Before we can begin we need to have a basic understanding of what is needed to build a Game Boy emulator. This starts by having a basic understanding of the Game Boy computer architecture.

## At a High Level

The Gameboy can be thought to contain the following pieces:

#### CPU (Central Processing Unit)

The CPU is the "brains" of a computer. The CPU is responsible for the following:
* Executing the game's instructions
* Reading and writting to memory
* Reacting to "events" (known as interrupts) that come from the I/O (input/output) hardware such as the screen and the gamepad controls
* Keeping track of very small pieces of data in "registers" that it manipulates when it runs instructions

The Game Boy's CPU is a custom chip similar to the [Intel 8080](https://en.wikipedia.org/wiki/Intel_8080), itself similar to the [Zilog Z80](https://en.wikipedia.org/wiki/Zilog_Z80).

#### RAM (Random Access Memory)

This is the piece of hardware that allows the Game Boy to remember data while it is running. Without the RAM, the Gameboy's CPU could still execute instructions and keep track of small pieces of data in its registers, but if the data no longer fits in its registers, the CPU would have to throw it away.

### ROM (Read Only Memory)

This is memory that has been "hardcoded" into the machine (hence it being read only). This memory is used to tell the machine how to set itself up (a.k.a bootstrap) to be able to begin accepting instructions as well as to play the [iconic splash screen](https://www.youtube.com/watch?v=ClJWTR_lCL4) on boot.

Game cartridges are also known as ROMs as they are mostly read only memory as well. The truth is a bit fuzzier than this, but we won't be working with cartridges until much later in the book.

### I/O (Input/Output)

The Gameboy has several pieces of I/O hardware:
* The screen,
* Hardware dedicated to playing sounds,
* A gamepad that the player uses to interact with the game through pushing various buttons.

We'll discuss how the CPU can interface with this hardware much later in the book.


Now that we have a small overview of the pieces we'll be talking about, let's take a closer look at the CPU!
