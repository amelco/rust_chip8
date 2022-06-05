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

    loop {
        chip8.run_instruction();
    }

}
