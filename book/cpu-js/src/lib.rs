#![feature(use_extern_macros)]

#[macro_use]
extern crate serde_derive;

extern crate wasm_bindgen;
extern crate dmg_01;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize)]
pub struct CPU(dmg_01::cpu::CPU);

#[wasm_bindgen]
impl CPU {
    #[wasm_bindgen(constructor)]
    pub fn new() -> CPU {
        let cpu = dmg_01::cpu::CPU::new();
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
        Target::A => Some(dmg_01::cpu::instruction::ArithmeticTarget::A),
        _ => None
    };

    if let Some(target) = target {
        cpu.execute(dmg_01::cpu::instruction::Instruction::ADD(target));
    }

    CPU(cpu)
}
