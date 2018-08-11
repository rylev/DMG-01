# Bit and Byte Manipulation

In this guide, we'll be discussing bit and byte manipulation through different bit-wise operators. If you're a little rusty on bits, bytes, binary and hexadecimal, check out our [guide on number notation](./number_notations.md) and our [guide on numbers](./numbers.md)

CPUs normally do very simple operations on their registers. Most of these operations should be familiar to everyone, but some involve manipulating the bits of a byte in some structured way. Let's take a look.

## AND (&)

Bitwise AND (&) is similiar to the boolean AND (&&) operator you're probably already familiar with except that it operates on each bit of a byte or set of bytes. It's important to remember that boolean values are equivalent to a single bit: 1 is true and 0 is false.

If we have two boolean values we already know how to AND them:

```ignore
true  && true  == true
false && true  == false
true  && false == false
false && false == false
```

For bitwise operators we simply do the same operation on the the bits of two numbers that are the same digit place. If we encounter a `1` and `1` it becomes `1`. All other combinations are `0`.

Let's take a look at an example:

```ignore
  1001
& 1100
------
  1000
```

Of course, we can do the same thing with hexadecimal numbers. The best way to think of these when you're getting used to them is to convert the hexadecimal number to binary and then do the AND:

```ignore
  0x8  1000
& 0x3  0011
------ ----
  0x0  0000
```

Bitwise AND is often used to get the value at a specific bit or byte number. For example, if you have a 4 byte value and you just want the value in the third byte position you can AND the number with another value that has zeros in every position but the thrid byte - this practice is known as "bit masking".

## OR (|)

Just like bitwise AND, Bitwise OR (|) is similiar to the boolean OR (||) operator except that it operates on each bit of a byte or set of bytes.

If we have two boolean values we already know how to || them:

```ignore
true  || true  == true
false || true  == true
true  || false == true
false || false == false
```

Let's take a look at a bitwise example:

```ignore
  1001
| 1100
------
  1101
```

The same advice around hexadecimal applies for bitwise OR as it did for AND. When in doubt, convert to binary and go bit by bit.

Bitwise OR can be used to combine two binary values together. For example, if you want to be sure that a value has it's least signficant bit set to 1, you can OR it with 0b1.

## Shift (<< >>)

Bit shifting is the practice of moving bits in order to different digit positions.

To accomplish this first we look at the operator to see what direction we're going: `<<` for left and `>>` for right. We then take each bit and shift it over as many places as the number to the right of the operator tells us to. Any numbers that no longer fit (i.e. they've been shift off the edge of the number) disappear. All digit places that are left empty by the shifting get replaced by 0s.

Let's take a look at some examples:

```ignore
1001 >> 1 == 0100
1001 >> 2 == 0010
1001 >> 3 == 0001
1001 >> 4 == 0000

1101 << 1 == 1010
1101 << 2 == 0100
1101 << 3 == 1000
1101 << 4 == 0000
```

Again, if you have a number in hexadecimal notation, you can first convert it to binary and go bit by bit.

One thing bit shifting is used for is to get the values of certain bytes in a number composed of many bytes. For example, in the following snippet, we're trying to get the value of the most significant byte:

```ignore
0x1c74a3 >> 16 == 0x1c
```
