use crate::memory::mmu::Mmu;
use crate::gpu::fifo::Fifo;

#[derive(Debug)]
enum Mode {
    HBlank,
    VBlank,
    OAM,
    PixelFetcher,
}

pub struct Ppu {
    mode: Mode,
    dots: u32,
    x: u8,
    clock: bool,
    backgorund_fifo: Fifo,
    _oam_fifo: Fifo,
}

impl Ppu {
    pub fn new() -> Self {
        Self {
            mode: Mode::HBlank,
            dots: 0,
            x: 0,
            clock: false,
            backgorund_fifo: Fifo::new(),
            _oam_fifo: Fifo::new(),
        }
    }

    pub fn pixel_transfer(&mut self, memory: &mut Mmu) -> Option<([u8; 4])> {
        self.backgorund_fifo.cycle(memory)
    }

    pub fn cycle(&mut self, frame: &mut [u8], memory: &mut Mmu) {
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
                        memory.set_interrupt_flag(0);
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
                    Some(color) => {
                        draw_color_at_pos(color, self.x.into(), ly.into(), frame);
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
        memory.set_stat_mode_flag( 
            match self.mode {
                Mode::HBlank => 0,
                Mode::VBlank => 1,
                Mode::OAM => 2,
                Mode::PixelFetcher => 3,
            });
        self.dots += 1;
    }
}

pub fn draw_color_at_pos(color: [u8;4], x: usize, y: usize, frame: &mut [u8]) {
    let index = (y * 160 + x) * 4;
    frame[index] = color[0];        // R
    frame[index + 1] = color[1];    // G
    frame[index + 2] = color[2];    // B
    frame[index + 3] = color[3];    // A
}