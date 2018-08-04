# Number Notations

In this guide we'll deal with the various ways that we can think about and write down numbers.

## Number Notations

### Binary

The smallest amount of data that computers, including the Game Boy, deal with are bits. A bit is either one of two values. When we talk about bits we say that a bit is one of two distinct values: "1" or "0". Bits in other words are "binary" (i.e. relating to, composed of, or involving two things - "bi" is a latin prefix meaning two).

So we can talk about bits by using either the symbol "1" or the symbol "0". This way of talking about numbers is called "binary notation". This is different from the way we normally talk about numbers where we have ten different possibilities composed of ten distinct symbols: "0", "1", "2", "3", "4", "5", "6", "7", "8", or "9" - a.k.a. decimal notation ("deci" is a latin prefix meaning ten). When talking binary we'll never need any other symbols besides "0" and "1".

> **Side Note**
>
> We don't *have* to use the symbols "1" and "0" for binary. People sometimes use other ways of representing the two distinct values including: "yes" or "no", true or false, on or off, high or low, and more! In fact, computers normally represent bits as either one voltage (e.g. five volts) or as some other, typically lower, voltage (e.g. zero volts). Again, a great resource for learning about how computers actually deal with bits, check out Ben Eater's series on [making an 8-bit Breadboard Computer](https://www.youtube.com/user/eaterbc).

So in binary notation we can represent the number zero with the symbol "0" and the number one with the symbol "1". What if we want to represent the number two? We just said we don't have the symbol "2" in binary - we only have either "1" or "0". Well let's think about what we do in our "normal" decimal system when we run out of symbols to use? In other words what happens when we're going from nine (represented by "9") to ten? Well, we reset the first digit to "0" and add a new one starting with "1"  and we end up with "10" for ten.

This is exactly what we do in binary. So, to represent the number two, we reset the first digit to "0" and a "1" to the left of it. The number two is therefore "10".

So what's the number three then? You guessed it! "11". Let's count to ten in binary!

Zero   "0"
One    "1"
Two    "10"
Three  "11"
Four   "100"
Five   "101"
Six    "110"
Seven  "111"
Eight  "1000"
Nine   "1001"
Ten    "1010"

We can now count as high as we want in binary!

Since binary numbers can look an awful lot like decimal numbers, it's helpful to have a way to say "this number is written in binary not in decimal!". A common way this is done (and how both we will do it in this book and how Rust also does it) is by prefixing numbers written in binary with "0b". So, the number three would be written "0b11"

> **Side Note**
>
> In order to avoid confusion, the best way to read binary numbers outloud (or in your head) is by reading each digit instead of using the decimal name for the number. So the number "0b1001" should either be read as "nine" or as "one zero zero one" never as "one thousand and one".

### Hexadecimal

One interesting and important thing to note is at what numbers we add an extra digit when using binary. Let's list them: two, four, eight, sixteen, etc. Can you spot the pattern? Each number is a power two! Another way to call a counting system that works like this is a "base two" counting system. Our decimal system on the other hand gains a digit at every number that is a power of ten - so it is a "base ten" counting system.

The natural question might arise: "are there any other types of counting systems besides base two and base ten". And the answer is yes! While there are many different systems, the only other one we'll make use of in our emulator is hexadecimal - base sixteen!

So, base two has two different symbols and base ten has ten different symbols, which means base sixteen should have sixteen different symbols. And here they are: "1", "2", "3", "4", "5", "6", "7", "8", "9", "a", "b", "c", "d", "e", "f". So, instead of writing "10" for ten, we would now write "a" and eleven would be "b" and so on. Once we run out of symbols (i.e. after fifteen - "f"), we do what we've always done: reset our number to "0" and add a digit. Sixteen is written "10".

Just like how in binary, we use "0b" to make it obvious we're talking binary and not decimal, we use "0x"for hexdecimal. So "0x18" is twenty-four not eighteen.
