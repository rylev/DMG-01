extern crate piston_window;
extern crate image;

pub mod cpu;
mod gpu;
mod memory_bus;

use std::io::Read;
use piston_window::*;

const ENLARGEMENT_FACTOR: usize = 1;
const WINDOW_DIMENSIONS: [u32; 2] = [(160 * ENLARGEMENT_FACTOR) as u32,
                                     (144 * ENLARGEMENT_FACTOR) as u32];
fn main() {
    let mut boot = std::fs::File::open("./roms/boot.gb").expect("File not there");
    let mut boot_buffer = Vec::new();
    boot.read_to_end(&mut boot_buffer).expect("Could not read file");

    let mut game = std::fs::File::open("./roms/tetris.gb").expect("File not there");
    let mut game_buffer = Vec::new();
    game.read_to_end(&mut game_buffer).expect("Could not read file");
    let mut cpu = cpu::CPU::new(Some(boot_buffer), game_buffer);
    let mut window: PistonWindow = WindowSettings::new("DMG-01", WINDOW_DIMENSIONS)
                                   .exit_on_esc(true)
                                   .build()
                                   .unwrap();

    let mut canvas = image::ImageBuffer::new(WINDOW_DIMENSIONS[0], WINDOW_DIMENSIONS[1]);
    let mut texture: G2dTexture = Texture::from_image(
        &mut window.factory,
        &canvas,
        &TextureSettings::new()).unwrap();

    while let Some(e) = window.next() {
        if let Some(_) = e.render_args() {
            texture.update(&mut window.encoder, &canvas).unwrap();
             window.draw_2d(&e, |c, g| {
                clear([1.0; 4], g);
                image(&texture, c.transform, g);
            });
        }
        if let Some(u) = e.update_args() {
            let mut cycles_elapsed = 0usize;
            let number_of_cycles = (4190000.0 * u.dt) as usize;
            loop {
                cycles_elapsed += cpu.step() as usize;
                if cycles_elapsed >= number_of_cycles {
                    break
                }
            }
            for (i, pixel) in cpu.bus.gpu.canvas_buffer.chunks(4).enumerate() {
                let x = i % 160;
                let y = i / 160;
                canvas.put_pixel(x as u32, y as u32, image::Rgba([pixel[0], pixel[1], pixel[2], pixel[3]]));
            }
        }
    }
}
