const WIDTH: usize = 64;
const HEIGHT: usize = 32;

pub struct Display {
    screen: [u8; WIDTH * HEIGHT],
}

impl Display {
    pub fn new() -> Display {
        Display {
            screen: [0; WIDTH * HEIGHT],
        }
    }

    pub fn get_index_from_coords(x: usize, y: usize) -> usize {
        y * WIDTH + x
    }

    pub fn debug_draw_sprite(&mut self, mut byte: u8, x: u8, y: u8) -> bool {
        let mut flipped = false;
        let mut coord_x = x as usize;
        let coord_y = y as usize;

        for _ in 0..8 {
            let index = Display::get_index_from_coords(coord_x, coord_y);
            let bit = (byte & 0b1000_0000) >> 7;
            let prev_value = self.screen[index];
            self.screen[index] ^= bit;

            if prev_value == 1 && self.screen[index] == 0 {
                flipped = true;
            }

            coord_x += 1;
            byte = byte << 1;
        }
        flipped 
    }

    pub fn clear_screen(&mut self) {
        for index in 0..self.screen.len() {
            self.screen[index] = 0;
        }
    }

    pub fn present(&self) {
        for index in 0..self.screen.len() {
            let pixel = self.screen[index];
            
            if index % WIDTH == 0 {
                print!("\n");
            }

            match pixel {
                0 => print!("_"),
                1 => print!("#"),
                _ => unreachable!()
            }
        }
        print!("\n");
    }

    pub fn get_display_buffer(&self) -> &[u8] {
        &self.screen
    }

}