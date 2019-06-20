#[cfg(feature = "serialize")]
#[macro_use]
extern crate serde_derive;

pub mod cpu;
mod gpu;
mod memory_bus;
mod interrupt_flags;
