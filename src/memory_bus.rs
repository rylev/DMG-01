use gpu::{
    GPU,
    TileMap,
    BackgroundAndWindowDataSelect,
    ObjectSize,
};

pub const BOOT_ROM_BEGIN: usize = 0x00;
pub const BOOT_ROM_END: usize = 0xFF;
pub const BOOT_ROM_SIZE: usize = BOOT_ROM_END - BOOT_ROM_BEGIN + 1;

pub const ROM_BANK_0_BEGIN: usize = 0x0000;
pub const ROM_BANK_0_END: usize = 0x3FFF;
pub const ROM_BANK_0_SIZE: usize = ROM_BANK_0_END - ROM_BANK_0_BEGIN + 1;

pub const ROM_BANK_N_BEGIN: usize = 0x4000;
pub const ROM_BANK_N_END: usize = 0x7FFF;
pub const ROM_BANK_N_SIZE: usize = ROM_BANK_N_END - ROM_BANK_N_BEGIN + 1;

pub const VRAM_BEGIN: usize = 0x8000;
pub const VRAM_END: usize = 0x9FFF;
pub const VRAM_SIZE: usize = VRAM_END - VRAM_BEGIN + 1;

pub const EXTERNAL_RAM_BEGIN: usize = 0xA000;
pub const EXTERNAL_RAM_END: usize = 0xBFFF;
pub const EXTERNAL_RAM_SIZE: usize = EXTERNAL_RAM_END - EXTERNAL_RAM_BEGIN + 1;

pub const WORKING_RAM_BEGIN: usize = 0xC000;
pub const WORKING_RAM_END: usize = 0xDFFF;
pub const WORKING_RAM_SIZE: usize = WORKING_RAM_END - WORKING_RAM_BEGIN + 1;

pub const OAM_BEGIN: usize = 0xFE00;
pub const OAM_END: usize = 0xFE9F;
pub const OAM_SIZE: usize = OAM_END - OAM_BEGIN + 1;

pub const IO_REGISTERS_BEGIN: usize = 0xFF00;
pub const IO_REGISTERS_END: usize = 0xFF7F;

pub const ZERO_PAGE_BEGIN: usize = 0xFF80;
pub const ZERO_PAGE_END: usize = 0xFFFE;
pub const ZERO_PAGE_SIZE: usize = ZERO_PAGE_END - ZERO_PAGE_BEGIN + 1;

pub struct MemoryBus {
    boot_rom: Option<[u8; BOOT_ROM_SIZE]>,
    rom_bank_0: [u8; ROM_BANK_0_SIZE],
    rom_bank_n: [u8; ROM_BANK_N_SIZE],
    external_ram: [u8; EXTERNAL_RAM_SIZE],
    working_ram: [u8; WORKING_RAM_SIZE],
    oam: [u8; OAM_SIZE],
    zero_page: [u8; ZERO_PAGE_SIZE],
    pub gpu: GPU
}

impl MemoryBus {
    pub fn new(boot_rom_buffer: Option<Vec<u8>>, game_rom: Vec<u8>) -> MemoryBus {
        let boot_rom = boot_rom_buffer.map(|boot_rom_buffer| {
            if boot_rom_buffer.len() != BOOT_ROM_SIZE {
                panic!("Supplied boot ROM is the wrong size. Is {} bytes but should be {} bytes", boot_rom_buffer.len(), BOOT_ROM_SIZE);
            }
            let mut boot_rom = [0; BOOT_ROM_SIZE];
            boot_rom.copy_from_slice(&boot_rom_buffer);
            boot_rom
        });

        let mut rom_bank_0 = [0; ROM_BANK_0_SIZE];
        for i in 0..ROM_BANK_0_SIZE {
            rom_bank_0[i] = game_rom[i];
        }
        let mut rom_bank_n = [0; ROM_BANK_N_SIZE];
        for i in 0..ROM_BANK_N_SIZE {
            rom_bank_n[i] = game_rom[ROM_BANK_0_SIZE + i];
        }
        MemoryBus {
            // Note: instead of modeling memory as one array of length 0xFFFF, we'll
            // break memory up into it's logical parts.
            boot_rom,
            rom_bank_0,
            rom_bank_n,
            external_ram: [0; EXTERNAL_RAM_SIZE],
            working_ram: [0; WORKING_RAM_SIZE],
            oam: [0; OAM_SIZE],
            zero_page: [0; ZERO_PAGE_SIZE],
            gpu: GPU::new()
        }
    }

    pub fn step(&mut self, cycles: u8) {
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
            ROM_BANK_N_BEGIN ... ROM_BANK_N_END => {
                self.rom_bank_n[address - ROM_BANK_N_BEGIN]
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
                self.gpu.write_vram(address - VRAM_BEGIN, value);
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
                bit(self.gpu.lcd_display_enabled)                   << 7 |
                bit(self.gpu.window_tile_map == TileMap::X9C00)     << 6 |
                bit(self.gpu.window_display_enabled)                << 5 |
                bit(self.gpu.background_and_window_data_select ==
                     BackgroundAndWindowDataSelect::X8000)           << 4 |
                bit(self.gpu.background_tile_map == TileMap::X9C00) << 3 |
                bit(self.gpu.object_size == ObjectSize::OS8X16)     << 2 |
                bit(self.gpu.object_display_enabled)                << 1 |
                bit(self.gpu.background_display_enabled)
            }
            0xFF42 => {
                // Scroll Y Position
                self.gpu.viewport_y_offset
            }
            0xFF44 => {
                // Current Line
                self.gpu.line
            }
            _ => panic!("Reading from an unknown I/O register {:x}", address)
        }
    }

    fn write_io_register(&mut self, address: usize, value: u8) {
        match address {
            0xFF11 => { /* Channel 1 Sound Length and Wave */ }
            0xFF12 => { /* Channel 1 Sound Control */ }
            0xFF13 => { /* Channel 1 Frequency lo */ }
            0xFF14 => { /* Channel 1 Control */ }
            0xFF24 => { /* Sound  Volume */ }
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
                // Viewport Y Offset
                self.gpu.viewport_y_offset = value;
            }
            0xFF47 => {
                // Background Colors Setting
                self.gpu.background_colors = value.into();
            }
            0xFF50 => {
                // Unmap boot ROM
                self.boot_rom = None;
            }
            _ => panic!("Writting '0b{:b}' to an unknown I/O register {:x}", value, address)
        }
    }
}

#[inline(always)]
fn bit(condition: bool) -> u8 {
    if condition { 1 } else { 0 }
}
