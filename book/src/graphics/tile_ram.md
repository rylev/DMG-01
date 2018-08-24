# Tile Ram

Before we can display background graphics to the screen, we have to have a good understanding of how background graphics actually work and where those graphics are stored in memory.

Game Boy games do not have direct control over what appears in the background. This is because the Game Boy is limited in how much it can store for graphics. The Game Boy has 0x1FFF (8191) bytes worth of storage for background graphics. Unlike more modern systems that employ a direct "frame buffer" (i.e., a long array of bytes where each byte or set of bytes describes how a corresponding pixel should be displayed on screen), the Game Boy uses a tiling system. This system allows the game to build 8 pixel by 8 pixel tiles and then place each tile on the screen at a certain index.

TODO: Graphic showing the difference between the two

## Creating Tiles

So before we look at how pixels on the screen are shown, we first have to see how games manipulate and store tiles.

As we've seen before in our overview of the memory map of the Game Boy, tile data is stored between 0x8000 and 0x97FF (0x1800 or 6144 bytes worth of memory). This area actually contains two seperate tile sets. This allows the game to very quickly switch between two differnt graphic styles without having to switch the tiles out in the time between two screens. We'll explore how the game switches between the two tile sets a bit later.

For now, we'll be focusing on the first tile set in memory that resides at 0x8000 to 0x8FFF (for a total of 0x1000 or 4096 bytes worth of data). Each tile is encoded in 16 bytes (we'll talk about exactly what this encoding looks like below). So if we 0x1000 bytes worth of memory and each tile is encoded in 16 bytes, then we have 0x1000 / 0x10 or 0x100 (256) different tiles.

An observant reader might wonder why the first tile set takes up 0x1000 of the 0x1800 or two thirds worth of space alloted for tile memory. The truth is that the second tile set starts at 0x8800 and goes to 0x97FF. The chunk between 0x8800 and 0x8FFF is therefore shared by the two tile sets.

TODO: Make nicer chart
```ignore
8000-87FF: First part of tile set #1
8800-8FFF: Second part of tile set #1
           First part of tile set #2
9000-97FF: Second part of tile set #2
```

So how are each of the tiles encoded? First, we need to understand how many different colors a pixel of the Game Boy can display. The Game Boy is capable of displaying 4 different colors: white, light gray, dark gray, and black. The minimal number of bits that we need to encode this information is 2 bits since two bits can encode 4 different numbers: 0b00, 0b01, 0b10, and 0b11.

>> *Learn More*
>>
>> The way the Game Boy hardware displays the 4 different colors is simply by emitting 4 different levels of white light. For "white" for instance the light is fully on, while for black the light is fully off. Light and dark gray are at 33% light and 66% light respectively. In fact, calling these colors white, gray and black isn't really true since the screen of the original Game Boy was green so the colors players see are actually shades of green.

The bit value to color mapping is as follows:

```ignore
+------+------------+
| 0b11 | white      |
| 0b10 | dark-gray  |
| 0b01 | light-gray |
| 0b00 | black      |
+------+------------+
```

So each pixel of our 8x8 pixel (i.e., 64 pixels in total) tile will take 2 bits to represent. That means we'll need 64 * 2 or 128 bits total to represent all the pixels. In terms of number of bytes that's 128 / 8 or 16 bytes total as we've said above.

So this shouldn't be too hard to encode right? Just start from the top left most pixel and every two bits we encode that pixels value right? Unfortunately not. The actual encoding scheme is a little bit more complicated.

Each row of a tile is 2 bytes worth of data (8 pixels with 2 bits per pixel equals 16 bits or 2 bytes). Instead of each pixels value coming one after the other, each pixel is split between the two bytes. So the first pixel is encoded with the left most (i.e., most signficant bit a.k.a bit 7) of each byte.

For example, let's imagine that the first two bytes of our tile set memory were 0xB5 (0b10110101) and 0x65 (0b01100101). These two bytes together will encode the data for the first tile. Byte 1 contains the value of the upper (a.k.a most significant) bit and byte 2 contains the value of the lower (least significant) bit.

Let's take a look at how this looks. In the following diagram that colors are represented by 1 letter "B" for black, "D" for dark-gray, "L" for light-gray and "W" for white.

```ignore
              Bit Position
A            7 6 5 4 3 2 1 0
d          +-----------------+
d  0x8000  | 1 0 1 1 0 1 0 1 |
r          |-----------------|
e  0x8001  | 0 1 1 0 0 1 0 1 |
s          +-----------------+
s            D L W D B W B W
                 Color
```

Since reading the tile data happens much more often than writing it, we can store the tile data internally in a more friendly way.

Let's write some code to see what we need. First, we're going to create a new struct that will be responsible for all the graphics needs of the Game Boy. This loosely mimics the set up of actual hardware where the CPU knows nothing about graphics and all. There's no one chip responsible for graphics instead there is dedicated video RAM and the screen hardware. It would over complicate things if we tried to too closely mimic this set up. Instead we'll create the `GPU` or "Graphic Processing Unit" to model all of our video needs.

For now, our GPU will hold on to video RAM and our tile set data. Our video ram is just a long array which holds on to raw byte values. The tile set will also be an array of tiles. A tile is simply an array of 8 rows where a row is an array of 8 `TileValue`s.

```rust, noplayground
const VRAM_BEGIN: usize = 0x8000;
const VRAM_END: usize = 0x9FFF;
const VRAM_SIZE: usize = VRAM_END - VRAM_BEGIN + 1;

#[derive(Copy,Clone)]
enum TilePixelValue {
    Zero,
    One,
    Two,
    Three,
}

type Tile = [[TilePixelValue; 8]; 8];
fn empty_tile() -> Tile {
    [[TilePixelValue::Zero; 8]; 8]
}

struct GPU{
    vram: [u8; VRAM_SIZE],
    tile_set: [Tile; 384],
}
```

Let's go back to our memory bus to redirect any of writes in memory to our video ram to go to the GPU:

```rust, noplayground
# const VRAM_BEGIN: usize = 0x8000;
# const VRAM_END: usize = 0x9FFF;
# struct GPU { }
# impl GPU { fn read_vram(&self,addr: usize) -> u8 { 0 }
             fn write_vram(&self, addr: usize, value: u8) {  } }
# struct MemoryBus { gpu: GPU }

impl MemoryBus {
    fn read_byte(&self, address: u16) -> u8 {
        let address = address as usize;
        match address {
            VRAM_BEGIN ... VRAM_END => {
                self.gpu.read_vram(address - VRAM_BEGIN)
            }
            _ => panic!("TODO: support other areas of memory")
        }
    }

    fn write_byte(&self, address: u16, value: u8) {
        let address = address as usize;
        match address {
            VRAM_BEGIN ... VRAM_END => {
                self.gpu.write_vram(address - VRAM_BEGIN, value)
            }
            _ => panic!("TODO: support other areas of memory")
        }
    }
}
```

Notice how from the `MemoryBus` we don't directly access the vram but instead go through two methods `read_vram` and `write_vram`. This is so we can easily cache our tile set in the `tile_set` field of our CPU. Let's take a look at how these are implemented.

`read_vram` is very simple as it actually just reads from the vram array:
```rust, noplayground
# struct GPU { vram: Vec<u8> }
impl GPU {
  fn read_vram(&self, address: usize) -> u8 {
    self.vram[address]
  }
}
```

However, `write_vram` is much more involved. Let's take a look at the code and go line by line to see what's happening:

```rust,noplayground
# #[derive(Copy,Clone)]
# enum TilePixelValue { Three, Two, One, Zero }
# struct GPU { vram: Vec<u8>, tile_set: [[[TilePixelValue; 8]; 8]; 384]  }
impl GPU {
    fn write_vram(&mut self, index: usize, value: u8) {
        self.vram[index] = value;
        // If our index is greater than 0x1800, we're not writing to the tile set storage
        // so we can just return.
        if index >= 0x1800 { return }

        // Tiles rows are encoded in two bytes with the first byte always
        // on an even address. Bitwise ANDing the address with 0xffe
        // gives us the address of the first byte.
        // For example: `12 & 0xFFFE == 12` and `13 & 0xFFFE == 12`
        let normalized_index = index & 0xFFFE;

        // First we need to get the two bytes that encode the tile row.
        let byte1 = self.vram[normalized_index];
        let byte2 = self.vram[normalized_index + 1];

        // A tiles is 8 rows tall. Since each row is encoded with two bytes a tile
        // is therefore 16 bytes in total.
        let tile_index = index / 16;
        // Every two bytes is a new row
        let row_index = (index % 16) / 2;

        // Now we're going to loop 8 times to get the 8 pixels that make up a given row.
        for pixel_index in 0..8 {
            // To determine a pixel's value we must first find the corresponding bit that encodes
            // that pixels value:
            // 1111_1111
            // 0123 4567
            //
            // As you can see the bit that corresponds to the nth pixel is the bit in the nth
            // position *from the left*. Bits are normally indexed from the right.
            //
            // To find the first pixel (a.k.a pixel 0) we find the left most bit (a.k.a bit 7). For
            // the second pixel (a.k.a pixel 1) we first the second most left bit (a.k.a bit 6) and
            // so on.
            //
            // We then create a mask with a 1 at that position and 0s everywhere else.
            //
            // Bitwise ANDing this mask with our bytes will leave that particular bit with its
            // original value and every other bit with a 0.
            let mask = 1 << (7 - pixel_index);
            let lsb = byte1 & mask;
            let msb = byte2 & mask;

            // If the masked values are not 0 the masked bit must be 1. If they are 0, the masked
            // bit must be 0.
            //
            // Finally we can tell which of the four tile values the pixel is. For example, if the least
            // significant byte's bit is 1 and the most significant byte's bit is also 1, then we
            // have tile value `Three`.
            let value = match (lsb != 0, msb != 0) {
                (true, true) => TilePixelValue::Three,
                (false, true) => TilePixelValue::Two,
                (true, false) => TilePixelValue::One,
                (false, false) => TilePixelValue::Zero,
            };

            self.tile_set[tile_index][row_index][pixel_index] = value;
        }

    }
}
```
