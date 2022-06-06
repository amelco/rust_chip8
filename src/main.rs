extern crate minifb;

use display::Display;
use minifb::{Key, WindowOptions, Window};
use std::fs::File;
use std::io::Read;
use chip8::Chip8;

mod ram;
mod chip8;
mod cpu;
mod display;
mod keyboard;
mod bus;

fn main() {
    let mut file = File::open("data/INVADERS").unwrap();
    let mut data: Vec<u8> = Vec::new();
    file.read_to_end(&mut data).unwrap_or_default();

    let mut chip8 = Chip8::new();
    chip8.load_rom(&data);

    let WIDTH = 640;
    let HEIGHT = 320;
    // ARGB buffer
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    for i in buffer.iter_mut() {
        *i = 0xffff0000;
    }

    let mut window = Window::new(
        "Rust8 - Chip8 emulator",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });


    while window.is_open() && !window.is_key_down(Key::Escape) {
       // update buffer
       chip8.run_instruction();
       let chip8_buffer = chip8.get_display_buffer();

       // scale up buffer
       for y in 0..HEIGHT {
           for x in 0..WIDTH {
               let index = Display::get_index_from_coords(x/10, y/10);
               let pixel = chip8_buffer[index];
               let color_pixel: u32 = match pixel {
                   0 => 0x0,
                   1 => 0xffffffff,
                   _ => unreachable!()
               };
               buffer[y * WIDTH + x] = color_pixel;
           }
       }

        window.update_with_buffer(&buffer).unwrap();
    }
}
