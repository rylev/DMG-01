pub mod flags_register;
pub mod registers;

use self::registers::Registers;

pub struct CPU {
    registers: Registers
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            registers: Registers::new()
        }
    }
}
