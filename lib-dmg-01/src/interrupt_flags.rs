#[cfg_attr(feature = "serialize", derive(Serialize))]
pub struct InterruptFlags {
    pub vblank: bool,
    pub lcdstat: bool,
    pub timer: bool,
    pub serial: bool,
    pub joypad: bool,
}

impl InterruptFlags {
    pub fn new() -> InterruptFlags {
        InterruptFlags {
            vblank: false,
            lcdstat: false,
            timer: false,
            serial: false,
            joypad: false,
        }
    }

    pub fn from_byte(&mut self, byte: u8) {
        self.vblank = (byte & 0b1) == 0b1;
        self.lcdstat = (byte & 0b10) == 0b10;
        self.timer = (byte & 0b100) == 0b100;
        self.serial = (byte & 0b1000) == 0b1000;
        self.joypad = (byte & 0b10000) == 0b10000;
    }

    pub fn to_byte(&self) -> u8 {
        0b11100000 | // unused bits always read as 1s
               ((if self.joypad { 1 } else { 0 }) << 4) |
               ((if self.serial { 1 } else { 0 }) << 3) |
               ((if self.timer { 1 } else { 0 }) << 2) |
               ((if self.lcdstat { 1 } else { 0 }) << 1) |
               (if self.vblank { 1 } else { 0 })

    }
}