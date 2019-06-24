#[macro_use]
extern crate serde_derive;

extern crate console_error_panic_hook;
extern crate lib_dmg_01;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

use lib_dmg_01::cpu::instruction;

#[wasm_bindgen]
#[derive(Serialize)]
pub struct CPU(lib_dmg_01::CPU);

#[wasm_bindgen]
impl CPU {
    #[wasm_bindgen(constructor)]
    pub fn new(boot_rom: Option<Vec<u8>>, game_rom: Vec<u8>) -> CPU {
        console_error_panic_hook::set_once();
        let cpu = lib_dmg_01::CPU::new(boot_rom, game_rom);
        CPU(cpu)
    }

    pub fn set_register(&mut self, register: Register, value: u16) {
        match register {
            Register::A => self.0.registers.a = value as u8,
            Register::B => self.0.registers.b = value as u8,
            Register::C => self.0.registers.c = value as u8,
            Register::D => self.0.registers.d = value as u8,
            Register::E => self.0.registers.e = value as u8,
            Register::F => self.0.registers.f = (value as u8).into(),
            Register::H => self.0.registers.h = value as u8,
            Register::L => self.0.registers.l = value as u8,
            Register::AF => self.0.registers.set_af(value),
            Register::BC => self.0.registers.set_bc(value),
            Register::DE => self.0.registers.set_de(value),
            Register::HL => self.0.registers.set_hl(value),
        }
    }

    pub fn step(&mut self) -> u8 {
        self.0.step()
    }

    pub fn set_joypad(&mut self, joypad: Joypad) {
        self.0.bus.joypad = joypad.0;
    }

    pub fn canvas_buffer(&self, buffer: &mut [u8]) {
        buffer.copy_from_slice(&self.0.bus.gpu.canvas_buffer);
    }

    pub fn to_json(&self) -> JsValue {
        JsValue::from_serde(&self).unwrap()
    }

    pub fn memory_slice(&self, start: u16, end: u16) -> Vec<u8> {
        self.0.bus.slice(start, end)
    }

    pub fn get_tile_set_buffer(&self, outline_tiles: bool) -> Vec<u8> {
        self.0.bus.gpu.tile_set_as_buffer(outline_tiles)
    }

    pub fn get_background_buffer(&self, outline_tiles: bool, show_viewport: bool) -> Vec<u8> {
        self.0
            .bus
            .gpu
            .background_as_buffer(outline_tiles, show_viewport)
    }

    pub fn get_tile_at(&self, x: usize, y: usize) -> Vec<u8> {
        let tile = self.0.bus.gpu.get_tile_buffer_at(x, y);
        let mut data = Vec::with_capacity(8 * 8);
        for row in tile.iter() {
            for pixel in row.iter() {
                data.push(*pixel as u8);
                data.push(*pixel as u8);
                data.push(*pixel as u8);
                data.push(255);
            }
        }
        data
    }
}

#[wasm_bindgen]
pub enum Target {
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L,
    AF,
    BC,
    DE,
    HL,
}

#[wasm_bindgen]
pub enum Register {
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L,
    AF,
    BC,
    DE,
    HL,
}

#[wasm_bindgen]
pub fn add(cpu: CPU, target: Target) -> CPU {
    let mut cpu = cpu.0;
    let target = match target {
        Target::A => Some(instruction::ArithmeticTarget::A),
        Target::B => Some(instruction::ArithmeticTarget::B),
        Target::C => Some(instruction::ArithmeticTarget::C),
        Target::D => Some(instruction::ArithmeticTarget::D),
        Target::E => Some(instruction::ArithmeticTarget::E),
        Target::H => Some(instruction::ArithmeticTarget::H),
        Target::L => Some(instruction::ArithmeticTarget::L),
        // TODO: think about returning error for invalid targets
        _ => None,
    };

    if let Some(target) = target {
        cpu.execute(instruction::Instruction::ADD(target));
    }

    CPU(cpu)
}

#[wasm_bindgen]
pub enum Button {
    A,
    B,
    Start,
    Select,
    Up,
    Down,
    Right,
    Left,
}
#[wasm_bindgen]
#[derive(Serialize)]
pub struct Joypad(lib_dmg_01::Joypad);

#[wasm_bindgen]
impl Joypad {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Joypad {
        Joypad(lib_dmg_01::Joypad::new())
    }

    pub fn set_button(&mut self, button: Button, to: bool) {
        match button {
            Button::A => self.0.a = to,
            Button::B => self.0.b = to,
            Button::Start => self.0.start = to,
            Button::Select => self.0.select = to,
            Button::Up => self.0.up = to,
            Button::Down => self.0.down = to,
            Button::Right => self.0.right = to,
            Button::Left => self.0.left = to,
        }
    }

}

#[cfg(test)]
mod tests {
    // use super::*;
    // #[test]
    // fn can_set_register() {
    //     let mut cpu = CPU::new();
    //     cpu.set_register(Register::A, 1);
    //     assert_eq!(cpu.0.registers.a, 1);
    // }
}
