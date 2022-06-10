extern crate minifb;

use display::Display;
use minifb::{KeyRepeat, Key, WindowOptions, Window};
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

    let mut chip8 = Chip8::new();
    chip8.load_rom(&data);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // read keypress
        let mut chip8_key: Option<u8> = None;
        window.get_keys_pressed(KeyRepeat::No).map(|keys| {
            for t in keys {
                chip8_key = match t {
                    Key::Key1 => Some(0x1),
                    Key::Key2 => Some(0x2),
                    Key::Key3 => Some(0x3),
                    Key::Key4 => Some(0xC),
                    Key::Q => Some(0x4),
                    Key::W => Some(0x5),
                    Key::E => Some(0x6),
                    Key::R => Some(0xD),
                    Key::A => Some(0x7),
                    Key::S => Some(0x8),
                    Key::D => Some(0x9),
                    Key::F => Some(0xE),
                    Key::Z => Some(0xA),
                    Key::X => Some(0x0),
                    Key::C => Some(0xB),
                    Key::V => Some(0xF),
                    _ => None,
                }
            }
        });
        chip8.set_key_pressed(chip8_key);

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
