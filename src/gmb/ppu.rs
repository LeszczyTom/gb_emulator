use std::collections::VecDeque;

use super::memory::MEMORY;

#[derive(Debug)]
enum Mode {
    HBlank,
    VBlank,
    OAM,
    PixelFetcher,
}

enum Step {
    ReadTileId,
    ReadData0,
    ReadData1,
    Idle,
}

struct Fetcher {
    step: Step,
    tile_id: u8,
    data0: u8,
    data1: u8,
    x: usize,
}

impl Fetcher {
    fn new() -> Fetcher {
        Fetcher {
            step: Step::ReadTileId,
            tile_id: 0,
            data0: 0,
            data1: 0,
            x: 0,
        }
    }

    fn cycle(&mut self, memory: &mut MEMORY, fifo: &mut VecDeque<(u8, u8)>) {
        let scx = memory.get_scx();
        let scy = memory.get_scy();
        let x = ((scx / 8) + self.x as u8) & 0x1F;
        let y = (memory.get_ly() + scy) & 255;
        let row: u16 = y as u16 / 8;
        let col = x as u16 ;

        let offset = 0x8000 + self.tile_id as u16 * 16;
        let addr = offset + (y as u16 % 8) * 2;
        match self.step {
            Step::Idle => {
                if fifo.len() <= 7 {
                    self.step = Step::ReadTileId;
                    for i in 0..8 {
                        let mut data = [0; 2];
                        data[0] = self.data0 >> (7 - i) & 1;
                        data[1] = self.data1 >> (7 - i) & 1;
                        fifo.push_back(((data[1] << 1) | data[0], 0));
                    }
                }
            }
            Step::ReadTileId => {
                self.tile_id = memory.read_byte(0x8000 + 0x1800 + (row * 32 + col)); // 0x1800 or 0x1C00
                self.step = Step::ReadData0;
            }
            Step::ReadData0 => {
                self.data0 = memory.read_byte(addr);
                self.step = Step::ReadData1;
            }
            Step::ReadData1 => {
                self.data1 = memory.read_byte(addr + 1);
                self.x += 1;
                self.step = Step::Idle;
            }
        }
    }
}

struct FIFO {
    fetcher: Fetcher,
    data: VecDeque<(u8, u8)>, // (color, pallette) tuples. color: [00, 01, 10, 11] - pallette: [0, 1, 2, 3]
    clock: bool,
}

impl FIFO {
    fn new() -> FIFO {
        FIFO {
            fetcher: Fetcher::new(),
            data:VecDeque::new(),
            clock: false,
        }
    }

    pub fn clear(&mut self) {
        self.data.clear();
        self.fetcher = Fetcher::new();
        self.clock = false;
    }

    pub fn cycle(&mut self, memory: &mut MEMORY) -> Option<(u8, u8)> {
        if self.clock { // 1Mhz
            self.clock = false;
            return self.push()
        }
        self.clock = true;
        
        self.fetcher.cycle(memory, &mut self.data);
        self.push()
    }

    fn push(&mut self) -> Option<(u8, u8)> {
        if self.data.len() <= 7 {
            return None;
        }

        self.data.pop_front()
    }
}

pub struct PPU {
    mode: Mode,
    dots: u32,
    x: u8,
    clock: bool,
    backgorund_fifo: FIFO,
    oam_fifo: FIFO,
}

impl PPU {
    pub fn new() -> Self {
        Self {
            mode: Mode::HBlank,
            dots: 0,
            x: 0,
            clock: false,
            backgorund_fifo: FIFO::new(),
            oam_fifo: FIFO::new(),
        }
    }

    pub fn pixel_transfer(&mut self, memory: &mut MEMORY) -> Option<(u8, u8)> {
        self.backgorund_fifo.cycle(memory)
    }

    pub fn cycle(&mut self, frame: &mut [u8], memory: &mut MEMORY) {
        if self.clock { // 2MHz
            self.clock = false;
        }
        self.clock = true;

        match self.mode {
            Mode::HBlank => { // Wait 85 to 208 dots 
                if self.dots == 456 { // 456 dots per line
                    self.dots = 0;
                    let mut ly = memory.get_ly();
                    ly += 1;
                    memory.set_ly(ly);
                    if ly == 144 { // At line 144, the PPU enters V-Blank
                        self.mode = Mode::VBlank;
                    } else {
                        self.mode = Mode::OAM;
                    }
                }
            },
            Mode::VBlank => { // Wait 4560 dots (10 lines)
                if self.dots == 456 { // 456 dots per line 
                    self.dots = 0;
                    let mut ly = memory.get_ly();
                    ly += 1;
                    memory.set_ly(ly);
                    if ly == 153 {// At line 154, the PPU returns to OAM mode
                        memory.set_ly(0);
                        self.mode = Mode::OAM;
                    }
                }
            },
            Mode::OAM => {
                if self.dots == 80 { // Searching OAM takes 80 dots
                    self.dots = 0;
                    self.mode = Mode::PixelFetcher;
                }
            },
            Mode::PixelFetcher => { // Reading OAM and VRAM takes 168 to 291 dots
                // Fetch pixel to the pixel FIFIO
                // Draw pixel from FIFO to the screen
                let ly = memory.get_ly();
                let data = self.pixel_transfer(memory);
                match data {
                    None => (),
                    Some(i) => {
                        let (color, pallette) = i;
                        let a = match color {
                            0 => [255, 255, 255, 255],
                            1 => [0, 0, 0, 255],
                            2 => [0, 0, 0, 255],
                            3 => [0, 0, 0, 255],
                            _ => panic!("Invalid color"),
                        };
                        draw_color_at_pos(a, self.x.into(), ly.into(), frame);
                        self.x += 1;
                    }
                }
                
                if self.x == 160 { // When the PPU reaches the end of a scanline, it enters H-Blank
                    self.x = 0;
                    self.backgorund_fifo.clear();
                    self.mode = Mode::HBlank;
                }
            }
        }
        self.dots += 1;
    }
}

pub fn draw_color_at_pos(color: [u8;4], x: usize, y: usize, frame: &mut [u8]) {
    let index = (y * crate::WIDTH as usize + x) * 4;
    frame[index] = color[0];        // R
    frame[index + 1] = color[1];    // G
    frame[index + 2] = color[2];    // B
    frame[index + 3] = color[3];    // A
}