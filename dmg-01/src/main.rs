extern crate minifb;
extern crate clap;
extern crate lib_dmg_01;

use clap::{Arg, App};
use minifb::{WindowOptions, Window, Key};

use std::io::Read;
use std::time::{Instant, Duration};
use std::thread::sleep;

use lib_dmg_01::cpu::{ CPU };

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

    let boot_buffer = matches.value_of("boot rom").map(|path| buffer_from_file(path));
    let game_buffer = matches.value_of("rom").map(|path| buffer_from_file(path)).unwrap();

    let cpu = CPU::new(boot_buffer, game_buffer);
    let window = Window::new("DMG-01", WINDOW_DIMENSIONS[0], WINDOW_DIMENSIONS[1], WindowOptions::default()).unwrap();

    run(cpu, window)
}

const ONE_SECOND_IN_MICROS: usize = 1000000000;
const ONE_SECOND_IN_CYCLES: usize = 4190000;
const ONE_FRAME_IN_CYCLES: usize = 70224;
const NUMBER_OF_PIXELS: usize = 23040;

fn run(mut cpu: CPU, mut window: Window) {
    let mut buffer = [0; NUMBER_OF_PIXELS];
    let mut cycles_elapsed_in_frame = 0usize;
    let mut now = Instant::now();
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let time_delta = now.elapsed().subsec_nanos();
        now = Instant::now();
        let delta = time_delta as f64 / ONE_SECOND_IN_MICROS as f64;
        let cycles_to_run = delta * ONE_SECOND_IN_CYCLES as f64;

        let mut cycles_elapsed = 0;
        while cycles_elapsed <= cycles_to_run as usize{
            cycles_elapsed += cpu.step() as usize;
        }
        cycles_elapsed_in_frame += cycles_elapsed;

        // TODO: Consider updating buffer after every line is rendered.
        if cycles_elapsed_in_frame >= ONE_FRAME_IN_CYCLES {
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

fn buffer_from_file(path: &str) -> Vec<u8> {
    let mut file = std::fs::File::open(path).expect("File not there");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Could not read file");
    buffer
}
