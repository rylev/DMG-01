pub mod flags_register;
pub mod registers;
pub mod instruction;

use std;

use self::registers::Registers;
use self::instruction::{
    Instruction,
    IncDecTarget,
    ArithmeticTarget,
    PrefixTarget,
    BitPosition,
    JumpTest,
    ADDHLTarget,
    LoadType,
    LoadByteSource,
    LoadByteTarget,
    LoadWordTarget,
    Indirect,
    StackTarget
};

/// # Macros
///
/// The following are macros for generating repetitive code needed for processing CPU
/// instructions. For more information on macros read [the chapter in the Rust book](https://doc.rust-lang.org/book/second-edition/appendix-04-macros.html).

// Macro for changing the CPU based on the value of a 8 bit register
macro_rules! manipulate_8bit_register {
    // Macro pattern for getting a value from a register and doing some work on that value
    //
    // # Example Usage:
    // ``` rust
    // manipulate_8bit_register!(self, a => print_register)
    // ```
    //
    // This above reads register `a` and then calls the method `print_register` with the
    // value from `a`
    ( $self:ident : $getter:ident => $work:ident) => {
        {
            let value = $self.registers.$getter;
            $self.$work(value)
        }
    };
    // Macro pattern for getting a value from a register and doing some work on that value and
    // writting it back to the register
    //
    // # Example Usage:
    // ``` rust
    // manipulate_8bit_register!(self, a => increment => d)
    // ```
    //
    // This above reads register `a` and then calls the method `increment` with the
    // value from `a` and then writes the result of `increment` into register `d`
    ( $self:ident : $getter:ident => $work:ident => $setter:ident) => {
        {
            let result = manipulate_8bit_register!($self: $getter => $work);
            $self.registers.$setter = result;
        }
    };
    // Macro pattern for getting a value from a register and doing some work on that value at a
    // specific bit pattern
    //
    // # Example Usage:
    // ``` rust
    // manipulate_8bit_register!(self, a => increment @ BitPosition::B2)
    // ```
    //
    // This above reads register `a` and then calls the method `increment` with the
    // value from `a` and the bit position marker `B2`
    ( $self:ident : ( $register:ident @ $bit_position:ident ) => $work:ident ) => {
        {
            let value = $self.registers.$register;
            $self.$work(value, $bit_position)
        }
    };
    // Macro pattern for getting a value from a register and doing some work on that value at a
    // specific bit pattern and writting it back to the register
    //
    // # Example Usage:
    // ``` rust
    // manipulate_8bit_register!(self, a => increment @ BitPosition::B2 => c)
    // ```
    //
    // This above reads register `a` and then calls the method `increment` with the
    // value from `a` and the bit position marker `B2` and then writes the result of the
    // call to `increment` into the register `c`.
    ( $self:ident : ( $getter:ident @ $bit_position:ident ) => $work:ident => $setter:ident) => {
        {
            let result = manipulate_8bit_register!($self: ( $getter @ $bit_position ) => $work);
            $self.registers.$setter = result;
        }
    };
}

// Macro for changing the value of a 16 bit register through some CPU method
// Arguments:
// * self (a.k.a the CPU)
// * a method for getting a register,
// * a method for changing register's value,
// * a method for setting a register,
//
// The macro gets the value from the register, performs work on that value and then sets the value back in the
// register
macro_rules! manipulate_16bit_register {
    ( $self:ident : $getter:ident => $work:ident => $setter:ident ) => {
        {
            let amount = $self.registers.$getter();
            let result = $self.$work(amount);
            $self.registers.$setter(result);
        }
    };
}

macro_rules! arithmetic_instruction {
    // Macro pattern for matching a register and then manipulating the register
    //
    // # Example Usage:
    // ``` rust
    // arithmetic_instruction!(register, self.foo)
    // ```
    //
    // The above matches a register and then calls the function `foo` to do work on the value
    // in that register.
    ( $register:ident, $self:ident.$work:ident) => {
        {
            match $register {
                ArithmeticTarget::A => manipulate_8bit_register!($self: a => $work),
                ArithmeticTarget::B => manipulate_8bit_register!($self: b => $work),
                ArithmeticTarget::C => manipulate_8bit_register!($self: c => $work),
                ArithmeticTarget::D => manipulate_8bit_register!($self: d => $work),
                ArithmeticTarget::E => manipulate_8bit_register!($self: e => $work),
                ArithmeticTarget::H => manipulate_8bit_register!($self: h => $work),
                ArithmeticTarget::L => manipulate_8bit_register!($self: l => $work),
                ArithmeticTarget::D8 => {
                    let value = $self.read_next_byte();
                    $self.$work(value);
                }
                ArithmeticTarget::HLI => {
                    let value = $self.bus.read_byte($self.registers.get_hl());
                    $self.$work(value);
                }
            };
            match $register {
                ArithmeticTarget::D8  => ($self.pc.wrapping_add(2), 8),
                ArithmeticTarget::HLI => ($self.pc.wrapping_add(1), 8),
                _                     => ($self.pc.wrapping_add(1), 4)
            }
        }
    };
    // Macro pattern for matching a register and then manipulating the register and writing the
    // value back to the a register
    //
    // # Example Usage:
    // ``` rust
    // arithmetic_instruction!(register, self.foo => a)
    // ```
    //
    // The above matches a register and then calls the function `foo` to do work on the value
    // in that register and writes the result of `foo` into the a register.
    ( $register:ident, $self:ident.$work:ident => a) => {
        {
            match $register {
                ArithmeticTarget::A => manipulate_8bit_register!($self: a => $work => a),
                ArithmeticTarget::B => manipulate_8bit_register!($self: b => $work => a),
                ArithmeticTarget::C => manipulate_8bit_register!($self: c => $work => a),
                ArithmeticTarget::D => manipulate_8bit_register!($self: d => $work => a),
                ArithmeticTarget::E => manipulate_8bit_register!($self: e => $work => a),
                ArithmeticTarget::H => manipulate_8bit_register!($self: h => $work => a),
                ArithmeticTarget::L => manipulate_8bit_register!($self: l => $work => a),
                ArithmeticTarget::D8 => {
                    let value = $self.read_next_byte();
                    let result = $self.$work(value);
                    $self.registers.a = result;
                }
                ArithmeticTarget::HLI => {
                    let value = $self.bus.read_byte($self.registers.get_hl());
                    let result = $self.$work(value);
                    $self.registers.set_hl(result as u16);
                }
            };
            match $register {
                ArithmeticTarget::D8  => ($self.pc.wrapping_add(2), 8),
                ArithmeticTarget::HLI => ($self.pc.wrapping_add(1), 8),
                _                     => ($self.pc.wrapping_add(1), 4)
            }
        }
    };
}

macro_rules! prefix_instruction {
    // Macro pattern for matching a register and then manipulating the register and writing the
    // value back to the a register
    //
    // # Example Usage:
    // ``` rust
    // prefix_instruction!(register, self.foo => a)
    // ```
    //
    // The above matches a register and then calls the function `foo` to do work on the value
    // in that register and writes the result of `foo` into the `a` register.
    ( $register:ident, $self:ident.$work:ident => reg) => {
        {
            match $register {
                PrefixTarget::A => manipulate_8bit_register!($self: a => $work => a),
                PrefixTarget::B => manipulate_8bit_register!($self: b => $work => b),
                PrefixTarget::C => manipulate_8bit_register!($self: c => $work => c),
                PrefixTarget::D => manipulate_8bit_register!($self: d => $work => d),
                PrefixTarget::E => manipulate_8bit_register!($self: e => $work => e),
                PrefixTarget::H => manipulate_8bit_register!($self: h => $work => h),
                PrefixTarget::L => manipulate_8bit_register!($self: l => $work => l),
                PrefixTarget::HLI => {
                    let value = $self.bus.read_byte($self.registers.get_hl());
                    let result = $self.$work(value);
                    $self.registers.set_hl(result as u16);
                }
            }
            let cycles = match $register {
                PrefixTarget::HLI => 16,
                _                 => 8
            };
            ($self.pc.wrapping_add(2), cycles)
        }
    };
    // Macro pattern for matching a register and then manipulating the register at a specific bit
    // location and writing the value back to the a register
    //
    // # Example Usage:
    // ``` rust
    // prefix_instruction!(register, (self.foo @ bit_position) => a)
    // ```
    //
    // The above matches a register and then calls the function `foo` to do work on the value
    // in that register at the bit position `bit_position` and writes the result of `foo` into the `a` register.
    ( $register:ident, ( $self:ident.$work:ident @ $bit_position:ident ) => reg) => {
        {
            match $register {
                PrefixTarget::A => manipulate_8bit_register!($self: (a @ $bit_position) => $work => a),
                PrefixTarget::B => manipulate_8bit_register!($self: (b @ $bit_position) => $work => b),
                PrefixTarget::C => manipulate_8bit_register!($self: (c @ $bit_position) => $work => c),
                PrefixTarget::D => manipulate_8bit_register!($self: (d @ $bit_position) => $work => d),
                PrefixTarget::E => manipulate_8bit_register!($self: (e @ $bit_position) => $work => e),
                PrefixTarget::H => manipulate_8bit_register!($self: (h @ $bit_position) => $work => h),
                PrefixTarget::L => manipulate_8bit_register!($self: (l @ $bit_position) => $work => l),
                PrefixTarget::HLI => {
                    let value = $self.bus.read_byte($self.registers.get_hl());
                    let result = $self.$work(value, $bit_position);
                    $self.registers.set_hl(result as u16);
                }
            }
            let cycles = match $register {
                PrefixTarget::HLI => 16,
                _                 => 8
            };
            ($self.pc.wrapping_add(2), cycles)
        }
    };
    // Macro pattern for matching a register and then manipulating the register at a specific bit
    // location
    //
    // # Example Usage:
    // ``` rust
    // prefix_instruction!(register, (self.foo @ bit_position))
    // ```
    //
    // The above matches a register and then calls the function `foo` to do work on the value
    // in that register at the bit position `bit_position`
    ( $register:ident, $self:ident.$work:ident @ $bit_position:ident ) => {
        {
            match $register {
                PrefixTarget::A => manipulate_8bit_register!($self: (a @ $bit_position) => $work),
                PrefixTarget::B => manipulate_8bit_register!($self: (b @ $bit_position) => $work),
                PrefixTarget::C => manipulate_8bit_register!($self: (c @ $bit_position) => $work),
                PrefixTarget::D => manipulate_8bit_register!($self: (d @ $bit_position) => $work),
                PrefixTarget::E => manipulate_8bit_register!($self: (e @ $bit_position) => $work),
                PrefixTarget::H => manipulate_8bit_register!($self: (h @ $bit_position) => $work),
                PrefixTarget::L => manipulate_8bit_register!($self: (l @ $bit_position) => $work),
                PrefixTarget::HLI => {
                    let value = $self.bus.read_byte($self.registers.get_hl());
                    $self.$work(value, $bit_position);
                }
            }
            let cycles = match $register {
                PrefixTarget::HLI => 16,
                _                 => 8
            };
            ($self.pc.wrapping_add(2), cycles)
        }
    };
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Color {
    White,
    LightGray,
    DarkGray,
    Black
}
impl std::convert::From<u8> for Color {
     fn from(n: u8) -> Self {
         match n {
             0 => Color::White,
             1 => Color::LightGray,
             2 => Color::DarkGray,
             3 => Color::Black,
             _ => panic!("Cannot covert {} to color", n)
         }
     }
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct BackgroundColors(
    Color,
    Color,
    Color,
    Color
);

impl BackgroundColors {
    fn new() -> BackgroundColors {
        BackgroundColors(Color::White, Color::LightGray, Color::DarkGray, Color::Black)
    }
}

impl std::convert::From<u8> for BackgroundColors {
     fn from(value: u8) -> Self {
        BackgroundColors(
            (value >> 6).into(),
            ((value >> 4) & 0b11).into(),
            ((value >> 2) & 0b11).into(),
            (value & 0b11).into()
        )
    }
}
#[derive(Copy, Clone, Debug, PartialEq)]
enum TileMap {
    X9800,
    X9C00,
}
#[derive(Copy, Clone, Debug, PartialEq)]
enum BackgroundAndWindowDataSelect {
    X8000,
    X8800,
}
#[derive(Copy, Clone, Debug, PartialEq)]
enum ObjectSize {
    OS8X8,
    OS8X16,
}

struct GPU {
    vram: [u8; VRAM_SIZE],
    background_colors: BackgroundColors,
    scroll_y: u8,
    lcd_display_enabled: bool,
    window_display_enabled: bool,
    background_display_enabled: bool,
    object_display_enabled: bool,
    window_tile_map: TileMap,
    background_tile_map: TileMap,
    background_and_window_data_select: BackgroundAndWindowDataSelect,
    object_size: ObjectSize,
    line: u8
}

impl GPU {
    fn new() -> GPU {
        GPU {
            vram: [0; VRAM_SIZE],
            background_colors: BackgroundColors::new(),
            scroll_y: 0,
            lcd_display_enabled: false,
            window_display_enabled: false,
            background_display_enabled: false,
            object_display_enabled: false,
            window_tile_map: TileMap::X9800,
            background_tile_map: TileMap::X9800,
            background_and_window_data_select: BackgroundAndWindowDataSelect::X8800,
            object_size: ObjectSize::OS8X8,
            line: 0
        }
    }

    fn step(&mut self, cycles: u8) {
    }
}

const BOOT_ROM_BEGIN: usize = 0x00;
const BOOT_ROM_END: usize = 0xFF;
const BOOT_ROM_SIZE: usize = BOOT_ROM_END - BOOT_ROM_BEGIN + 1;

const ROM_BANK_0_BEGIN: usize = 0x0000;
const ROM_BANK_0_END: usize = 0x3FFF;
const ROM_BANK_0_SIZE: usize = ROM_BANK_0_END - ROM_BANK_0_BEGIN + 1;

const VRAM_BEGIN: usize = 0x8000;
const VRAM_END: usize = 0x9FFF;
const VRAM_SIZE: usize = VRAM_END - VRAM_BEGIN + 1;

const EXTERNAL_RAM_BEGIN: usize = 0xA000;
const EXTERNAL_RAM_END: usize = 0xBFFF;
const EXTERNAL_RAM_SIZE: usize = EXTERNAL_RAM_END - EXTERNAL_RAM_BEGIN + 1;

const WORKING_RAM_BEGIN: usize = 0xC000;
const WORKING_RAM_END: usize = 0xDFFF;
const WORKING_RAM_SIZE: usize = WORKING_RAM_END - WORKING_RAM_BEGIN + 1;

const OAM_BEGIN: usize = 0xFE00;
const OAM_END: usize = 0xFE9F;
const OAM_SIZE: usize = OAM_END - OAM_BEGIN + 1;

const IO_REGISTERS_BEGIN: usize = 0xFF00;
const IO_REGISTERS_END: usize = 0xFF7F;

const ZERO_PAGE_BEGIN: usize = 0xFF80;
const ZERO_PAGE_END: usize = 0xFFFE;
const ZERO_PAGE_SIZE: usize = ZERO_PAGE_END - ZERO_PAGE_BEGIN + 1;

struct MemoryBus {
    boot_rom: Option<[u8; BOOT_ROM_SIZE]>,
    rom_bank_0: [u8; ROM_BANK_0_SIZE],
    external_ram: [u8; EXTERNAL_RAM_SIZE],
    working_ram: [u8; WORKING_RAM_SIZE],
    oam: [u8; OAM_SIZE],
    zero_page: [u8; ZERO_PAGE_SIZE],
    gpu: GPU
}

impl MemoryBus {
    pub fn new(boot_rom_buffer: Option<Vec<u8>>) -> MemoryBus {
        let boot_rom = boot_rom_buffer.map(|boot_rom_buffer| {
            if boot_rom_buffer.len() != BOOT_ROM_SIZE {
                panic!("Supplied boot ROM is the wrong size. Is {} bytes but should be {} bytes", boot_rom_buffer.len(), BOOT_ROM_SIZE);
            }
            let mut boot_rom = [0; BOOT_ROM_SIZE];
            boot_rom.copy_from_slice(&boot_rom_buffer);
            boot_rom
        });
        MemoryBus {
            // Note: instead of modeling memory as one array of length 0xFFFF, we'll
            // break memory up into it's logical parts.
            boot_rom: boot_rom,
            rom_bank_0: [0; ROM_BANK_0_SIZE],
            external_ram: [0; EXTERNAL_RAM_SIZE],
            working_ram: [0; WORKING_RAM_SIZE],
            oam: [0; OAM_SIZE],
            zero_page: [0; ZERO_PAGE_SIZE],
            gpu: GPU::new()
        }
    }

    fn step(&mut self, cycles: u8) {
        self.gpu.step(cycles);
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        let address = address as usize;
        match address {
            BOOT_ROM_BEGIN ... BOOT_ROM_END => {
                if let Some(boot_rom) = self.boot_rom {
                    boot_rom[address]
                } else {
                    self.rom_bank_0[address]
                }
            }
            ROM_BANK_0_BEGIN ... ROM_BANK_0_END => {
                self.rom_bank_0[address]
            }
            VRAM_BEGIN ... VRAM_END => {
                self.gpu.vram[address - VRAM_BEGIN]
            }
            EXTERNAL_RAM_BEGIN ... EXTERNAL_RAM_END => {
                self.external_ram[address - EXTERNAL_RAM_BEGIN]
            }
            WORKING_RAM_BEGIN ... WORKING_RAM_END => {
                self.working_ram[address - WORKING_RAM_BEGIN]
            }
            OAM_BEGIN ... OAM_END => {
                self.oam[address - OAM_BEGIN]
            }
            IO_REGISTERS_BEGIN ... IO_REGISTERS_END => {
                self.read_io_register(address)
            }
            ZERO_PAGE_BEGIN ... ZERO_PAGE_END => {
                self.zero_page[address - ZERO_PAGE_BEGIN]
            }
            _ => {
                panic!("Reading from an unkown part of memory at address 0x{:x}", address);
            }
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        let address = address as usize;
        match address {
            ROM_BANK_0_BEGIN ... ROM_BANK_0_END => {
                self.rom_bank_0[address] = value;
            }
            VRAM_BEGIN ... VRAM_END => {
                self.gpu.vram[address - VRAM_BEGIN] = value;
            }
            EXTERNAL_RAM_BEGIN ... EXTERNAL_RAM_END => {
                self.external_ram[address - EXTERNAL_RAM_BEGIN] = value;
            }
            WORKING_RAM_BEGIN ... WORKING_RAM_END => {
                self.working_ram[address - WORKING_RAM_BEGIN] = value;
            }
            OAM_BEGIN ... OAM_END => {
                self.oam[address - OAM_BEGIN] = value;
            }
            IO_REGISTERS_BEGIN ... IO_REGISTERS_END => {
                self.write_io_register(address, value);
            }
            ZERO_PAGE_BEGIN ... ZERO_PAGE_END => {
                self.zero_page[address - ZERO_PAGE_BEGIN] = value;
            }
            _ => {
                panic!("Writing to an unkown part of memory at address 0x{:x}", address);
            }
        }
    }

    fn read_io_register(&self, address: usize) -> u8 {
        match address {
            0xFF40 => {
                // LCD Control
                (if self.gpu.lcd_display_enabled { 1 } else { 0 })               << 7 |
                (if self.gpu.window_tile_map == TileMap::X9C00 { 1 } else { 0 }) << 6 |
                (if self.gpu.window_display_enabled { 1 } else { 0 })            << 5 |
                (if self.gpu.background_and_window_data_select == BackgroundAndWindowDataSelect::X8000 { 1 } else { 0 }) << 4 |
                (if self.gpu.background_tile_map == TileMap::X9C00 { 1 } else { 0 }) << 3 |
                (if self.gpu.object_size == ObjectSize::OS8X16 { 1 } else { 0 }) << 2 |
                (if self.gpu.object_display_enabled { 1 } else { 0 }) << 1 |
                (if self.gpu.background_display_enabled { 1 } else { 0 })
            }
            0xFF44 => {
                self.gpu.line
            }
            _ => panic!("Reading from an unknown I/O register {:x}", address)
        }
    }

    fn write_io_register(&mut self, address: usize, value: u8) {
        match address {
            0xFF11 => { /* Channel 1 Sound Length and Wave */ }
            0xFF25 => { /* Sound output terminal selection */ }
            0xFF26 => { /* Sound on/off */ }
            0xFF40 => {
                // LCD Control
                self.gpu.lcd_display_enabled = (value >> 7) == 1;
                self.gpu.window_tile_map = if ((value >> 6) & 0b1) == 1 {
                    TileMap::X9C00
                } else {
                    TileMap::X9800
                };
                self.gpu.window_display_enabled = ((value >> 5) & 0b1) == 1;
                self.gpu.background_and_window_data_select = if ((value >> 4) & 0b1) == 1 {
                    BackgroundAndWindowDataSelect::X8000
                } else {
                    BackgroundAndWindowDataSelect::X8800
                };
                self.gpu.background_tile_map = if ((value >> 3) & 0b1) == 1 {
                    TileMap::X9C00
                } else {
                    TileMap::X9800
                };
                self.gpu.object_size = if ((value >> 2) & 0b1) == 1 {
                    ObjectSize::OS8X16
                } else {
                    ObjectSize::OS8X8
                };
                self.gpu.object_display_enabled = ((value >> 1) & 0b1) == 1;
                self.gpu.background_display_enabled = (value & 0b1) == 1;
            }
            0xFF42 => {
                // Scroll Y Position
                self.gpu.scroll_y = value;
            }
            0xFF47 => {
                // Background Colors Setting
                self.gpu.background_colors = value.into();
            }
            _ => panic!("Writting '0b{:b}' to an unknown I/O register {:x}", value, address)
        }
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
pub struct CPU {
    pub registers: Registers,
    pc: u16,
    sp: u16,
    #[cfg_attr(feature = "serialize", serde(skip_serializing))]
    bus: MemoryBus,
    is_halted: bool,
}

impl CPU {
    pub fn new(boot_rom: Option<Vec<u8>>) -> CPU {
        CPU {
            registers: Registers::new(),
            pc: 0x0,
            sp: 0x00,
            bus: MemoryBus::new(boot_rom),
            is_halted: false,
        }
    }

    pub fn step(&mut self) {
        if self.is_halted { return }

        let mut instruction_byte = self.bus.read_byte(self.pc);

        let prefixed = instruction_byte == 0xCB;
        if prefixed {
            instruction_byte = self.read_next_byte();
        }

        let (next_pc, cycles) = if let Some(instruction) = Instruction::from_byte(instruction_byte, prefixed) {
            self.execute(instruction)
        } else {
            let description = format!("0x{}{:x}", if prefixed { "cb" } else { "" }, instruction_byte);
            panic!("Unkown instruction found for: {}", description)
        };

        self.bus.step(cycles);

        if !self.is_halted {
            self.pc = next_pc;
        }
    }

    pub fn execute(&mut self, instruction: Instruction) -> (u16, u8) {
        match instruction {
            Instruction::INC(target) => {
                // DESCRIPTION: (increment) - increment the value in a specific register by 1
                // WHEN: target is 16 bit register
                // PC: +1
                // Cycles: 12
                // Z:- S:- H:- C:-
                // WHEN: target is (HL)
                // PC:+1
                // Cycles: 8
                // ELSE:
                // PC: +1
                // Cycles: 4
                // Z:? S:0 H:? C:-
                match target {
                    IncDecTarget::A => manipulate_8bit_register!(self: a => inc_8bit => a),
                    IncDecTarget::B => manipulate_8bit_register!(self: b => inc_8bit => b),
                    IncDecTarget::C => manipulate_8bit_register!(self: c => inc_8bit => c),
                    IncDecTarget::D => manipulate_8bit_register!(self: d => inc_8bit => d),
                    IncDecTarget::E => manipulate_8bit_register!(self: e => inc_8bit => e),
                    IncDecTarget::H => manipulate_8bit_register!(self: h => inc_8bit => h),
                    IncDecTarget::L => manipulate_8bit_register!(self: l => inc_8bit => l),
                    IncDecTarget::HLI => {
                        let hl = self.registers.get_hl();
                        let amount = self.bus.read_byte(hl);
                        let result = self.inc_8bit(amount);
                        self.bus.write_byte(hl, result);
                    }
                    IncDecTarget::BC => manipulate_16bit_register!(self: get_bc => inc_16bit => set_bc),
                    IncDecTarget::DE => manipulate_16bit_register!(self: get_de => inc_16bit => set_de),
                    IncDecTarget::HL => manipulate_16bit_register!(self: get_hl => inc_16bit => set_hl),
                    IncDecTarget::SP => {
                        let amount = self.sp;
                        let result = self.inc_16bit(amount);
                        self.sp = result;
                    }
                };
                let cycles = match target {
                    IncDecTarget::BC | IncDecTarget::DE | IncDecTarget::HL | IncDecTarget::SP => 8,
                    IncDecTarget::HLI => 12,
                    _ => 4
                };
                (self.pc.wrapping_add(1), cycles)
            },
            Instruction::DEC(target) => {
                // DESCRIPTION: (decrement) - decrement the value in a specific register by 1
                // WHEN: target is 16 bit register
                // PC: +1
                // Cycles: 12
                // Z:- S:- H:- C:-
                // WHEN: target is (HL)
                // PC:+1
                // Cycles: 8
                // ELSE:
                // PC: +1
                // Cycles: 4
                // Z:? S:0 H:? C:-
                match target {
                    IncDecTarget::A => manipulate_8bit_register!(self: a => dec_8bit => a),
                    IncDecTarget::B => manipulate_8bit_register!(self: b => dec_8bit => b),
                    IncDecTarget::C => manipulate_8bit_register!(self: c => dec_8bit => c),
                    IncDecTarget::D => manipulate_8bit_register!(self: d => dec_8bit => d),
                    IncDecTarget::E => manipulate_8bit_register!(self: e => dec_8bit => e),
                    IncDecTarget::H => manipulate_8bit_register!(self: h => dec_8bit => h),
                    IncDecTarget::L => manipulate_8bit_register!(self: l => dec_8bit => l),
                    IncDecTarget::HLI => {
                        let hl = self.registers.get_hl();
                        let amount = self.bus.read_byte(hl);
                        let result = self.dec_8bit(amount);
                        self.bus.write_byte(hl, result);
                    }
                    IncDecTarget::BC => manipulate_16bit_register!(self: get_bc => dec_16bit => set_bc),
                    IncDecTarget::DE => manipulate_16bit_register!(self: get_de => dec_16bit => set_de),
                    IncDecTarget::HL => manipulate_16bit_register!(self: get_hl => dec_16bit => set_hl),
                    IncDecTarget::SP => {
                        let amount = self.sp;
                        let result = self.dec_16bit(amount);
                        self.sp = result;
                    }
                };
                let cycles = match target {
                    IncDecTarget::BC | IncDecTarget::DE | IncDecTarget::HL | IncDecTarget::SP => 8,
                    IncDecTarget::HLI => 12,
                    _ => 4
                };
                (self.pc.wrapping_add(1), cycles)
            },
            Instruction::ADD(register) => {
                // DESCRIPTION: (add) - add the value stored in a specific register
                // with the value in the A register
                // WHEN: target is D8
                // PC:+2
                // Cycles: 8
                // WHEN: target is (HL)
                // PC:+1
                // Cycles: 8
                // ELSE:
                // PC: +1
                // Cycles: 4
                // Z:? S:0 H:? C:?
                arithmetic_instruction!(register, self.add_without_carry => a)
            },
            Instruction::ADDHL(register) => {
                // DESCRIPTION: (add) - add the value stored in a specific register
                // with the value in the HL register
                // PC:+1
                // Cycles: 8
                // Z:- S:0 H:? C:?
                let value = match register {
                    ADDHLTarget::BC => self.registers.get_bc(),
                    ADDHLTarget::DE => self.registers.get_de(),
                    ADDHLTarget::HL => self.registers.get_hl(),
                    ADDHLTarget::SP => self.sp,
                };
                let result = self.add_hl(value);
                self.registers.set_hl(result);
                (self.pc.wrapping_add(1), 8)
            },
            Instruction::ADDSP => {
                // DESCRIPTION: (add stack pointer) - add a one byte signed number to
                // the value stored in the stack pointer register
                // PC:+2
                // Cycles: 16
                // Z:0 S:0 H:? C:?

                // First cast the byte as signed with `as i8` then extend it to 16 bits
                // with `as i16` and then stop treating it like a signed integer with
                // `as u16`
                let value = self.read_next_byte() as i8 as i16 as u16;
                let result = self.sp.wrapping_add(value);

                // Half and whole carry are computed at the nibble and byte level instead
                // of the byte and word level like you might expect for 16 bit values
                let half_carry_mask = 0xF;
                self.registers.f.half_carry = (self.sp & half_carry_mask) + (value & half_carry_mask) > half_carry_mask;
                let carry_mask = 0xff;
                self.registers.f.carry = (self.sp & carry_mask) + (value & carry_mask) > carry_mask;

                self.sp = result;

                (self.pc.wrapping_add(2), 16)
            },
            Instruction::ADC(register) => {
                // DESCRIPTION: (add with carry) - add the value stored in a specific
                // register with the value in the A register and the value in the carry flag
                // WHEN: target is D8
                // PC:+2
                // Cycles: 8
                // WHEN: target is (HL)
                // PC:+1
                // Cycles: 8
                // ELSE:
                // PC: +1
                // Cycles: 4
                // Z:? S:0 H:? C:?
                arithmetic_instruction!(register, self.add_with_carry => a)
            },
            Instruction::SUB(register) => {
                // DESCRIPTION: (subtract) - subtract the value stored in a specific register
                // with the value in the A register
                // WHEN: target is D8
                // PC:+2
                // Cycles: 8
                // WHEN: target is (HL)
                // PC:+1
                // Cycles: 8
                // ELSE:
                // PC: +1
                // Cycles: 4
                // Z:? S:1 H:? C:?
                arithmetic_instruction!(register, self.sub_without_carry => a)
            },
            Instruction::SBC(register) => {
                // DESCRIPTION: (subtract) - subtract the value stored in a specific register
                // with the value in the A register and the value in the carry flag
                // WHEN: target is D8
                // PC:+2
                // Cycles: 8
                // WHEN: target is (HL)
                // PC:+1
                // Cycles: 8
                // ELSE:
                // PC: +1
                // Cycles: 4
                // Z:? S:1 H:? C:?
                arithmetic_instruction!(register, self.sub_with_carry => a)
            },
            Instruction::AND(register) => {
                // DESCRIPTION: (AND) - do a bitwise and on the value in a specific
                // register and the value in the A register
                // WHEN: target is D8
                // PC:+2
                // Cycles: 8
                // WHEN: target is (HL)
                // PC:+1
                // Cycles: 8
                // ELSE:
                // PC: +1
                // Cycles: 4
                // Z:? S:0 H:1 C:0
                arithmetic_instruction!(register, self.and => a)
            },
            Instruction::OR(register) => {
                // DESCRIPTION: (OR) - do a bitwise or on the value in a specific
                // register and the value in the A register
                // WHEN: target is D8
                // PC:+2
                // Cycles: 8
                // WHEN: target is (HL)
                // PC:+1
                // Cycles: 8
                // ELSE:
                // PC: +1
                // Cycles: 4
                // Z:? S:0 H:0 C:0
                arithmetic_instruction!(register, self.or => a)
            },
            Instruction::XOR(register) => {
                // DESCRIPTION: (XOR) - do a bitwise xor on the value in a specific
                // register and the value in the A register
                // WHEN: target is D8
                // PC:+2
                // Cycles: 8
                // WHEN: target is (HL)
                // PC:+1
                // Cycles: 8
                // ELSE:
                // PC: +1
                // Cycles: 4
                // Z:? S:0 H:0 C:0
                arithmetic_instruction!(register, self.xor => a)
            },
            Instruction::CP(register) => {
                // DESCRIPTION: (compare) - just like SUB except the result of the
                // subtraction is not stored back into A
                // WHEN: target is D8
                // PC:+2
                // Cycles: 8
                // WHEN: target is (HL)
                // PC:+1
                // Cycles: 8
                // ELSE:
                // PC: +1
                // Cycles: 4
                // Z:? S:1 H:? C:?
                arithmetic_instruction!(register, self.compare)
            },
            Instruction::CCF => {
                // DESCRIPTION: (complement carry flag) - toggle the value of the carry flag
                // PC:+1
                // Cycles: 4
                // Z:- S:0 H:0 C:?
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                self.registers.f.carry = !self.registers.f.carry;
                (self.pc.wrapping_add(1), 4)
            }
            Instruction::SCF => {
                // DESCRIPTION: (set carry flag) - set the carry flag to true
                // PC:+1
                // Cycles: 4
                // Z:- S:0 H:0 C:1
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                self.registers.f.carry = true;
                (self.pc.wrapping_add(1), 4)
            }
            Instruction::RRA => {
                // DESCRIPTION: (rotate right A register) - bit rotate A register right through the carry flag
                // PC:+1
                // Cycles: 4
                // Z:0 S:0 H:0 C:?
                manipulate_8bit_register!(self: a => rotate_right_through_carry_retain_zero => a);
                (self.pc.wrapping_add(1), 4)
            }
            Instruction::RLA => {
                // DESCRIPTION: (rotate left A register) - bit rotate A register left through the carry flag
                // PC:+1
                // Cycles: 4
                // Z:0 S:0 H:0 C:?
                manipulate_8bit_register!(self: a => rotate_left_through_carry_retain_zero => a);
                (self.pc.wrapping_add(1), 4)
            }
            Instruction::RRCA => {
                // DESCRIPTION: (rotate right A register) - bit rotate A register right (not through the carry flag)
                // PC:+1
                // Cycles: 4
                // Z:0 S:0 H:0 C:?
                manipulate_8bit_register!(self: a => rotate_right_retain_zero => a);
                (self.pc.wrapping_add(1), 4)
            }
            Instruction::RLCA => {
                // DESCRIPTION: (rotate left A register) - bit rotate A register left (not through the carry flag)
                // PC:+1
                // Cycles: 4
                // Z:0 S:0 H:0 C:?
                manipulate_8bit_register!(self: a => rotate_left_retain_zero => a);
                (self.pc.wrapping_add(1), 4)
            }
            Instruction::CPL => {
                // DESCRIPTION: (complement) - toggle every bit of the A register
                // PC:+1
                // Cycles: 4
                // Z:- S:1 H:1 C:-
                manipulate_8bit_register!(self: a => complement => a);
                (self.pc.wrapping_add(1), 4)
            }
            Instruction::BIT(register, bit_position) => {
                // DESCRIPTION: (bit test) - test to see if a specific bit of a specific register is set
                // PC:+2
                // WHEN: target is (HL):
                // Cycles: 16
                // ELSE:
                // Cycles: 8
                // Z:? S:0 H:1 C:-
                prefix_instruction!(register, self.bit_test @ bit_position)
            }
            Instruction::RES(register, bit_position) => {
                // DESCRIPTION: (bit reset) - set a specific bit of a specific register to 0
                // PC:+2
                // WHEN: target is (HL):
                // Cycles: 16
                // ELSE:
                // Cycles: 8
                // Z:- S:- H:- C:-
                prefix_instruction!(register, (self.reset_bit @ bit_position) => reg)
            }
            Instruction::SET(register, bit_position) => {
                // DESCRIPTION: (bit set) - set a specific bit of a specific register to 1
                // PC:+2
                // WHEN: target is (HL):
                // Cycles: 16
                // ELSE:
                // Cycles: 8
                // Z:- S:- H:- C:-
                prefix_instruction!(register, (self.set_bit @ bit_position) => reg)
            }
            Instruction::SRL(register) => {
                // DESCRIPTION: (shift right logical) - bit shift a specific register right by 1
                // PC:+2
                // WHEN: target is (HL):
                // Cycles: 16
                // ELSE:
                // Cycles: 8
                // Z:? S:0 H:0 C:?
                prefix_instruction!(register, self.shift_right_logical => reg)
            }
            Instruction::RR(register) => {
                // DESCRIPTION: (rotate right) - bit rotate a specific register right by 1 through the carry flag
                // PC:+2
                // WHEN: target is (HL):
                // Cycles: 16
                // ELSE:
                // Cycles: 8
                // Z:? S:0 H:0 C:?
                prefix_instruction!(register, self.rotate_right_through_carry_set_zero => reg)
            }
            Instruction::RL(register) => {
                // DESCRIPTION: (rotate left) - bit rotate a specific register left by 1 through the carry flag
                // PC:+2
                // WHEN: target is (HL):
                // Cycles: 16
                // ELSE:
                // Cycles: 8
                // Z:? S:0 H:0 C:?
                prefix_instruction!(register, self.rotate_left_through_carry_set_zero => reg)
            }
            Instruction::RRC(register) => {
                // DESCRIPTION: (rotate right) - bit rotate a specific register right by 1 (not through the carry flag)
                // PC:+2
                // WHEN: target is (HL):
                // Cycles: 16
                // ELSE:
                // Cycles: 8
                // Z:? S:0 H:0 C:?
                prefix_instruction!(register, self.rotate_right_set_zero => reg)
            }
            Instruction::RLC(register) => {
                // DESCRIPTION: (rotate left) - bit rotate a specific register left by 1 (not through the carry flag)
                // PC:+2
                // WHEN: target is (HL):
                // Cycles: 16
                // ELSE:
                // Cycles: 8
                // Z:? S:0 H:0 C:?
                prefix_instruction!(register, self.rotate_left_set_zero => reg)
            }
            Instruction::SRA(register) => {
                // DESCRIPTION: (shift right arithmetic) - arithmetic shift a specific register right by 1
                // PC:+2
                // WHEN: target is (HL):
                // Cycles: 16
                // ELSE:
                // Cycles: 8
                // Z:? S:0 H:0 C:?
                prefix_instruction!(register, self.shift_right_arithmetic => reg)
            }
            Instruction::SLA(register) => {
                // DESCRIPTION: (shift left arithmetic) - arithmetic shift a specific register left by 1
                // PC:+2
                // WHEN: target is (HL):
                // Cycles: 16
                // ELSE:
                // Cycles: 8
                // Z:? S:0 H:0 C:?
                prefix_instruction!(register, self.shift_left_arithmetic => reg)
            }
            Instruction::SWAP(register) => {
                // DESCRIPTION: switch upper and lower nibble of a specific register
                // PC:+2
                // WHEN: target is (HL):
                // Cycles: 16
                // ELSE:
                // Cycles: 8
                // Z:? S:0 H:0 C:0
                prefix_instruction!(register, self.swap_nibbles => reg)
            },
            Instruction::JP(test) => {
                // DESCRIPTION: conditionally jump to the address stored in the next word in memory
                // PC:?/+3
                // Cycles: 16/12
                // Z:- N:- H:- C:-
                let jump_condition = match test {
                    JumpTest::NotZero => !self.registers.f.zero,
                    JumpTest::NotCarry => !self.registers.f.carry,
                    JumpTest::Zero => self.registers.f.zero,
                    JumpTest::Carry => self.registers.f.carry,
                    JumpTest::Always => true
                };
                self.jump(jump_condition)
            }
            Instruction::JR(test) => {
                // DESCRIPTION: conditionally jump to the address that is N bytes away in memory
                // where N is the next byte in memory interpreted as a signed byte
                // PC:?/+2
                // Cycles: 16/12
                // Z:- N:- H:- C:-
                let jump_condition = match test {
                    JumpTest::NotZero => !self.registers.f.zero,
                    JumpTest::NotCarry => !self.registers.f.carry,
                    JumpTest::Zero => self.registers.f.zero,
                    JumpTest::Carry => self.registers.f.carry,
                    JumpTest::Always => true
                };
                self.jump_relative(jump_condition)
            }
            Instruction::JPI => {
                // DESCRIPTION: jump to the address stored in HL
                // 1
                // PC:HL
                // Cycles: 4
                // Z:- N:- H:- C:-
                (self.registers.get_hl(), 4)
            }
            Instruction::LD(load_type) => {
                match load_type {
                    // DESCRIPTION: load byte store in a particular register into another
                    // particular register
                    // WHEN: source is d8
                    // PC:+2
                    // Cycles: 8
                    // WHEN: source is (HL)
                    // PC:+1
                    // Cycles: 8
                    // ELSE:
                    // PC:+1
                    // Cycles: 4
                    // Z:- N:- H:- C:-
                    LoadType::Byte(target, source) => {
                        let source_value = match source {
                            LoadByteSource::A => self.registers.a,
                            LoadByteSource::B => self.registers.b,
                            LoadByteSource::C => self.registers.c,
                            LoadByteSource::D => self.registers.d,
                            LoadByteSource::E => self.registers.e,
                            LoadByteSource::H => self.registers.h,
                            LoadByteSource::L => self.registers.l,
                            LoadByteSource::D8 => self.read_next_byte(),
                            LoadByteSource::HLI => self.bus.read_byte(self.registers.get_hl())
                        };
                        match target {
                            LoadByteTarget::A => self.registers.a = source_value,
                            LoadByteTarget::B => self.registers.b = source_value,
                            LoadByteTarget::C => self.registers.c = source_value,
                            LoadByteTarget::D => self.registers.d = source_value,
                            LoadByteTarget::E => self.registers.e = source_value,
                            LoadByteTarget::H => self.registers.h = source_value,
                            LoadByteTarget::L => self.registers.l = source_value,
                            LoadByteTarget::HLI => self.bus.write_byte(self.registers.get_hl(), source_value)
                        };
                        match source {
                            LoadByteSource::D8  => (self.pc.wrapping_add(2), 8),
                            LoadByteSource::HLI => (self.pc.wrapping_add(1), 8),
                            _                   => (self.pc.wrapping_add(1), 4),
                        }
                    },
                    // DESCRIPTION: load next word in memory into a particular register
                    // PC:+3
                    // Cycles: 12
                    // Z:- N:- H:- C:-
                    LoadType::Word(target) => {
                        let word = self.read_next_word();
                        match target {
                            LoadWordTarget::BC => self.registers.set_bc(word),
                            LoadWordTarget::DE => self.registers.set_de(word),
                            LoadWordTarget::HL => self.registers.set_hl(word),
                            LoadWordTarget::SP => self.sp = word,
                        };
                        (self.pc.wrapping_add(3), 12)
                    },
                    // DESCRIPTION: load a particular value stored at the source address into A
                    // WHEN: source is byte indirect
                    // PC:+2
                    // Cycles: 8
                    // WHEN: source is word indirect
                    // PC:+3
                    // Cycles: 16
                    // ELSE:
                    // PC:+1
                    // Cycles: 8
                    // Z:- N:- H:- C:-
                    LoadType::AFromIndirect(source) => {
                        self.registers.a = match source {
                            Indirect::BCIndirect => self.bus.read_byte(self.registers.get_bc()),
                            Indirect::DEIndirect => self.bus.read_byte(self.registers.get_de()),
                            Indirect::HLIndirectMinus => {
                                let hl = self.registers.get_hl();
                                self.registers.set_hl(hl.wrapping_sub(1));
                                self.bus.read_byte(hl)
                            }
                            Indirect::HLIndirectPlus => {
                                let hl = self.registers.get_hl();
                                self.registers.set_hl(hl.wrapping_add(1));
                                self.bus.read_byte(hl)
                            }
                            Indirect::WordIndirect => self.bus.read_byte(self.read_next_word()),
                            Indirect::LastByteIndirect => self.bus.read_byte(0xFF00 + self.registers.c as u16),
                        };

                        match source {
                            Indirect::WordIndirect     => (self.pc.wrapping_add(3), 16),
                            Indirect::LastByteIndirect => (self.pc.wrapping_add(2), 8),
                            _                          => (self.pc.wrapping_add(1), 8)
                        }
                    },
                    // DESCRIPTION: load the A register into memory at the source address
                    // WHEN: instruction.source is byte indirect
                    // PC:+2
                    // Cycles: 8
                    // WHEN: instruction.source is word indirect
                    // PC:+3
                    // Cycles: 16
                    // ELSE:
                    // PC:+1
                    // Cycles: 8
                    // Z:- N:- H:- C:-
                    LoadType::IndirectFromA(target) => {
                        let a = self.registers.a;
                        match target {
                            Indirect::BCIndirect => {
                                let bc = self.registers.get_bc();
                                self.bus.write_byte(bc, a)
                            }
                            Indirect::DEIndirect => {
                                let de = self.registers.get_de();
                                self.bus.write_byte(de, a)
                            }
                            Indirect::HLIndirectMinus => {
                                let hl = self.registers.get_hl();
                                self.registers.set_hl(hl.wrapping_sub(1));
                                self.bus.write_byte(hl, a);
                            }
                            Indirect::HLIndirectPlus => {
                                let hl = self.registers.get_hl();
                                self.registers.set_hl(hl.wrapping_add(1));
                                self.bus.write_byte(hl, a);
                            }
                            Indirect::WordIndirect => {
                                let word = self.read_next_word();
                                self.bus.write_byte(word, a);
                            },
                            Indirect::LastByteIndirect => {
                                let c = self.registers.c as u16;
                                self.bus.write_byte(0xFF00 + c, a);
                            }
                        };

                        match target {
                            Indirect::WordIndirect     => (self.pc.wrapping_add(3), 16),
                            Indirect::LastByteIndirect => (self.pc.wrapping_add(2), 8),
                            _                          => (self.pc.wrapping_add(1), 8)
                        }
                    },
                    // DESCRIPTION: Load the value in A into memory location located at 0xFF plus
                    // an offset stored as the next byte in memory
                    // PC:+2
                    // Cycles: 12
                    // Z:- N:- H:- C:-
                    LoadType::ByteAddressFromA => {
                        let offset = self.bus.read_byte(self.pc + 1) as u16;
                        self.bus.write_byte(0xFF00 + offset, self.registers.a);
                        (self.pc.wrapping_add(2), 12)
                    },
                    // DESCRIPTION: Load the value located at 0xFF plus an offset stored as the next byte in memory into A
                    // PC:+2
                    // Cycles: 12
                    // Z:- N:- H:- C:-
                    LoadType::AFromByteAddress => {
                        self.registers.a = self.bus.read_byte(0xFF00 + self.read_next_byte() as u16);
                        (self.pc.wrapping_add(2), 12)
                    },
                    // DESCRIPTION: Load the value in HL into SP
                    // PC:+1
                    // Cycles: 8
                    // Z:- N:- H:- C:-
                    LoadType::SPFromHL => {
                        self.sp = self.registers.get_hl();
                        (self.pc.wrapping_add(1), 8)
                    },
                    // DESCRIPTION: Load memory address with the contents of SP
                    // PC:+3
                    // Cycles: 20
                    // Z:- N:- H:- C:-
                    LoadType::IndirectFromSP => {
                        let address = self.read_next_word();
                        let sp = self.sp;
                        self.bus.write_byte(address, (sp & 0xFF) as u8);
                        self.bus.write_byte(address.wrapping_add(1), ((sp & 0xFF00) >> 8) as u8);
                        (self.pc.wrapping_add(3), 20)
                    },
                    // DESCRIPTION: load HL with SP plus some specified byte
                    // PC:+2
                    // Cycles: 12
                    // Z:0 N:0 H:? C:?
                    LoadType::HLFromSPN => {
                        let value = self.read_next_byte() as i8 as i16 as u16;
                        let result = self.sp.wrapping_add(value);
                        self.registers.set_hl(result);
                        self.registers.f.zero = false;
                        self.registers.f.subtract = false;
                        // Half and whole carry are computed at the nibble and byte level instead
                        // of the byte and word level like you might expect for 16 bit values
                        self.registers.f.half_carry = (self.sp & 0xF) + (value & 0xF) > 0xF;
                        self.registers.f.carry = (self.sp & 0xFF) + (value & 0xFF) > 0xFF;
                        (self.pc.wrapping_add(2), 12)
                    },
                }
            }
            Instruction::PUSH(target) => {
                // DESCRIPTION: push a value from a given register on to the stack
                // PC:+1
                // Cycles: 16
                // Z:- N:- H:- C:-
                let value = match target {
                    StackTarget::AF => self.registers.get_af(),
                    StackTarget::BC => self.registers.get_bc(),
                    StackTarget::DE => self.registers.get_de(),
                    StackTarget::HL => self.registers.get_hl(),
                };
                self.push(value);
                (self.pc.wrapping_add(1), 16)
            }
            Instruction::POP(target) => {
                // DESCRIPTION: pop a value from the stack and store it in a given register
                // PC:+1
                // Cycles: 12
                // WHEN: target is AF
                // Z:? N:? H:? C:?
                // ELSE:
                // Z:- N:- H:- C:-
                let result = self.pop();
                match target {
                    StackTarget::AF => self.registers.set_af(result),
                    StackTarget::BC => self.registers.set_bc(result),
                    StackTarget::DE => self.registers.set_de(result),
                    StackTarget::HL => self.registers.set_hl(result),
                };
                (self.pc.wrapping_add(1), 12)
            }
            Instruction::CALL(test) => {
                // DESCRIPTION: Conditionally PUSH the would be instruction on to the
                // stack and then jump to a specific address
                // PC:?/+3
                // Cycles: 24/12
                // Z:- N:- H:- C:-
                let jump_condition = match test {
                    JumpTest::NotZero => !self.registers.f.zero,
                    JumpTest::NotCarry => !self.registers.f.carry,
                    JumpTest::Zero => self.registers.f.zero,
                    JumpTest::Carry => self.registers.f.carry,
                    JumpTest::Always => true
                };
                self.call(jump_condition)
            }
            Instruction::RET(test) => {
                // DESCRIPTION: Conditionally POP two bytes from the stack and jump to that address
                // PC:?/+1
                // WHEN: condition is 'always'
                // Cycles: 16/8
                // ELSE:
                // Cycles: 20/8
                // Z:- N:- H:- C:-
                let jump_condition = match test {
                    JumpTest::NotZero => !self.registers.f.zero,
                    JumpTest::NotCarry => !self.registers.f.carry,
                    JumpTest::Zero => self.registers.f.zero,
                    JumpTest::Carry => self.registers.f.carry,
                    JumpTest::Always => true
                };
                let next_pc = self.return_(jump_condition);

                let cycles = if jump_condition && test == JumpTest::Always {
                    16
                } else if jump_condition {
                    20
                } else {
                    8
                };
                (next_pc, cycles)
            }
            Instruction::NOP => {
                // PC:+1
                // Cycles: 4
                // Z:- N:- H:- C:-
                (self.pc.wrapping_add(1), 4)
            }
            Instruction::HALT => {
                // PC:+1
                // Cycles: 4
                // Z:- N:- H:- C:-
                self.is_halted = true;
                (self.pc.wrapping_add(1), 4)
            }
        }
    }

    #[inline(always)]
    fn push(&mut self, value: u16) {
        self.sp = self.sp.wrapping_sub(1);
        self.bus.write_byte(self.sp, ((value & 0xFF00) >> 8) as u8);

        self.sp = self.sp.wrapping_sub(1);
        self.bus.write_byte(self.sp, (value & 0xFF) as u8);
    }

    #[inline(always)]
    fn pop(&mut self) -> u16 {
        let lsb = self.bus.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);

        let msb = self.bus.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);

        (msb << 8) | lsb
    }

    #[inline(always)]
    fn read_next_word(&self) -> u16 {
        // Gameboy is little endian so read pc + 2 as most significant bit
        // and pc + 1 as least significant bit
        ((self.bus.read_byte(self.pc + 2) as u16) << 8)  | (self.bus.read_byte(self.pc + 1) as u16)
    }

    #[inline(always)]
    fn read_next_byte(&self) -> u8 {
        self.bus.read_byte(self.pc + 1)
    }

    #[inline(always)]
    fn inc_8bit(&mut self, value: u8) -> u8 {
        let new_value = value.wrapping_add(1);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        // Half Carry is set if the lower nibble of the value is equal to 0xF.
        // If the nibble is equal to 0xF (0b1111) that means incrementing the value
        // by 1 would cause a carry from the lower nibble to the upper nibble.
        self.registers.f.half_carry = value & 0xF == 0xF;
        new_value
    }

    #[inline(always)]
    fn inc_16bit(&mut self, value: u16) -> u16 {
        value.wrapping_add(1)
    }

    #[inline(always)]
    fn dec_8bit(&mut self, value: u8) -> u8 {
        let new_value = value.wrapping_sub(1);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = true;
        // Half Carry is set if the lower nibble of the value is equal to 0x0.
        // If the nibble is equal to 0x0 (0b0000) that means decrementing the value
        // by 1 would cause a carry from the upper nibble to the lower nibble.
        self.registers.f.half_carry = value & 0xF == 0x0;
        new_value
    }

    #[inline(always)]
    fn dec_16bit(&mut self, value: u16) -> u16 {
        value.wrapping_sub(1)
    }

    #[inline(always)]
    fn add_without_carry(&mut self, value: u8) -> u8 {
        self.add(value, false)
    }

    #[inline(always)]
    fn add_with_carry(&mut self, value: u8) -> u8 {
        self.add(value, true)
    }

    #[inline(always)]
    fn add(&mut self, value: u8, add_carry: bool) -> u8 {
        let additional_carry = if add_carry && self.registers.f.carry { 1 } else { 0 };
        let (add, carry) = self.registers.a.overflowing_add(value);
        let (add2, carry2) = add.overflowing_add(additional_carry);
        self.registers.f.zero = add2 == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = carry || carry2;
        // Half Carry is set if adding the lower nibbles of the value and register A
        // together (plus the optional carry bit) result in a value bigger the 0xF.
        // If the result is larger than 0xF than the addition caused a carry from
        // the lower nibble to the upper nibble.
        self.registers.f.half_carry = ((self.registers.a & 0xF) + (value & 0xF) + additional_carry) > 0xF;
        add2
    }

    #[inline(always)]
    fn add_hl(&mut self, value: u16) -> u16 {
        let hl = self.registers.get_hl();
        let (result, carry) = hl.overflowing_add(value);
        self.registers.f.carry = carry;
        self.registers.f.subtract = false;
        // Half carry tests if we flow over the 11th bit i.e. does adding the two
        // numbers together cause the 11th bit to flip
        let mask = 0b111_1111_1111; // mask out bits 11-15
        self.registers.f.half_carry = (value & mask) + (hl & mask) > mask;

        result
    }

    #[inline(always)]
    fn sub_without_carry(&mut self, value: u8) -> u8 {
        self.sub(value, false)
    }

    #[inline(always)]
    fn sub_with_carry(&mut self, value: u8) -> u8 {
        self.sub(value, true)
    }

    #[inline(always)]
    fn sub(&mut self, value: u8, sub_carry: bool) -> u8 {
        let additional_carry = if sub_carry && self.registers.f.carry { 1 } else { 0 };
        let (sub, carry) = self.registers.a.overflowing_sub(value);
        let (sub2, carry2) = sub.overflowing_sub(additional_carry);
        self.registers.f.zero = sub2 == 0;
        self.registers.f.subtract = true;
        self.registers.f.carry = carry || carry2;
        // Half Carry is set if subtracting the lower nibbles of the value (and the
        // optional carry bit) with register a will result in a value lower than 0x0.
        // To avoid underflowing in this test, we can check if the lower nibble of a
        // is less than the lower nibble of the value (with the additional carry)
        self.registers.f.half_carry = (self.registers.a & 0xF) < (value & 0xF) + additional_carry;
        sub2
    }

    #[inline(always)]
    fn and(&mut self, value: u8) -> u8 {
        let new_value = self.registers.a & value;
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = true;
        self.registers.f.carry = false;
        new_value
    }

    #[inline(always)]
    fn or(&mut self, value: u8) -> u8 {
        let new_value = self.registers.a | value;
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = false;
        new_value
    }

    #[inline(always)]
    fn xor(&mut self, value: u8) -> u8 {
        let new_value = self.registers.a ^ value;
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = false;
        new_value
    }

    #[inline(always)]
    fn compare(&mut self, value: u8) {
        self.registers.f.zero = self.registers.a == value;
        self.registers.f.subtract = true;
        // Half Carry is set if subtracting the lower nibbles of the value with register
        // a will result in a value lower than 0x0.  To avoid underflowing in this test,
        // we can check if the lower nibble of a is less than the lower nibble of the value
        self.registers.f.half_carry = (self.registers.a & 0xF) < (value & 0xF);
        self.registers.f.carry = self.registers.a < value;
    }

    #[inline(always)]
    fn rotate_right_through_carry_retain_zero(&mut self, value: u8) -> u8 {
        self.rotate_right_through_carry(value, false)
    }

    #[inline(always)]
    fn rotate_right_through_carry_set_zero(&mut self, value: u8) -> u8 {
        self.rotate_right_through_carry(value, true)
    }

    #[inline(always)]
    fn rotate_right_through_carry(&mut self, value: u8, set_zero: bool) -> u8 {
        let carry_bit = if self.registers.f.carry { 1 } else { 0 } << 7;
        let new_value = carry_bit | (value >> 1);
        self.registers.f.zero = set_zero && new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = value & 0b1 == 0b1;
        new_value
    }

    #[inline(always)]
    fn rotate_left_through_carry_retain_zero(&mut self, value: u8) -> u8 {
        self.rotate_left_through_carry(value, false)
    }

    #[inline(always)]
    fn rotate_left_through_carry_set_zero(&mut self, value: u8) -> u8 {
        self.rotate_left_through_carry(value, true)
    }

    #[inline(always)]
    fn rotate_left_through_carry(&mut self, value: u8, set_zero: bool) -> u8 {
        let carry_bit = if self.registers.f.carry { 1 } else { 0 };
        let new_value = (value << 1) | carry_bit;
        self.registers.f.zero = set_zero && new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = (value & 0x80) == 0x80;
        new_value
    }

    #[inline(always)]
    fn rotate_right_set_zero(&mut self, value: u8) -> u8 {
        self.rotate_right(value, true)
    }

    #[inline(always)]
    fn rotate_right_retain_zero(&mut self, value: u8) -> u8 {
        self.rotate_right(value, false)
    }

    #[inline(always)]
    fn rotate_left_set_zero(&mut self, value: u8) -> u8 {
        self.rotate_left(value, true)
    }

    #[inline(always)]
    fn rotate_left_retain_zero(&mut self, value: u8) -> u8 {
        self.rotate_left(value, false)
    }

    #[inline(always)]
    fn rotate_left(&mut self, value: u8, set_zero: bool) -> u8 {
        let new_value = value.rotate_left(1);
        self.registers.f.zero = set_zero && new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = value & 0x80 == 0x80;
        new_value
    }

    #[inline(always)]
    fn rotate_right(&mut self, value: u8, set_zero: bool) -> u8 {
        let new_value = value.rotate_right(1);
        self.registers.f.zero = set_zero && new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = value & 0b1 == 0b1;
        new_value
    }

    #[inline(always)]
    fn complement(&mut self, value: u8) -> u8 {
        let new_value = !value;
        self.registers.f.subtract = true;
        self.registers.f.half_carry = true;
        new_value
    }

    #[inline(always)]
    fn bit_test(&mut self, value: u8, bit_position: BitPosition) {
        let bit_position: u8 = bit_position.into();
        let result = (value >> bit_position) & 0b1;
        self.registers.f.zero = result == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = true;

    }

    #[inline(always)]
    fn reset_bit(&mut self, value: u8, bit_position: BitPosition) -> u8 {
        let bit_position: u8 = bit_position.into();
        value & !(1 << bit_position)
    }

    #[inline(always)]
    fn set_bit(&mut self, value: u8, bit_position: BitPosition) -> u8 {
        let bit_position: u8 = bit_position.into();
        value | (1 << bit_position)
    }

    #[inline(always)]
    fn shift_right_logical(&mut self, value: u8) -> u8 {
        let new_value = value >> 1;
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = value & 0b1 == 0b1;
        new_value
    }

    #[inline(always)]
    fn shift_right_arithmetic(&mut self, value: u8) -> u8 {
        let msb = value & 0x80;
        let new_value = msb | (value >> 1);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = value & 0b1 == 0b1;
        new_value
    }

    #[inline(always)]
    fn shift_left_arithmetic(&mut self, value: u8) -> u8 {
        let new_value = value << 1;
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = value & 0x80 == 0x80;
        new_value
    }

    #[inline(always)]
    fn swap_nibbles(&mut self, value: u8) -> u8 {
        let new_value = ((value & 0xf) << 4) | ((value & 0xf0) >> 4);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = false;
        new_value
    }

    #[inline(always)]
    fn jump(&self, should_jump: bool) -> (u16, u8) {
        if should_jump {
            (self.read_next_word(), 16)
        } else {
            (self.pc.wrapping_add(3), 12)
        }
    }

    #[inline(always)]
    fn jump_relative(&self, should_jump: bool) -> (u16, u8) {
        let next_step = self.pc.wrapping_add(2);
         if should_jump {
            let offset = self.read_next_byte() as i8;
            let pc = if offset >= 0 {
                next_step.wrapping_add(offset as u16)
            } else {
                next_step.wrapping_sub(offset.abs() as u16)
            };
            (pc, 16)
        } else {
            (next_step, 12)
        }
    }

    #[inline(always)]
    fn call(&mut self, should_jump: bool) -> (u16, u8) {
        let next_pc = self.pc.wrapping_add(3);
        if should_jump {
            self.push(next_pc);
            (self.read_next_word(), 24)
        } else {
            (next_pc, 12)
        }
    }

    #[inline(always)]
    fn return_(&mut self, should_jump: bool) -> u16 {
        if should_jump {
            self.pop()
        } else {
            self.pc.wrapping_add(1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_instruction {
        ( $instruction:expr, $( $($register:ident).* => $value:expr ),* ) => {
            {
                let mut cpu = CPU::new(None);
                $(
                    cpu.registers$(.$register)* = $value;
                 )*
                cpu.execute($instruction);
                cpu
            }
        };
    }
    macro_rules! check_flags {
        ( $cpu:ident,  zero => $zero:ident, subtract => $subtract:ident, half_carry => $half_carry:ident, carry => $carry:ident ) => {
            {
                let flags = $cpu.registers.f;
                println!("Flags: {:?}", flags);
                assert_eq!(flags.zero, $zero);
                assert_eq!(flags.subtract, $subtract);
                assert_eq!(flags.half_carry, $half_carry);
                assert_eq!(flags.carry, $carry);
            }
        };
    }

    // INC
    #[test]
    fn execute_inc_8bit_non_overflow() {
        let cpu = test_instruction!(Instruction::INC(IncDecTarget::A), a => 0x7);

        assert_eq!(cpu.registers.a, 0x8);
        check_flags!(cpu, zero => false, subtract => false, half_carry => false, carry => false);
    }

    #[test]
    fn execute_inc_8bit_half_carry() {
        let cpu = test_instruction!(Instruction::INC(IncDecTarget::A), a => 0xF);

        assert_eq!(cpu.registers.a, 0x10);
        check_flags!(cpu, zero => false, subtract => false, half_carry => true, carry => false);
    }

    #[test]
    fn execute_inc_8bit_overflow() {
        let cpu = test_instruction!(Instruction::INC(IncDecTarget::A), a => 0xFF);

        assert_eq!(cpu.registers.a, 0x0);
        check_flags!(cpu, zero => true, subtract => false, half_carry => true, carry => false);
    }

    #[test]
    fn execute_inc_16bit_byte_overflow() {
        let instruction = Instruction::INC(IncDecTarget::BC);
        let mut cpu = CPU::new(None);
        cpu.registers.set_bc(0xFF);
        cpu.execute(instruction);

        assert_eq!(cpu.registers.get_bc(), 0x0100);
        assert_eq!(cpu.registers.b, 0x01);
        assert_eq!(cpu.registers.c, 0x00);
        check_flags!(cpu, zero => false, subtract => false, half_carry => false, carry => false);
    }

    #[test]
    fn execute_inc_16bit_overflow() {
        let instruction = Instruction::INC(IncDecTarget::BC);
        let mut cpu = CPU::new(None);
        cpu.registers.set_bc(0xFFFF);
        cpu.execute(instruction);

        assert_eq!(cpu.registers.get_bc(), 0x0);
        assert_eq!(cpu.registers.b, 0x00);
        assert_eq!(cpu.registers.c, 0x00);
        check_flags!(cpu, zero => false, subtract => false, half_carry => false, carry => false);
    }

    // DEC
    #[test]
    fn execute_dec_8bit_non_overflow() {
        let cpu = test_instruction!(Instruction::DEC(IncDecTarget::A), a => 0x7);

        assert_eq!(cpu.registers.a, 0x6);
        check_flags!(cpu, zero => false, subtract => true, half_carry => false, carry => false);
    }

    #[test]
    fn execute_dec_8bit_half_carry() {
        let cpu = test_instruction!(Instruction::DEC(IncDecTarget::A), a => 0x80);

        assert_eq!(cpu.registers.a, 0x7f);
        check_flags!(cpu, zero => false, subtract => true, half_carry => true, carry => false);
    }

    #[test]
    fn execute_dec_8bit_underflow() {
        let cpu = test_instruction!(Instruction::DEC(IncDecTarget::A), a => 0x0);

        assert_eq!(cpu.registers.a, 0xFF);
        check_flags!(cpu, zero => false, subtract => true, half_carry => true, carry => false);
    }

    #[test]
    fn execute_dec_16bit_underflow() {
        let instruction = Instruction::DEC(IncDecTarget::BC);
        let mut cpu = CPU::new(None);
        cpu.registers.set_bc(0x0000);
        cpu.execute(instruction);

        assert_eq!(cpu.registers.get_bc(), 0xFFFF);
        assert_eq!(cpu.registers.b, 0xFF);
        assert_eq!(cpu.registers.c, 0xFF);
        check_flags!(cpu, zero => false, subtract => false, half_carry => false, carry => false);
    }

    // ADD
    #[test]
    fn execute_add_8bit_non_overflow_target_a() {
        let cpu = test_instruction!(Instruction::ADD(ArithmeticTarget::A), a => 0x7);

        assert_eq!(cpu.registers.a, 0xe);
        check_flags!(cpu, zero => false, subtract => false, half_carry => false, carry => false);
    }

    #[test]
    fn execute_add_8bit_non_overflow_target_c() {
        let cpu = test_instruction!(Instruction::ADD(ArithmeticTarget::C), a => 0x7, c => 0x3);

        assert_eq!(cpu.registers.a, 0xa);
        check_flags!(cpu, zero => false, subtract => false, half_carry => false, carry => false);
    }

    #[test]
    fn execute_add_8bit_non_overflow_target_c_with_carry() {
        let cpu = test_instruction!(Instruction::ADD(ArithmeticTarget::C), a => 0x7, c => 0x3, f.carry => true);

        assert_eq!(cpu.registers.a, 0xa);
        check_flags!(cpu, zero => false, subtract => false, half_carry => false, carry => false);
    }

    #[test]
    fn execute_add_8bit_carry() {
        let cpu = test_instruction!(Instruction::ADD(ArithmeticTarget::B), a => 0xFC, b => 0x9);

        assert_eq!(cpu.registers.a, 0x05);
        check_flags!(cpu, zero => false, subtract => false, half_carry => true, carry => true);
    }

    // ADDHL
    #[test]
    fn execute_add_hl() {
        let cpu = test_instruction!(Instruction::ADDHL(ADDHLTarget::BC), b => 0x07, c => 0x00, h => 0x03, l => 0x00);

        assert_eq!(cpu.registers.get_hl(), 0x0A00);
        check_flags!(cpu, zero => false, subtract => false, half_carry => true, carry => false);
    }

    // ADC
    #[test]
    fn execute_addc_8bit_non_overflow_target_a_no_carry() {
        let cpu = test_instruction!(Instruction::ADD(ArithmeticTarget::A), a => 0x7);

        assert_eq!(cpu.registers.a, 0xe);
        check_flags!(cpu, zero => false, subtract => false, half_carry => false, carry => false);
    }

    #[test]
    fn execute_addc_8bit_non_overflow_target_a_with_carry() {
        let cpu = test_instruction!(Instruction::ADC(ArithmeticTarget::A), a => 0x7, f.carry => true);

        assert_eq!(cpu.registers.a, 0xf);
        check_flags!(cpu, zero => false, subtract => false, half_carry => false, carry => false);
    }

    #[test]
    fn execute_addc_8bit_non_overflow_target_c_with_carry() {
        let cpu = test_instruction!(Instruction::ADC(ArithmeticTarget::C), a => 0x7, c => 0x3, f.carry => true);

        assert_eq!(cpu.registers.a, 0xb);
        check_flags!(cpu, zero => false, subtract => false, half_carry => false, carry => false);
    }

    #[test]
    fn execute_addc_8bit_carry_with_carry() {
        let cpu = test_instruction!(Instruction::ADC(ArithmeticTarget::B), a => 0xFC, b => 0x3, f.carry => true);

        assert_eq!(cpu.registers.a, 0x00);
        check_flags!(cpu, zero => true, subtract => false, half_carry => true, carry => true);
    }

    // SUB
    #[test]
    fn execute_sub_8bit_non_underflow_target_a() {
        let cpu = test_instruction!(Instruction::SUB(ArithmeticTarget::A), a => 0x7);

        assert_eq!(cpu.registers.a, 0x0);
        check_flags!(cpu, zero => true, subtract => true, half_carry => false, carry => false);
    }

    #[test]
    fn execute_sub_8bit_non_underflow_target_c() {
        let cpu = test_instruction!(Instruction::SUB(ArithmeticTarget::C), a => 0x7, c => 0x3);

        assert_eq!(cpu.registers.a, 0x4);
        check_flags!(cpu, zero => false, subtract => true, half_carry => false, carry => false);
    }

    #[test]
    fn execute_sub_8bit_non_overflow_target_c_with_carry() {
        let cpu = test_instruction!(Instruction::SUB(ArithmeticTarget::C), a => 0x7, c => 0x3, f.carry => true);

        assert_eq!(cpu.registers.a, 0x4);
        check_flags!(cpu, zero => false, subtract => true, half_carry => false, carry => false);
    }

    #[test]
    fn execute_sub_8bit_carry() {
        let cpu = test_instruction!(Instruction::SUB(ArithmeticTarget::B), a => 0x4, b => 0x9);

        assert_eq!(cpu.registers.a, 0xFB);
        check_flags!(cpu, zero => false, subtract => true, half_carry => true, carry => true);
    }

    // SBC
    #[test]
    fn execute_subc_8bit_non_overflow_target_a_no_carry() {
        let cpu = test_instruction!(Instruction::SBC(ArithmeticTarget::A), a => 0x7);

        assert_eq!(cpu.registers.a, 0x0);
        check_flags!(cpu, zero => true, subtract => true, half_carry => false, carry => false);
    }

    #[test]
    fn execute_subc_8bit_non_overflow_target_a_with_carry() {
        let cpu = test_instruction!(Instruction::SBC(ArithmeticTarget::A), a => 0x7, f.carry => true);

        assert_eq!(cpu.registers.a, 0xFF);
        check_flags!(cpu, zero => false, subtract => true, half_carry => true, carry => true);
    }

    #[test]
    fn execute_subc_8bit_non_overflow_target_c_with_carry() {
        let cpu = test_instruction!(Instruction::SBC(ArithmeticTarget::C), a => 0x7, c => 0x3, f.carry => true);

        assert_eq!(cpu.registers.a, 0x3);
        check_flags!(cpu, zero => false, subtract => true, half_carry => false, carry => false);
    }

    // AND
    #[test]
    fn execute_and_8bit() {
        let cpu = test_instruction!(Instruction::AND(ArithmeticTarget::A), a => 0x7);

        assert_eq!(cpu.registers.a, 0x7);
        check_flags!(cpu, zero => false, subtract => false, half_carry => true, carry => false);
    }

    #[test]
    fn execute_and_8bit_with_zero() {
        let cpu = test_instruction!(Instruction::AND(ArithmeticTarget::B), a => 0x8);

        assert_eq!(cpu.registers.a, 0x0);
        check_flags!(cpu, zero => true, subtract => false, half_carry => true, carry => false);
    }

    // OR
    #[test]
    fn execute_or_8bit() {
        let cpu = test_instruction!(Instruction::OR(ArithmeticTarget::A), a => 0x7);

        assert_eq!(cpu.registers.a, 0x7);
        check_flags!(cpu, zero => false, subtract => false, half_carry => false, carry => false);
    }

    #[test]
    fn execute_or_8bit_with_zero() {
        let cpu = test_instruction!(Instruction::OR(ArithmeticTarget::B), a => 0x8);

        assert_eq!(cpu.registers.a, 0x8);
        check_flags!(cpu, zero => false, subtract => false, half_carry => false, carry => false);
    }

    // XOR
    #[test]
    fn execute_xor_8bit() {
        let cpu = test_instruction!(Instruction::XOR(ArithmeticTarget::A), a => 0b0000_0111);

        assert_eq!(cpu.registers.a, 0x0);
        check_flags!(cpu, zero => true, subtract => false, half_carry => false, carry => false);
    }

    #[test]
    fn execute_xor_8bit_with_zero() {
        let cpu = test_instruction!(Instruction::XOR(ArithmeticTarget::B), a => 0x8);

        assert_eq!(cpu.registers.a, 0x8);
        check_flags!(cpu, zero => false, subtract => false, half_carry => false, carry => false);
    }

    // CP
    #[test]
    fn execute_cp_8bit_non_underflow_target_a() {
        let cpu = test_instruction!(Instruction::CP(ArithmeticTarget::A), a => 0x7);

        assert_eq!(cpu.registers.a, 0x7);
        check_flags!(cpu, zero => true, subtract => true, half_carry => false, carry => false);
    }

    #[test]
    fn execute_cp_8bit_non_underflow_target_c() {
        let cpu = test_instruction!(Instruction::CP(ArithmeticTarget::C), a => 0x7, c => 0x3);

        assert_eq!(cpu.registers.a, 0x7);
        check_flags!(cpu, zero => false, subtract => true, half_carry => false, carry => false);
    }

    #[test]
    fn execute_cp_8bit_non_overflow_target_c_with_carry() {
        let cpu = test_instruction!(Instruction::CP(ArithmeticTarget::C), a => 0x7, c => 0x3, f.carry => true);

        assert_eq!(cpu.registers.a, 0x7);
        check_flags!(cpu, zero => false, subtract => true, half_carry => false, carry => false);
    }

    #[test]
    fn execute_cp_8bit_carry() {
        let cpu = test_instruction!(Instruction::CP(ArithmeticTarget::B), a => 0x4, b => 0x9);

        assert_eq!(cpu.registers.a, 0x4);
        check_flags!(cpu, zero => false, subtract => true, half_carry => true, carry => true);
    }

    // RRA
    #[test]
    fn execute_rra_8bit() {
        let cpu = test_instruction!(Instruction::RRA, a => 0b1);

        assert_eq!(cpu.registers.a, 0x0);
        check_flags!(cpu, zero => false, subtract => false, half_carry => false, carry => true);
    }

    // RLA
    #[test]
    fn execute_rla_8bit() {
        let cpu = test_instruction!(Instruction::RLA, a => 0x80);

        assert_eq!(cpu.registers.a, 0x0);
        check_flags!(cpu, zero => false, subtract => false, half_carry => false, carry => true);
    }

    // RRCA
    #[test]
    fn execute_rrca_8bit() {
        let cpu = test_instruction!(Instruction::RRCA, a => 0b1, f.carry => true);

        assert_eq!(cpu.registers.a, 0x80);
        check_flags!(cpu, zero => false, subtract => false, half_carry => false, carry => true);
    }

    // RLCA
    #[test]
    fn execute_rlca_8bit() {
        let cpu = test_instruction!(Instruction::RLCA, a => 0x80, f.carry => true);

        assert_eq!(cpu.registers.a, 0x1);
        check_flags!(cpu, zero => false, subtract => false, half_carry => false, carry => true);
    }

    // CPL
    #[test]
    fn execute_cpl_8bit() {
        let cpu = test_instruction!(Instruction::CPL, a => 0b1011_0100);

        assert_eq!(cpu.registers.a, 0b0100_1011);
        check_flags!(cpu, zero => false, subtract => true, half_carry => true, carry => false);
    }

    // BIT
    #[test]
    fn execute_bit_8bit() {
        let cpu = test_instruction!(Instruction::BIT(PrefixTarget::A, BitPosition::B2), a => 0b1011_0100);

        assert_eq!(cpu.registers.a, 0b1011_0100);
        check_flags!(cpu, zero => false, subtract => false, half_carry => true, carry => false);

        let cpu = test_instruction!(Instruction::BIT(PrefixTarget::A, BitPosition::B1), a => 0b1011_0100);
        assert_eq!(cpu.registers.a, 0b1011_0100);
        check_flags!(cpu, zero => true, subtract => false, half_carry => true, carry => false);
    }

    // RES
    #[test]
    fn execute_res_8bit() {
        let cpu = test_instruction!(Instruction::RES(PrefixTarget::A, BitPosition::B2), a => 0b1011_0100);

        assert_eq!(cpu.registers.a, 0b1011_0000);
        check_flags!(cpu, zero => false, subtract => false, half_carry => false, carry => false);

        let cpu = test_instruction!(Instruction::RES(PrefixTarget::A, BitPosition::B1), a => 0b1011_0100);
        assert_eq!(cpu.registers.a, 0b1011_0100);
        check_flags!(cpu, zero => false, subtract => false, half_carry => false, carry => false);
    }

    // SET
    #[test]
    fn execute_set_8bit() {
        let cpu = test_instruction!(Instruction::SET(PrefixTarget::A, BitPosition::B2), a => 0b1011_0100);

        assert_eq!(cpu.registers.a, 0b1011_0100);
        check_flags!(cpu, zero => false, subtract => false, half_carry => false, carry => false);

        let cpu = test_instruction!(Instruction::SET(PrefixTarget::A, BitPosition::B1), a => 0b1011_0100);
        assert_eq!(cpu.registers.a, 0b1011_0110);
        check_flags!(cpu, zero => false, subtract => false, half_carry => false, carry => false);
    }

    // SRL
    #[test]
    fn execute_srl_8bit() {
        let cpu = test_instruction!(Instruction::SRL(PrefixTarget::A), a => 0b1011_0101);

        assert_eq!(cpu.registers.a, 0b0101_1010);
        check_flags!(cpu, zero => false, subtract => false, half_carry => false, carry => true);
    }

    // RR
    #[test]
    fn execute_rr() {
        let cpu = test_instruction!(Instruction::RR(PrefixTarget::A), a => 0b1011_0101);

        assert_eq!(cpu.registers.a, 0b0101_1010);
        check_flags!(cpu, zero => false, subtract => false, half_carry => false, carry => true);

        let cpu = test_instruction!(Instruction::RR(PrefixTarget::A), a => 0b1011_0101, f.carry => true);

        assert_eq!(cpu.registers.a, 0b1101_1010);
        check_flags!(cpu, zero => false, subtract => false, half_carry => false, carry => true);
    }

    // RL
    #[test]
    fn execute_rl() {
        let cpu = test_instruction!(Instruction::RL(PrefixTarget::A), a => 0b1011_0101);

        assert_eq!(cpu.registers.a, 0b0110_1010);
        check_flags!(cpu, zero => false, subtract => false, half_carry => false, carry => true);

        let cpu = test_instruction!(Instruction::RL(PrefixTarget::A), a => 0b1011_0101, f.carry => true);

        assert_eq!(cpu.registers.a, 0b0110_1011);
        check_flags!(cpu, zero => false, subtract => false, half_carry => false, carry => true);
    }

    // SRA
    #[test]
    fn execute_sra() {
        let cpu = test_instruction!(Instruction::SRA(PrefixTarget::A), a => 0b1011_0101);

        assert_eq!(cpu.registers.a, 0b1101_1010);
        check_flags!(cpu, zero => false, subtract => false, half_carry => false, carry => true);
    }

    // SLA
    #[test]
    fn execute_sla() {
        let cpu = test_instruction!(Instruction::SLA(PrefixTarget::A), a => 0b1011_0101);

        assert_eq!(cpu.registers.a, 0b0110_1010);
        check_flags!(cpu, zero => false, subtract => false, half_carry => false, carry => true);
    }

    // SWAP
    #[test]
    fn execute_swap() {
        let cpu = test_instruction!(Instruction::SWAP(PrefixTarget::A), a => 0b1011_0101);

        assert_eq!(cpu.registers.a, 0b0101_1011);
        check_flags!(cpu, zero => false, subtract => false, half_carry => false, carry => false);
    }

    // JP
    #[test]
    fn execute_jp() {
        let mut cpu = CPU::new(None);
        cpu.pc = 0xF8;
        cpu.bus.rom_bank_0[0xF9] = 0xFC;
        cpu.bus.rom_bank_0[0xFA] = 0x02;
        let next_pc = cpu.execute(Instruction::JP(JumpTest::Always));

        assert_eq!(next_pc, 0x02FC);

        let next_pc = cpu.execute(Instruction::JP(JumpTest::Carry));

        assert_eq!(next_pc, 0xFB);
    }

    // JR
    #[test]
    fn execute_jr() {
        let mut cpu = CPU::new(None);
        cpu.pc = 0xF8;
        cpu.bus.rom_bank_0[0xF9] = 0x4;
        let next_pc = cpu.execute(Instruction::JR(JumpTest::Always));

        assert_eq!(next_pc, 0xFE);

        cpu.bus.rom_bank_0[0xF9] = 0xFC; // == -4
        let next_pc = cpu.execute(Instruction::JR(JumpTest::Always));
        assert_eq!(next_pc, 0xF6);
    }

    // LD a, (??)
    #[test]
    fn execute_ld_a_indirect() {
        let mut cpu = CPU::new(None);
        cpu.registers.set_bc(0xF9);
        cpu.bus.rom_bank_0[0xF9] = 0x4;
        cpu.execute(Instruction::LD(LoadType::AFromIndirect(Indirect::BCIndirect)));

        assert_eq!(cpu.registers.a, 0x04);

        cpu.registers.set_hl(0xA1);
        cpu.bus.rom_bank_0[0xA1] = 0x9;
        cpu.execute(Instruction::LD(LoadType::AFromIndirect(Indirect::HLIndirectPlus)));

        assert_eq!(cpu.registers.a, 0x09);
        assert_eq!(cpu.registers.get_hl(), 0xA2);
    }

    // LD ?, ?
    #[test]
    fn execute_ld_byte() {
        let mut cpu = CPU::new(None);
        cpu.registers.b = 0x4;
        cpu.execute(Instruction::LD(LoadType::Byte(LoadByteTarget::D, LoadByteSource::B)));

        assert_eq!(cpu.registers.b, 0x4);
        assert_eq!(cpu.registers.d, 0x4);
    }

    // PUSH/POP
    #[test]
    fn execute_push_pop() {
        let mut cpu = CPU::new(None);
        cpu.registers.b = 0x4;
        cpu.registers.c = 0x89;
        cpu.sp = 0x10;
        cpu.execute(Instruction::PUSH(StackTarget::BC));

        assert_eq!(cpu.bus.read_byte(0xF), 0x04);
        assert_eq!(cpu.bus.read_byte(0xE), 0x89);
        assert_eq!(cpu.sp, 0xE);

        cpu.execute(Instruction::POP(StackTarget::DE));

        assert_eq!(cpu.registers.d, 0x04);
        assert_eq!(cpu.registers.e, 0x89);
    }

    // -----------------------------------------------------------------------------

    // Step
    #[test]
    fn test_step() {
        let mut cpu = CPU::new(None);
        cpu.bus.rom_bank_0[0] = 0x23; //INC(HL)
        cpu.bus.rom_bank_0[1] = 0xB5; //OR(L)
        cpu.bus.rom_bank_0[2] = 0xCB; //PREFIX
        cpu.bus.rom_bank_0[3] = 0xe8; //SET(B, 5)
        for _ in 0..3 {
            cpu.step();
        }

        assert_eq!(cpu.registers.h, 0b0);
        assert_eq!(cpu.registers.l, 0b1);
        assert_eq!(cpu.registers.a, 0b1);
        assert_eq!(cpu.registers.b, 0b0010_0000);
    }
}
