use gb_core::core::Core;
use wasm_bindgen::prelude::*;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}
pub(crate) use log;

const WIDTH: usize = 160;
const HEIGHT: usize = 144;
const BUFFER_SIZE: usize = WIDTH * HEIGHT * 4;

#[wasm_bindgen]
pub struct Game {
    buffer: [u8; 92160],
    gb: Core,
}

#[wasm_bindgen]
impl Game {
    pub fn new() -> Game {
        console_error_panic_hook::set_once();

        Game {
            buffer: [150; BUFFER_SIZE],
            gb: Core::default(),
        }
    }

    pub fn read_bios(&mut self, data: &[u8], len: usize) -> bool {
        return self.gb.load_bios(data, len);
    }

    pub fn read_rom(&mut self, data: &[u8], len: usize) -> bool {
        return self.gb.load_rom(data, len);
    }

    pub fn buffer_ptr(&self) -> *const u8 {
        return self.buffer.as_ptr();
    }

    // 16 ms
    pub fn tick(&mut self, frames: u32) {
        for _ in 0..17556 * frames {
            self.gb.tick();
        }

        self.buffer = self.gb.get_screen_buffer();
    }
}
