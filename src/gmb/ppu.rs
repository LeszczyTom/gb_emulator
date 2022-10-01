pub struct PPU {
    pixels: Vec<u8>,
    updated: bool,
}

impl PPU {
    pub fn new() -> Self {
        let mut pixels = Vec::new();
        for _ in 0..WIDTH * HEIGHT + 1 {
            pixels.push(0);
        }
        Self {
            pixels,
            updated: false,
        }
    }

    pub fn set_pixel_at(&mut self, x: usize, y: usize, color: u8) {   
        if y * WIDTH + x > WIDTH * HEIGHT {
            return;
        }

        self.pixels[y * WIDTH + x] = color;
        self.updated = true;
    }

    pub fn set_tile_at(&mut self, x: usize, y: usize, tile: &Vec<u8>) {
        for i in (0..tile.len() - 1).step_by(2) {
            let byte = tile[i];
            let byte1 = tile[i + 1];
            for j in 0..8 {
                let bit = (byte >> (7 - j)) & 1;
                let bit1 = (byte1 >> (7 - j)) & 1;
                let color = (bit1 << 1) | bit;
                self.set_pixel_at(x + j, y + i / 2, color);
            }
        }
    }

    pub fn get_pixel_at(&self, x: usize, y: usize) -> [u8; 4] {
        match self.pixels[y * WIDTH + x] {
            0 => [0xff, 0xff, 0xff, 0xff],
            1 => [0xD3, 0xd3, 0xd3, 0xd3],
            2 => [0xa9, 0xa9, 0xa9, 0xa9],
            _ => [0x00, 0x00, 0x00, 0x00],
        }
    }

    pub fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = i % WIDTH;
            let y = i / WIDTH;

            let rgba = self.get_pixel_at(x, y);
            pixel.copy_from_slice(&rgba);
        }
    }

    pub fn get_data(&self) -> &[u8] {
        &self.pixels
    }

    pub fn reset_updated(&mut self) {
        self.updated = false;
    }

    pub fn is_updated(&self) -> bool {
        self.updated
    }

    pub fn read_tiles(&mut self, vram: Vec<u8>) {    
        let mut tile: Vec<u8> = Vec::new();
        let mut x = 0;
        let mut y = 0;
        for byte in vram {
            tile.push(byte);
            if tile.len() == 16 {
                self.set_tile_at(x, y, &tile);
                x += 8;
                if x >= WIDTH {
                    x = 0;
                    y += 8;
                }
                tile.clear();
            }    
        }
    }
}

const WIDTH: usize = 160;
const HEIGHT: usize = 144;