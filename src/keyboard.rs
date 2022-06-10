use minifb::{Window, KeyRepeat, Key};

pub struct Keyboard {
    key_pressed: Option<u8>
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard {
            key_pressed: None
        }
    }

    // Todo: implemet proper key handling
    pub fn is_key_pressed(&self, key_code: u8) -> bool {
        match self.key_pressed {
            Some(key_code) => true,
            _ => false
        }
    }

    pub fn set_key_pressed(&mut self, key_code: Option<u8>)
    {
        self.key_pressed = key_code
    }

    pub fn get_key_pressed(&self) -> Option<u8> {
        self.key_pressed
    }
}