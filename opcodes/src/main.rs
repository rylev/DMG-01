#![feature(proc_macro_non_items)]
#![feature(use_extern_macros)]

extern crate maud;
use maud::{html, Markup};

fn main() {
    let markup = html! {
        head {
            meta charset="utf-8" {}
            title { "DMG-01: Instruction Guide" }
            link rel="stylesheet" href="index.css" {}
        }
        body {
            div {
                div class="header" {
                    div class="title"{
                        "Instruction Guide"
                    }
                    (instruction_table())
                    (instruction_list())
                }
            }
        }
    };
    println!("{}", markup.into_string());
}

struct Instruction {
    name: &'static str,
    code: u8,
}
const INSTRUCTIONS: [Instruction; 6] = [
    Instruction {
        name: "NOP",
        code: 0x00,
        pc: 1,
        cycles: 4,
    },
    Instruction {
        name: "LD BC,d16",
        code: 0x01,
        pc: 3,
        cycles: 12,
    },
    // Instruction { name: "LD (BC), A", code: 0x02 },
    Instruction {
        name: "STOP 0",
        code: 0x10,
        pc: 2,
        cycles: 12,
    },
    Instruction {
        name: "LD DE,d16",
        code: 0x11,
    },
    // Instruction { name: "LD (DE),A", code: 0x12 },
    Instruction {
        name: "JR NZ,r8",
        code: 0x20,
    },
    Instruction {
        name: "LD HL,d16",
        code: 0x21,
    },
    // Instruction { name: "LD (HL+),A", code: 0x22 },
];
const INSTRUCTION_TABLE_LABELS: [&'static str; 2] = [
    "0x0",
    "0x1", // "0x2", "0x3", "0x4", "0x5", "0x6", "0x7",
           //"0x8", "0x9", "0xA", "0xB", "0xC", "0xD", "0xE", "0xF",
];
fn instruction_table() -> Markup {
    let instruction_table = form_instruction_table();
    html! {
        div {
            div class="table-header" style="display: flex;" {
                @for label in &INSTRUCTION_TABLE_LABELS {
                    div {
                        (label)
                    }
                }
            }
            @for (row_index, row) in instruction_table.iter().enumerate() {
                div class=(format!("row{}", row_index)) style="display:flex;"{
                    div {
                        (INSTRUCTION_TABLE_LABELS[row_index])
                    }
                    @for (column_index, instruction) in row.iter().enumerate() {
                        div class=(format!("column{}", column_index)) style="width: 100px;height: 100px; text-align: center;"{
                            (instruction.name)
                        }
                    }
                }
            }
        }
    }
}

fn form_instruction_table<'a>() -> Vec<Vec<&'a Instruction>> {
    let mut rows = Vec::with_capacity(16);
    for row_index in 0..(INSTRUCTION_TABLE_LABELS.len() as u8) {
        let mut row = Vec::with_capacity(16);
        for column_index in 0..(INSTRUCTION_TABLE_LABELS.len() as u8) {
            let code = column_index << 4 | row_index;
            let instruction = INSTRUCTIONS
                .iter()
                .find(|i| i.code == code)
                .expect(&format!("Instruction for code: {}", code));
            row.push(instruction);
        }
        rows.push(row);
    }

    rows
}

fn instruction_list() -> Markup {
    html! {
        div id="instruction-list" {
            @for instruction in INSTRUCTIONS.iter() {
                div class="instruction" {
                    div { (instruction.name) }
                    (instruction_details(instruction))
                }
            }
        }
    }
}

fn instruction_details(instruction: &Instruction) -> Markup {
    html! {
        div class="instruction-info" {
            div class="quick-info" {
                div class="pc-cycles" {
                    div { (instruction.pc) }
                    // div { (instruction.cycles) }
                }
                div class="flags" {
                    // div { (instruction.flags.zero) }
                    // div { (instruction.flags.negative) }
                    // div { (instruction.flags.half_carry) }
                    // div { (instruction.flags.carry) }
                }
            }
            div class="description" {
                // (instruction.description)
            }
            div class="playground" { }
        }
    }
}
