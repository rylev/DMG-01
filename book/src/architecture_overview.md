# Architecture Overview

Before we can begin we need to have a basic understanding of what is needed to build a Game Boy emulator. This starts by having a basic understanding of the Game Boy computer architecture. If you're already familiar with the very basics of CPUs, computer memory and I/O devices, you can skip to the next page.

## At a High Level

The Gameboy can be thought to contain the following pieces:

#### CPU (Central Processing Unit)

The CPU is the "brains" of a computer. The CPU is responsible for the following:
* Executing instructions defined by the game the Game Boy is running
* Reading and writting to memory
* Reacting to "events" (known as interrupts) that come from the I/O (input/output) hardware such as the screen and the gamepad controls
* Keeping track of very small pieces of data in "registers" that it manipulates when it runs instructions

#### RAM (Random Access Memory)

This is the piece of hardware that allows the Game Boy to remember data while it is running. Without the RAM, the Gameboy's CPU could still execute instructions and keep track of small pieces of data in its registers, but if the data no longer fits in its registers, the CPU would have to throw it away.

### ROM (Read Only Memory)

This is memory that has been "hardcoded" into the machine (hence it being read only). This memory is used to tell the machine how to set itself up (a.k.a bootstrap) to be able to begin accepting instructions as well as to play the [iconic splash screen](https://www.youtube.com/watch?v=ClJWTR_lCL4) on boot.

Game cartridges are also known as ROMs as they are mostly read only memory as well.

### I/O (Input/Output)

The Gameboy has several pieces of I/O hardware:
* The screen,
* Hardware dedicated to playing sounds,
* A gamepad that the player uses to interact with the game through pushing various buttons.

We'll discuss how the CPU can interface with this hardware much later in the book.

> **Learn More**
>
> If you're interested in learning more about how computers work all the way down to the electrical level, I suggest watching Ben Eater's series on [making an 8-bit Breadboard Computer](https://www.youtube.com/user/eaterbc). Ben does an absolutely wonderful job of explaining how CPUs, RAM, ROM and even I/O devices are built from the ground up!


Now that we have a small overview of the pieces we'll be talking about, let's take a closer look at the CPU!
