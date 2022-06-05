pub struct Keyboard {

}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard {}
    }

    // Todo: implemet proper key handling
    pub fn key_pressed(&self, key_code: u8) -> bool {
        true
    }

}