#[cfg(feature = "serialize")]
#[macro_use]
extern crate serde_derive;

pub mod cpu;
mod gpu;
mod interrupt_flags;
mod memory_bus;
mod timer;