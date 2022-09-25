pub struct PPU {
    pixels: Vec<u8>
}

impl PPU {
    pub fn new() -> Self {
        let mut pixels = Vec::new();
        for i in 0..23_141 {
            pixels.push(0x5e);
            pixels.push(0x48);
            pixels.push(0xe8);
            pixels.push(0xff);
        }
        Self {
            pixels,
        }
    }

    pub fn set_pixel_at(&mut self, x: usize, y: usize, color: [u8; 4]) {        
        self.pixels[y * WIDTH + x] = color[0];
        self.pixels[y * WIDTH + x + 1] = color[1];
        self.pixels[y * WIDTH + x + 2] = color[2];
        self.pixels[y * WIDTH + x + 3] = color[3];
    }

    pub fn get_pixel_at(&self, x: usize, y: usize) -> [u8; 4] {
        [
            self.pixels[y * WIDTH + x],
            self.pixels[y * WIDTH + x + 1],
            self.pixels[y * WIDTH + x + 2],
            self.pixels[y * WIDTH + x + 3],
        ]
    }

    pub fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = i % WIDTH;
            let y = i / WIDTH;

            let rgba = self.get_pixel_at(x, y);
            //let rgba = [0x5e, 0x48, 0xe8, 0xff];
            pixel.copy_from_slice(&rgba);
        }
    }
}

const WIDTH: usize = 160;
const HEIGHT: usize = 144;