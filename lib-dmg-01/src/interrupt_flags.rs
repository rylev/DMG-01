pub struct InterruptFlags {
    vblank: bool,
    lcdstat: bool,
    timer: bool,
    serial: bool,
    joypad: bool,
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
}