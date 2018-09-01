#![feature(use_extern_macros)]

#[macro_use]
extern crate serde_derive;

extern crate wasm_bindgen;
extern crate lib_dmg_01;

use wasm_bindgen::prelude::*;

use lib_dmg_01::cpu::instruction;

#[wasm_bindgen]
#[derive(Serialize)]
pub struct CPU(lib_dmg_01::cpu::CPU);

#[wasm_bindgen]
impl CPU {
    #[wasm_bindgen(constructor)]
    pub fn new(boot_rom: Option<Vec<u8>>, game_rom: Vec<u8>) -> CPU {
        let cpu = lib_dmg_01::cpu::CPU::new(boot_rom, game_rom);
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
            Register::HL => self.0.registers.set_hl(value)
        }
    }

    pub fn step(&mut self) -> u8 {
        self.0.step()
    }

    pub fn to_json(&self) -> JsValue {
        JsValue::from_serde(&self).unwrap()
    }
}


#[wasm_bindgen]
pub enum Target {
    A, B, C, D, E, F, H, L, AF, BC, DE, HL
}

#[wasm_bindgen]
pub enum Register {
    A, B, C, D, E, F, H, L, AF, BC, DE, HL
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
        _ => None
    };

    if let Some(target) = target {
        cpu.execute(instruction::Instruction::ADD(target));
    }

    CPU(cpu)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_set_register() {
        let mut cpu = CPU::new();
        cpu.set_register(Register::A, 1);
        assert_eq!(cpu.0.registers.a, 1);
    }
}
