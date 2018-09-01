extern crate minifb;
extern crate image;
extern crate clap;

use clap::{Arg, App};
use minifb::{WindowOptions, Window, Key};

use std::io::Read;
use std::time::{Instant, Duration};
use std::thread::sleep;

mod cpu;
mod gpu;
mod memory_bus;

const ENLARGEMENT_FACTOR: usize = 1;
const WINDOW_DIMENSIONS: [usize; 2] = [(160 * ENLARGEMENT_FACTOR),
                                     (144 * ENLARGEMENT_FACTOR)];
fn main() {
    let matches = App::new("DMG-01")
        .author("Ryan Levick <ryan.levick@gmail.com>")
        .arg(Arg::with_name("boot rom")
            .short("b")
            .value_name("FILE"))
        .arg(Arg::with_name("rom")
             .short("r")
             .required(true)
             .value_name("FILE"))
        .get_matches();
    let boot_buffer = matches.value_of("boot rom").map(|path| {
        let mut boot = std::fs::File::open(path).expect("File not there");
        let mut boot_buffer = Vec::new();
        boot.read_to_end(&mut boot_buffer).expect("Could not read file");
        boot_buffer
    });

    let rom_path = matches.value_of("rom").unwrap();
    let mut game = std::fs::File::open(rom_path).expect("File not there");
    let mut game_buffer = Vec::new();
    game.read_to_end(&mut game_buffer).expect("Could not read file");

    let mut cpu = cpu::CPU::new(boot_buffer, game_buffer);

    let mut window = Window::new("DMG-01", WINDOW_DIMENSIONS[0], WINDOW_DIMENSIONS[1], WindowOptions::default()).unwrap();

    let mut buffer = [0; 23040];
    let mut cycles_elapsed_in_frame = 0usize;
    let mut now = Instant::now();
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let time_delta = now.elapsed().subsec_nanos();
        now = Instant::now();
        let delta = time_delta as f64 / 1000000000.0;
        let cycles_to_run = (4190000.0 * delta) as usize;

        let mut cycles_elapsed = 0;
        while cycles_elapsed <= cycles_to_run {
            cycles_elapsed += cpu.step() as usize;
        }
        cycles_elapsed_in_frame += cycles_elapsed;

        if cycles_elapsed_in_frame >= 70224 {
            for (i, pixel) in cpu.bus.gpu.canvas_buffer.chunks(4).enumerate() {
                buffer[i] = (pixel[3] as u32) << 24 | (pixel[2] as u32) << 16 | (pixel[1] as u32) << 8 | (pixel[0] as u32)
            }
            window.update_with_buffer(&buffer).unwrap();
            cycles_elapsed_in_frame = 0;
        } else {
            sleep(Duration::from_nanos(2))
        }
    }
}
