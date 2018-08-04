# Numbers in Computers

In this guide, we'll look at how numbers are stored in the Gameboy's CPU, RAM, and ROM. In this guide we'll be using different types of number notations: binary, decimal and hexadecimal. If you're unfamiliar with these different ways of writing numbers, check out our [guide on number notations](./number_notations.md).

## Bits

At a very basic level, computers can only read and write two different values that we'll call 1 and 0. This piece of data is called a bit. Computer memory is really just a long array of bits that the computer can read or write.

> **Learn More**
>
> Computers normally represent bits as either one voltage (e.g., five volts) or as some other, typically lower, voltage (e.g., zero volts). Again, a great resource for learning about how computers actually deal with bits, check out Ben Eater's series on [making an 8-bit Breadboard Computer](https://www.youtube.com/user/eaterbc).

Bits can represent only two different values 0 or 1. If we want to numbers larger than one we need to compose bits together. To get to three for instance we would write 0b11. The total count of numbers we can represent is equal to 2^(# of bits). So one bit can represent 2^1 a.k.a two numbers and 7 bits can represent 2^7 a.k.a 128 numbers.

Since being able to manipulate numbers larger than 1 is pretty useful, we normally talk about and the computer typically reads and writes bits in large chunks called bytes.

### Bytes

Bytes are defined as a collection of 8 bits. Our Gameboy, as an 8-bit machine, typically deals with one byte at a time and each compartment in memory stores one byte. However, the Game Boy also has 16-bit instructions which act on two bytes at a time. A byte can represent numbers 2^8 a.k.a 256 numbers (0 to 255) while 8 bytes (composed of 64 bits) and can represent 2^64 a.k.a 9,223,372,036,854,775,808 numbers (0 to 9,223,372,036,854,775,807).

> **Learn More**
>
> Some times we'll actually only deal with half a byte (i.e., 4 bits) at a time. This is usually referred to as a "nibble".

Since writing out bytes in binary can be quite tedious, we normally write out bytes in hexadecimal notation: So while we could write out the byte representing the number 134 as "0b10000110" we typically write it as "0x86". These two notations specify the same number, "0x86" is just shorter so it's more often used.

When disucssing numbers composed of multiple bytes, for example 0xFFA1 (composed of three bytes), we'll often need to talk about which byte is "most significant" (MSB - most significant byte) and which is "least significant" (LSB - least significant byte). Going back to math class, you may remember that when writing numbers like "178", the digit on the right (i.e., the "8") is the least sigificant, it adds the least amount to the total sum of the number (just eight) while the digit on the left (i.e., the "1") is the most significant since it adds the most to the sum of the number (one hundred!). Bytes work the same way - in 0xFFA1, 0xFF is the most significant byte and 0xA1 is the least significant.

## Endianess

Let's take the example of two bytes sitting next to each other in memory: first at address 0 there is 0xFF and then at address 1 there is 0x16. If we want to read these two bytes together as a 16 bit number, should it be read as 0xFF16 or as 0x16FF? Even if one way or the other makes more sense to you, the answer is: it depends on the machine. In the case of the Game Boy the order is 0xFF16 - in other words the least significant byte is first in memory. This scheme is known as little-endian and its opposite is known as big-endian.

### Signed Numbers

Ok so we know how to conceivably represent any number from 0 to some very large positive number. We can just keep adding bytes until we have enough to represent our number. But what about negative numbers? Well one way we could chose to do it (and the way the Game Boy does it) is using something called the "two's complement".

Let's say we have the number 0b00111011 a.k.a. 59 and we want to represent -59 instead. In two's complement, we do the following:
* Invert every digit - 1s become 0s and 0s become 1s
  * 0b00111011 becomes 0b11000100
* Add 1 to the number
  * 0b11000100 becomes 0b11000101

So -59 is 0b11000101. But wait is 0b11000101 already 197? Yes it is! Whether we chose to interpret a byte as a number from 0 to 255 or as two's complement number capable of representing -128 to 127 is up to programmer! Interpreting a number as only positive means it is "unsigned" and interpeting as being possibly negative with two's complement means it is "signed".

### Rust

In Rust, the various number types tell us both how many bits are used to represent that particular integer and whether the integer is in two's complement or not. For example, the number type `u8` is a number composed of 8 bits (i.e., 1 byte) and is unsigned while `i64` is a number composed of 64 bits (i.e., 8 bytes) and is signed.
