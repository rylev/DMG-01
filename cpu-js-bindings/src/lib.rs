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
        let mut registers = self.0.registers;
        match register {
            Register::A => registers.a = value as u8,
            Register::B => registers.b = value as u8,
            Register::C => registers.c = value as u8,
            Register::D => registers.d = value as u8,
            Register::E => registers.e = value as u8,
            Register::F => registers.f = (value as u8).into(),
            Register::H => registers.h = value as u8,
            Register::L => registers.l = value as u8,
            Register::AF => registers.set_af(value),
            Register::BC => registers.set_bc(value),
            Register::DE => registers.set_de(value),
            Register::HL => registers.set_hl(value)
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
        Target::B => Some(dmg_01::cpu::instruction::ArithmeticTarget::B),
        Target::C => Some(dmg_01::cpu::instruction::ArithmeticTarget::C),
        Target::D => Some(dmg_01::cpu::instruction::ArithmeticTarget::D),
        Target::E => Some(dmg_01::cpu::instruction::ArithmeticTarget::E),
        Target::H => Some(dmg_01::cpu::instruction::ArithmeticTarget::H),
        Target::L => Some(dmg_01::cpu::instruction::ArithmeticTarget::L),
        // TODO: think about returning error for invalid targets
        _ => None
    };

    if let Some(target) = target {
        cpu.execute(dmg_01::cpu::instruction::Instruction::ADD(target));
    }

    CPU(cpu)
}
