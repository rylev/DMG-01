# Introduction

If you really want to know how a particular computer works, there's no better way to learn than by emulating that computer. In this book, we'll be looking at one of the most loved computers of all time, Nintendo's Game Boy. We'll be going through the process from nothing and building the Game Boy up piece by piece. In the process not only will we learn more about the Game Boy itself, but we'll also get a good glimpse into how computers work in general as well as how to build emulators for other computer systems as well.

## What You Need to Know

We'll be assuming only basic programming knowledge. If you've programmed in any language before you should be good to go. The particular language we'll be using is the [Rust programming language](https://www.rust-lang.org), but if you've never used Rust, don't worry; we'll be taking things slow, and I'll do my best to link to relevant learning material as things come up. If you want to get a head start on learning the basics of Rust, the amazing [Rust book](https://doc.rust-lang.org/book/second-edition/index.html) is a great way to learn.

If you've already build emulators before, and are just looking for a reference specific to the Game Boy, you might find this book to be too detailed. I recommend the [Pan Docs](http://bgb.bircd.org/pandocs.htm) as a great place to quickly learn the nity-grity of the Game Boy. You'll find even more resources in the [resources guide](./appendix/resources.md).

## Why Rust?

For many emulation projects performance is a key consideration. This means that emulators are often written in low-level languages that allow the programmer to easily write performant code. While our Game Boy emulator could be written in other languages like JavaScript or Python, it's best to use a language that would be appropriate for more resource intensive emulation (e.g. Sony's PlayStation 2 or Nintendo's Wii) so these skills can be used for future emulation projects. Rust fits the bill perfectly here, and has the added bonus of being much more beginner friendly than C or C++.

Rust also has a great cross platform story - we'll be focusing on web and on desktop, but in the future, we might also be able to bring our emualtor to mobile platforms and embedded devices!

## Setup

In order to get started, you'll only need your favorite text editor and Rust related tooling. Follow the instructions [on the Rust website](https://www.rust-lang.org/en-US/install.html) for how to install the rustup tool which gives you access to the Rust compiler and the Rust build tool and package manager called Cargo, as well as some other tools that we'll be using later on in our journey.

If you've successfully been able to install Rust you can create a new project by running the following command:

```bash
cargo new emulator
```

Navigate into your project's directory and have a look around. To run your project run the following:

```bash
cargo run
```

You're all good to go! Let's get emulating!
