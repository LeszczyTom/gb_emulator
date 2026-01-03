use crate::ppu::Palette;

pub struct Pixel {
    pub data: u8,
    pub priority: bool,
    pub palette: Palette,
}

impl Pixel {
    pub fn new(data: u8, priority: bool, palette: Palette) -> Self {
        Self {
            data,
            priority,
            palette,
        }
    }
}
