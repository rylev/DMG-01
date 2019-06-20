use std;

use crate::memory_bus::{VRAM_BEGIN, VRAM_SIZE};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Color {
    White = 255,
    LightGray = 192,
    DarkGray = 96,
    Black = 0,
}

impl std::convert::From<u8> for Color {
    fn from(n: u8) -> Self {
        match n {
            0 => Color::White,
            1 => Color::LightGray,
            2 => Color::DarkGray,
            3 => Color::Black,
            _ => panic!("Cannot covert {} to color", n),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BackgroundColors(Color, Color, Color, Color);

impl BackgroundColors {
    fn new() -> BackgroundColors {
        BackgroundColors(
            Color::White,
            Color::LightGray,
            Color::DarkGray,
            Color::Black,
        )
    }
}

impl std::convert::From<u8> for BackgroundColors {
    fn from(value: u8) -> Self {
        BackgroundColors(
            (value & 0b11).into(),
            ((value >> 2) & 0b11).into(),
            ((value >> 4) & 0b11).into(),
            (value >> 6).into(),
        )
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TileMap {
    X9800,
    X9C00,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BackgroundAndWindowDataSelect {
    X8000,
    X8800,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ObjectSize {
    OS8X8,
    OS8X16,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Mode {
    HorizontalBlank,
    VerticalBlank,
    OAMAccess,
    VRAMAccess,
}
impl std::convert::From<Mode> for u8 {
    fn from(value: Mode) -> Self {
        match value {
            Mode::HorizontalBlank => 0,
            Mode::VerticalBlank => 1,
            Mode::OAMAccess => 2,
            Mode::VRAMAccess => 3,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TilePixelValue {
    Zero,
    One,
    Two,
    Three,
}

type Tile = [[TilePixelValue; 8]; 8];
#[inline(always)]
fn empty_tile() -> Tile {
    [[TilePixelValue::Zero; 8]; 8]
}

#[derive(Eq, PartialEq)]
pub enum InterruptRequest {
    None,
    VBlank,
    LCDStat,
    Both,
}

impl InterruptRequest {
    fn add(&mut self, other: InterruptRequest) {
        match self {
            InterruptRequest::None => *self = other,
            InterruptRequest::VBlank if other == InterruptRequest::LCDStat => {
                *self = InterruptRequest::Both
            }
            InterruptRequest::LCDStat if other == InterruptRequest::VBlank => {
                *self = InterruptRequest::Both
            }
            _ => {}
        };
    }
}

const SCREEN_WIDTH: usize = 160;
const SCREEN_HEIGHT: usize = 144;
pub struct GPU {
    pub canvas_buffer: [u8; SCREEN_WIDTH * SCREEN_HEIGHT * 4],
    pub tile_set: [Tile; 384],
    pub vram: [u8; VRAM_SIZE],
    pub background_colors: BackgroundColors,
    pub viewport_x_offset: u8,
    pub viewport_y_offset: u8,
    pub lcd_display_enabled: bool,
    pub window_display_enabled: bool,
    pub background_display_enabled: bool,
    pub object_display_enabled: bool,
    pub line_equals_line_check_interrupt_enabled: bool,
    pub oam_interrupt_enabled: bool,
    pub vblank_interrupt_enabled: bool,
    pub hblank_interrupt_enabled: bool,
    pub line_check: u8,
    pub line_equals_line_check: bool,
    pub window_tile_map: TileMap,
    pub background_tile_map: TileMap,
    pub background_and_window_data_select: BackgroundAndWindowDataSelect,
    pub object_size: ObjectSize,
    pub line: u8,
    pub mode: Mode,
    cycles: u16,
}

impl GPU {
    pub fn new() -> GPU {
        GPU {
            canvas_buffer: [0; SCREEN_WIDTH * SCREEN_HEIGHT * 4],
            tile_set: [empty_tile(); 384],
            vram: [0; VRAM_SIZE],
            background_colors: BackgroundColors::new(),
            viewport_x_offset: 0,
            viewport_y_offset: 0,
            lcd_display_enabled: false,
            window_display_enabled: false,
            background_display_enabled: false,
            object_display_enabled: false,
            line_equals_line_check_interrupt_enabled: false,
            oam_interrupt_enabled: false,
            vblank_interrupt_enabled: false,
            hblank_interrupt_enabled: false,
            line_check: 0,
            line_equals_line_check: false,
            window_tile_map: TileMap::X9800,
            background_tile_map: TileMap::X9800,
            background_and_window_data_select: BackgroundAndWindowDataSelect::X8800,
            object_size: ObjectSize::OS8X8,
            line: 0,
            cycles: 0,
            mode: Mode::HorizontalBlank,
        }
    }

    pub fn write_vram(&mut self, index: usize, value: u8) {
        self.vram[index] = value;
        if index >= 0x1800 {
            return;
        }

        // Tiles rows are encoded in two bytes with the first byte always
        // on an even address. Bitwise ANDing the address with 0xffe
        // gives us the address of the first byte.
        // For example: `12 & 0xFFFE == 12` and `13 & 0xFFFE == 12`
        let normalized_index = index & 0xFFFE;

        // First we need to get the two bytes that encode the tile row.
        let byte1 = self.vram[normalized_index];
        let byte2 = self.vram[normalized_index + 1];

        // A tiles is 8 rows tall. Since each row is encoded with two bytes a tile
        // is therefore 16 bytes in total.
        let tile_index = index / 16;
        // Every two bytes is a new row
        let row_index = (index % 16) / 2;

        // Now we're going to loop 8 times to get the 8 pixels that make up a given row.
        for pixel_index in 0..8 {
            // To determine a pixel's value we must first find the corresponding bit that encodes
            // that pixels value:
            // 1111_1111
            // 0123 4567
            //
            // As you can see the bit that corresponds to the nth pixel is the bit in the nth
            // position *from the left*. Bits are normally indexed from the right.
            //
            // To find the first pixel (a.k.a pixel 0) we find the left most bit (a.k.a bit 7). For
            // the second pixel (a.k.a pixel 1) we first the second most left bit (a.k.a bit 6) and
            // so on.
            //
            // We then create a mask with a 1 at that position and 0s everywhere else.
            //
            // Bitwise ANDing this mask with our bytes will leave that particular bit with its
            // original value and every other bit with a 0.
            let mask = 1 << (7 - pixel_index);
            let lsb = byte1 & mask;
            let msb = byte2 & mask;

            // If the masked values are not 0 the masked bit must be 1. If they are 0, the masked
            // bit must be 0.
            //
            // Finally we can tell which of the four tile values the pixel is. For example, if the least
            // significant byte's bit is 1 and the most significant byte's bit is also 1, then we
            // have tile value `Three`.
            let value = match (lsb != 0, msb != 0) {
                (true, true) => TilePixelValue::Three,
                (false, true) => TilePixelValue::Two,
                (true, false) => TilePixelValue::One,
                (false, false) => TilePixelValue::Zero,
            };

            self.tile_set[tile_index][row_index][pixel_index] = value;
        }
    }

    pub fn step(&mut self, cycles: u8) -> InterruptRequest {
        let mut request = InterruptRequest::None;
        if !self.lcd_display_enabled {
            return request;
        }
        self.cycles += cycles as u16;

        let mode = self.mode;
        match mode {
            Mode::HorizontalBlank => {
                if self.cycles >= 200 {
                    self.cycles = self.cycles % 200;
                    self.line += 1;

                    if self.line >= 144 {
                        self.mode = Mode::VerticalBlank;
                        request.add(InterruptRequest::VBlank);
                        if self.vblank_interrupt_enabled {
                            request.add(InterruptRequest::LCDStat)
                        }
                    } else {
                        self.mode = Mode::OAMAccess;
                        if self.oam_interrupt_enabled {
                            request.add(InterruptRequest::LCDStat)
                        }
                    }
                    self.set_equal_lines_check(&mut request);
                }
            }
            Mode::VerticalBlank => {
                if self.cycles >= 456 {
                    self.cycles = self.cycles % 456;
                    self.line += 1;
                    if self.line == 154 {
                        self.mode = Mode::OAMAccess;
                        self.line = 0;
                        if self.oam_interrupt_enabled {
                            request.add(InterruptRequest::LCDStat)
                        }
                    }
                    self.set_equal_lines_check(&mut request);
                }
            }
            Mode::OAMAccess => {
                if self.cycles >= 80 {
                    self.cycles = self.cycles % 80;
                    self.mode = Mode::VRAMAccess;
                }
            }
            Mode::VRAMAccess => {
                if self.cycles >= 172 {
                    self.cycles = self.cycles % 172;
                    if self.hblank_interrupt_enabled {
                        request.add(InterruptRequest::LCDStat)
                    }
                    self.mode = Mode::HorizontalBlank;
                    self.render_scan_line()
                }
            }
        }
        request
    }


    fn set_equal_lines_check(&mut self, request: &mut InterruptRequest) {
        let line_equals_line_check = self.line == self.line_check;
        if line_equals_line_check && self.line_equals_line_check_interrupt_enabled {
            request.add(InterruptRequest::LCDStat);
        }
        self.line_equals_line_check = line_equals_line_check;
    }

    pub fn background_as_buffer(&self, outline_tiles: bool, show_viewport: bool) -> Vec<u8> {
        if self.background_tile_map != TileMap::X9800 {
            panic!("We only support tilemap at 0x9800 for now");
        }

        let width_in_tiles = 32;
        let height_in_tiles = 32;

        let tile_width_in_pixels = 8;
        let tile_height_in_pixels = 8;

        let values_per_pixel = 4;

        let row_width_in_canvas_values = tile_width_in_pixels * width_in_tiles * values_per_pixel;

        let data_length = width_in_tiles
            * height_in_tiles
            * tile_height_in_pixels
            * tile_width_in_pixels
            * values_per_pixel;
        let mut data = vec![0; data_length];

        let tiles = self
            .background_1()
            .iter()
            .map(|byte| self.tile_set[*byte as usize]);

        for (tile_index, tile) in tiles.enumerate() {
            let tile_row = tile_index / height_in_tiles;
            let tile_column = tile_index % width_in_tiles;
            let final_tile_row = tile_row == height_in_tiles - 1;
            let final_tile_column = tile_column == width_in_tiles - 1;

            for (row_index, row) in tile.iter().enumerate() {
                let pixel_row_index = (tile_row * tile_height_in_pixels) + row_index;
                let beginning_of_canvas_row = pixel_row_index * row_width_in_canvas_values;
                let beginning_of_column = tile_column * tile_width_in_pixels;
                let final_pixel_row = final_tile_row && row_index == 7;
                let mut index = beginning_of_canvas_row + (beginning_of_column * values_per_pixel);

                for (pixel_index, pixel) in row.iter().enumerate() {
                    let pixel_column_index = beginning_of_column + pixel_index;
                    let viewport_x_offset = self.viewport_x_offset as usize;
                    let viewport_y_offset = self.viewport_y_offset as usize;
                    let (screen_border_right, did_overflow_x) =
                        self.viewport_x_offset.overflowing_add(SCREEN_WIDTH as u8);
                    let (screen_border_bottom, did_overflow_y) =
                        self.viewport_y_offset.overflowing_add(SCREEN_HEIGHT as u8);
                    let is_inside_screen_horizontally = if did_overflow_x {
                        pixel_column_index < (screen_border_right as usize)
                            || pixel_column_index > viewport_x_offset
                    } else {
                        pixel_column_index < (screen_border_right as usize)
                            && pixel_column_index > viewport_x_offset
                    };
                    let is_on_screen_horizontal_edge = viewport_y_offset == pixel_row_index
                        || pixel_row_index == (screen_border_bottom as usize);
                    let on_screen_border_x =
                        is_inside_screen_horizontally && is_on_screen_horizontal_edge;

                    let is_inside_screen_vertically = if did_overflow_y {
                        pixel_row_index < (screen_border_bottom as usize)
                            || pixel_row_index > viewport_y_offset
                    } else {
                        pixel_row_index < (screen_border_bottom as usize)
                            && pixel_row_index > viewport_y_offset
                    };
                    let is_on_screen_vertical_edge = viewport_x_offset == pixel_column_index
                        || pixel_column_index == (screen_border_right as usize);
                    let on_screen_border_y =
                        is_inside_screen_vertically && is_on_screen_vertical_edge;

                    let on_tile_border_x = pixel_row_index % 8 == 0;
                    let on_tile_border_y = pixel_column_index % 8 == 0;
                    let final_pixel_column = final_tile_column && pixel_index == 7;

                    if show_viewport && (on_screen_border_x || on_screen_border_y) {
                        data[index] = 255;
                        data[index + 1] = 0;
                        data[index + 2] = 0;
                    } else if outline_tiles
                        && (on_tile_border_x
                            || on_tile_border_y
                            || final_pixel_row
                            || final_pixel_column)
                    {
                        data[index] = 0;
                        data[index + 1] = 0;
                        data[index + 2] = 255;
                    } else {
                        let color = self.tile_value_to_background_color(pixel);
                        data[index] = color as u8;
                        data[index + 1] = color as u8;
                        data[index + 2] = color as u8;
                    }
                    data[index + 3] = 255;

                    index = index + values_per_pixel;
                }
            }
        }

        data
    }

    pub fn tile_set_as_buffer(&self, outline_tiles: bool) -> Vec<u8> {
        let values_per_pixel = 4;
        let tile_width = 8;
        let tile_height = 8;

        let width_in_tiles = 24;
        let height_in_tiles = self.tile_set.len() / width_in_tiles;

        let row_width = tile_width * width_in_tiles * values_per_pixel;
        let mut data =
            vec![0; width_in_tiles * height_in_tiles * tile_height * tile_width * values_per_pixel];

        for (tile_index, tile) in self.tile_set.iter().enumerate() {
            let tile_row = tile_index / width_in_tiles;
            let tile_column = tile_index % width_in_tiles;
            let final_tile_row = tile_row == height_in_tiles - 1;
            let final_tile_column = tile_column == width_in_tiles - 1;

            for (row_index, row) in tile.iter().enumerate() {
                let pixel_row_index = (tile_row * tile_height) + row_index;
                let beginning_of_canvas_row = pixel_row_index * row_width;
                let on_tile_row_border = pixel_row_index % 8 == 0;
                let beginning_of_column = tile_column * tile_width;
                let final_pixel_row = final_tile_row && row_index == 7;
                let mut index = beginning_of_canvas_row + (beginning_of_column * values_per_pixel);

                for (pixel_index, pixel) in row.iter().enumerate() {
                    let on_tile_column_border = pixel_index == 0;
                    let final_pixel_column = final_tile_column && pixel_index == 7;
                    if outline_tiles
                        && (on_tile_row_border
                            || on_tile_column_border
                            || final_pixel_row
                            || final_pixel_column)
                    {
                        data[index] = 0;
                        data[index + 1] = 0;
                        data[index + 2] = 255;
                    } else {
                        let color = self.tile_value_to_background_color(pixel);
                        data[index] = color as u8;
                        data[index + 1] = color as u8;
                        data[index + 2] = color as u8;
                    }
                    data[index + 3] = 255;
                    index = index + values_per_pixel;
                }
            }
        }

        data
    }

    // Get a specific tile at the specified coordinates within the entire background space
    pub fn get_tile_buffer_at(&self, pixel_x: usize, pixel_y: usize) -> [[u8; 8]; 8] {
        let tile_x = pixel_x / 8;
        let tile_y = pixel_y / 8;

        let index = (tile_y * 32) + tile_x;
        let mut result = [[0u8; 8]; 8];
        let byte = self.background_1().iter().nth(index).unwrap();
        let tile = self.tile_set[*byte as usize];
        for (row_index, row) in tile.iter().enumerate() {
            for (pixel_index, pixel) in row.iter().enumerate() {
                result[row_index][pixel_index] = self.tile_value_to_background_color(pixel) as u8;
            }
        }

        result
    }

    fn background_1(&self) -> &[u8] {
        &self.vram[0x1800..0x1C00]
    }

    fn render_scan_line(&mut self) {
        if self.background_display_enabled {
            // The x index of the current tile
            let mut tile_x_index = self.viewport_x_offset / 8;
            // The current scan line's y-offset in the entire background space is a combination
            // of both the line inside the view port we're currently on and the amount of the view port is scrolled
            let tile_y_index = self.line.wrapping_add(self.viewport_y_offset);
            // The current tile we're on is equal to the total y offset broken up into 8 pixel chunks
            // and multipled by the width of the entire background (i.e. 32 tiles)
            let tile_offset = (tile_y_index as u16 / 8) * 32u16;

            // Where is our tile map defined?
            let background_tile_map = if self.background_tile_map == TileMap::X9800 {
                0x9800
            } else {
                0x9C00
            };
            // Munge this so that the beginning of VRAM is index 0
            let tile_map_begin = background_tile_map - VRAM_BEGIN;
            // Where we are in the tile map is the beginning of the tile map
            // plus the current tile's offset
            let tile_map_offset = tile_map_begin + tile_offset as usize;

            // When line and scrollY are zero we just start at the top of the tile
            // If they're non-zero we must index into the tile cycling through 0 - 7
            let row_y_offset = tile_y_index % 8;
            let mut pixel_x_index = self.viewport_x_offset % 8;

            if self.background_and_window_data_select == BackgroundAndWindowDataSelect::X8800 {
                panic!("TODO: support 0x8800 background and window data select");
            }

            let mut canvas_buffer_offset = self.line as usize * SCREEN_WIDTH * 4;
            // Start at the beginning of the line and go pixel by pixel
            for _ in 0..SCREEN_WIDTH {
                // Grab the tile index specified in the tile map
                let tile_index = self.vram[tile_map_offset + tile_x_index as usize];

                let tile_value = self.tile_set[tile_index as usize][row_y_offset as usize]
                    [pixel_x_index as usize];
                let color = self.tile_value_to_background_color(&tile_value);

                self.canvas_buffer[canvas_buffer_offset] = color as u8;
                self.canvas_buffer[canvas_buffer_offset + 1] = color as u8;
                self.canvas_buffer[canvas_buffer_offset + 2] = color as u8;
                self.canvas_buffer[canvas_buffer_offset + 3] = 255;
                canvas_buffer_offset += 4;
                // Loop through the 8 pixels within the tile
                pixel_x_index = (pixel_x_index + 1) % 8;

                // Check if we've fully looped through the tile
                if pixel_x_index == 0 {
                    // Now increase the tile x_offset by 1
                    tile_x_index = tile_x_index + 1;
                }
                if self.background_and_window_data_select == BackgroundAndWindowDataSelect::X8800 {
                    panic!("TODO: support 0x8800 background and window data select");
                }
            }
        }
    }

    fn tile_value_to_background_color(&self, tile_value: &TilePixelValue) -> Color {
        match tile_value {
            TilePixelValue::Zero => self.background_colors.0,
            TilePixelValue::One => self.background_colors.1,
            TilePixelValue::Two => self.background_colors.2,
            TilePixelValue::Three => self.background_colors.3,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tile_set_buffer() {
        let gpu = GPU::new();
        gpu.tile_set_as_buffer(false);
    }
}
