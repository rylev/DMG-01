# CPU

The Game Boy's CPU is a custom chip called the Sharp LR35902. The chip is very similar to the much more popular [Intel 8080](https://en.wikipedia.org/wiki/Intel_8080) and the [Zilog Z80](https://en.wikipedia.org/wiki/Zilog_Z80). The 8080 was used by lots of different computers in the 70s and 80s including the very first comercially successful personal computer the [Altair 8800](https://en.wikipedia.org/wiki/Altair_8800). The Z80 was also a very popular chip being used in many home electronic devices including in many Sega home consoles such as the Master System and the Sega Genesis/Mega Drive.

We won't go into the specifics of what makes the LR35902 different from the Intel 8080 or Z80, but in general it's good to know that a large portion of what we'll learn about this custom chip will be applicable to other popular chips from yesteryear.

In the next few sections we'll be looking at the different CPU instructions that the LR35902 can execute as long with how it reads instructions from memory, decodes them and updates its internal state as well as the contents of memory and different I/O devices.
