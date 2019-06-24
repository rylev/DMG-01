#[cfg(feature = "serialize")]
#[macro_use]
extern crate serde_derive;

pub mod cpu;
mod gpu;
mod interrupt_flags;
mod joypad;
mod memory_bus;
mod timer;
mod utils;

pub use cpu::CPU;
pub use joypad::Joypad;