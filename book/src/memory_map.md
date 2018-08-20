# Memory Map

Up until now we've been treating the Game Boy's memory as one long array with 0xFFFF entries in it. While this was a helpful simplification for our work on the CPU (which doesn't know anything more about memory than that), in reality things are bit more complex. Sections of memory are actually used for very specific purposes. In fact there are parts of "memory" that are not actually backed by a RAM chip but instead are directly tied to things like the Game Boy screen or the Game Boy's audio device. Below we'll be "mapping" out memory and talking at a high level about what the different sections of the Game Boy's memory map do. We'll start at the very beginning of memory with address 0x0000 and work our way all the up to the top. This chapter should serve as the jumping off point for the rest of the book, so if you don't understand exactly how something works don't worry, we'll be going in much more detail later on.

## 0x0000 - 0x00FF: Boot ROM

When the Game Boy first boot's the very bottom 256 bytes of memory is occuppied with the boot ROM. We've talked a little bit about the boot ROM before - how it's responsible for bootstrapping the Game Boy to be able to run a game as well as for playing the [iconic splash screen](https://www.youtube.com/watch?v=ClJWTR_lCL4) on boot. Later in the book, we'll be examining the boot ROM very closely.

## 0x0000 - 0x3FFF: Game ROM Bank 0

Once the Game Boy is done booting, it unmaps the boot ROM from memory so it is no longer available. From this point the area once occupied by the boot ROM all the way up to address 0x3FFF is occupied by game code loaded in from the cartridge. Inside of this memory are two areas worth noting:
* 0x0000 - 0x00FF - the area once occupied by the Boot ROM now holds memory called the "Interrupt Table". We'll be talking at length in the future about interrupt's, but for now the best way to think about them is just like "events" in higher level programming. When specific things happen the hardware automatically looks inside the interrupt table at specific locations for how to handle those events.
* 0x0100 - 0x014F - This area is known as the cartridge header area. It contains data about the cartridge that was loaded into Game Boy including its name, the size of cartridge ROM and even the nintendo logo. We'll talk about more about the contents of the cartridge when we talk about the boot ROM since it directly references this header area. If you want to really dive into specifics checkout [the cartridge header guide](./appendix/cartridge_header.md)

After these special areas is plain game code. The reason this area is refered to as Bank 0 is explained in the next section.

## 0x4000 - 0x7FFF: Game ROM Bank N

Game ROMs can be quite large - much larger than what can fit into the memory area available. To handle this, the Game Boy allows for ROM "bank switching". A ROM bank is simply a chunk of the cartirdge ROM. The game can switch in these chunks at run time into the area between 0x4000 - 0x7FFF. The first bank, bank 0, is always the same memory and cannot be switched out. Only the area of memory between 0x4000 and 0x7FFF is capable of being switched out. Later in the book we'll go into specifics of how this works.

## 0x8000 - 0x97FF: Tile RAM

This area of memory contains data about the graphics that can be displayed to the screen. The Game Boy uses a tiling system for grapics meaning that a game doesn't control the specific pixels that get drawn to the screen, at least not directly. Instead, the game creates a set of tiles which are square chunks of pixels. It can then place these tiles on the screen. So instead of saying "draw pixel 438 light green", it first says "create a tile with these pixels values" and then later "place that tile I made earlier in positon 5". The placement of tiles at certain positions happens in the next chunk of memory...

## 0x9800 - 0x9FFF: Background Map

As we described above, the Game Boy uses a tiling system for graphics. In memory 0x8000 to 0x97FF the game creates different tiles. These tiles however don't show up on screen. That happens in this section of memory where the game can map tiles to sections of the screen.

## 0xA000 - 0xBFFF: Cartridge RAM

Cartridges (being physical devices) sometimes had extra RAM on them. This gave games even more memory to work with. If the cartridge had this extra RAM the Game Boy automatically mapped the RAM into this area of memory.

## 0xC000 - 0xDFFF: Working RAM

This is the RAM that the Game Boy allows a game to use. Our idea of RAM really just being a plain old array where the game could read and write bytes to really only applies to this section of memory.

## 0xE000 - 0xFDFF: Echo RAM

This section of memory directly mirrors the working RAM section - meaning if you write into the first address of working RAM (0xC000), the same value will appear in the first spot of echo RAM (0xE000). Nintendo actively discouraged developers from using this area of memory and as such we can just pretend it doesn't exist.

## 0xFE00 - 0xFE9F: OAM (Object Atribute Memory)

This area of memory contains the description of graphical sprites. The tiles we talked about above were used for backgrounds and levels but not for characters, enemies or objects the user interacted with. These entities, known as sprites, have extra capabilties. The description for how they should look lives here.

## 0xFEA0 - 0xFEFF: Unused

This area is completely unmapped: reading from it just returns 0s and writing to it does nothing.

## 0xFF00 - 0xFF7F: I/O Registers

This is one of the most dense areas of memory. Practically every byte has a special meaning. It's used by both the screen and the sound system to determine different settings. We'll be talking a lot about this in the future.

## 0xFF80 - 0xFFFE: High RAM Area

This area is also just normal RAM but is used a lot because some of the `LD` instructions we've already seen can easily target this area in memory. This area is also sometimes used for the stack, but the top of working RAM is also used for this purpose.

## 0xFFFF: Interrupt Enabled Register

The very last byte of memory has special meaning used for handling interrupt events. We'll be looking at this closely later in the book.


Now that we have an idea of what all the areas of memory are used for we can start diving more in detail. The first area we'll be looking at is at tile RAM and the background map along with a few of the I/O registers that will help us finally get some graphics on a screen!
