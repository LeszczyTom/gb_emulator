use crate::gpu::lcdc;
use crate::memory::mmu::Mmu;

const WIDTH: usize = 160;

#[derive(PartialEq)]
enum Mode {
    HBlank,
    VBlank,
    OAM,
    DrawingPixel,
}

pub struct Ppu {
    dots: u16,
    mode: Mode,
}

impl Ppu {
    pub fn new() -> Self {
        Self {
            dots: 0,
            mode: Mode::HBlank,
        }
    }

    fn set_mode(&mut self, mode: Mode, mmu: &mut Mmu) {
        let mode_flag_bit = match &mode {
            Mode::HBlank => 0,
            Mode::VBlank => 1,
            Mode::OAM => 2,
            Mode::DrawingPixel => 3,
        };
        let mode_flag = mmu.read_byte(0xFF41) & 0xFC; // Unset bit 0 and 1
        mmu.write_byte(0xFF41, mode_flag | mode_flag_bit); // Set correct bit
        self.mode = mode;

        match self.mode {
            Mode::DrawingPixel => return,
            Mode::HBlank => {
                if (mmu.read_byte(0xFF41) >> 3) & 1 == 1 {
                    mmu.set_interrupt_flag(1);
                }
            }
            Mode::VBlank => {
                if (mmu.read_byte(0xFF41) >> 4) & 1 == 1 {
                    mmu.set_interrupt_flag(1);
                }
            }
            Mode::OAM => {
                if (mmu.read_byte(0xFF41) >> 5) & 1 == 1 {
                    mmu.set_interrupt_flag(1);
                }
            }
        }
    }

    pub fn update(&mut self, frame: &mut [u8], mmu: &mut Mmu) {
        // Update the PPU if the LCD is enabled
        if !lcdc::is_ldc_and_ppu_enable(mmu) {
            reset_ly(mmu); // Reset scanline
            self.dots = 0;
            mmu.write_byte(0xFF41, mmu.read_byte(0xFF41) & 0xFC | 1); // Set to mode 1
            return;
        }

        match self.mode {
            Mode::HBlank => {
                if self.dots == 376 {
                    // MODE 3 + MODE 0 - MODE 2 = 456 - 80
                    self.dots = 0;

                    let ly = increment_ly(mmu);
                    if ly == 144 {
                        // At line 144, the PPU enters V-Blank and requests a V-Blank interrupt
                        self.set_mode(Mode::VBlank, mmu);
                        mmu.set_interrupt_flag(0);
                    } else {
                        self.set_mode(Mode::OAM, mmu)
                    }
                }
            }
            Mode::VBlank => {
                if self.dots == 456 {
                    // 456 dots per line
                    self.dots = 0;

                    let ly = increment_ly(mmu);
                    if ly == 154 {
                        // At line 154, the PPU returns to OAM mode
                        reset_ly(mmu);
                        self.set_mode(Mode::OAM, mmu)
                    }
                }
            }
            Mode::OAM => {
                if self.dots == 80 {
                    // Searching OAM takes 80 dots
                    self.dots = 0;
                    self.set_mode(Mode::DrawingPixel, mmu)
                }
            }
            Mode::DrawingPixel => {
                if self.dots == 172 {
                    self.dots = 0;
                    draw_scanline(frame, mmu);
                    self.set_mode(Mode::HBlank, mmu)
                }
            }
        }

        check_coincidence_flag(mmu);

        self.dots += 1;
    }
}

fn check_coincidence_flag(mmu: &mut Mmu) {
    let ly = mmu.get_ly();
    let lyc = mmu.read_byte(0xFF45);
    let stat = mmu.read_byte(0xFF41);

    if ly == lyc {
        // If LY == LYC, set the coincidence flag in STAT
        mmu.write_byte(0xFF41, stat | 0x4);

        // If the coincidence interrupt is enabled, request STAT interrupt
        if stat & 0x40 == 0x40 {
            mmu.set_interrupt_flag(1);
        }
    } else {
        mmu.write_byte(0xFF41, stat & 0xFB);
    }
}

fn reset_ly(mmu: &mut Mmu) {
    mmu.write_byte(0xFF44, 0);
}

fn increment_ly(mmu: &mut Mmu) -> u8 {
    mmu.increment_ly();
    mmu.get_ly()
}

fn draw_scanline(frame: &mut [u8], mmu: &mut Mmu) {
    if lcdc::get_lcdc_n(0, mmu) {
        draw_bg(frame, mmu);
    }

    if lcdc::get_lcdc_n(5, mmu) {
        draw_window(frame, mmu);
    }

    // if lcdc::get_lcdc_n(1, mmu) {
    //     draw_sprites(frame, mmu);
    // }
}

fn draw_bg(frame: &mut [u8], mmu: &mut Mmu) {
    let scx = mmu.read_byte(0xFF43);
    let scy = mmu.read_byte(0xFF42);

    let tile_data = if lcdc::get_lcdc_n(4, mmu) {
        0x8000
    } else {
        0x8800
    };
    let background = if lcdc::get_lcdc_n(3, mmu) {
        0x9C00
    } else {
        0x9800
    };

    let y = mmu.get_ly() + scy;

    let tile_row = (y / 8) as u16 * 32;
    for x in 0..160 {
        let x_pos = (x as u8).wrapping_add(scx);

        if x_pos > 159 || mmu.get_ly() > 143 {
            continue;
        }

        let tile_col = (x_pos / 8) as u16;
        let tile_addr = background + tile_row + tile_col;
        let tile_num = if tile_data == 0x8000 {
            mmu.read_byte(tile_addr) as u16
        } else {
            mmu.read_byte(tile_addr) as i8 as u16
        };

        let tile_loc = if tile_data == 0x8000 {
            tile_data + tile_num * 16
        } else {
            (tile_data as u32 + (tile_num as u32 + 128) * 16) as u16
        };

        let line = ((y % 8) * 2) as u16;
        let data1 = mmu.read_byte(tile_loc + line);
        let data2 = mmu.read_byte(tile_loc + line + 1);

        let pixel = x_pos % 8;

        let color = ((data1 >> (7 - pixel) & 1) << 1) | ((data2 >> (7 - pixel)) & 1);

        draw_color_at_pos(
            COLORS[mmu.get_background_palette(color)],
            x,
            mmu.get_ly(),
            frame,
        );
    }
}

fn draw_window(frame: &mut [u8], mmu: &mut Mmu) {
    let wx = mmu.read_byte(0xFF4B).wrapping_sub(7);
    let wy = mmu.read_byte(0xFF4A);
    let scx = mmu.read_byte(0xFF43);

    let ly = mmu.get_ly();
    if ly < wy {
        return;
    }

    let tile_data = if lcdc::get_lcdc_n(4, mmu) {
        0x8000
    } else {
        0x8800
    };
    let background = if lcdc::get_lcdc_n(6, mmu) {
        0x9C00
    } else {
        0x9800
    };

    let y = mmu.get_ly() - wy;

    let tile_row = (y / 8) as u16 * 32;
    for x in 0..160 {
        if x < wx {
            continue;
        }

        let x_pos = if x >= wx { x - wx } else { x + scx };

        if x_pos > 159 {
            continue;
        }

        let tile_col = (x_pos / 8) as u16;
        let tile_addr = background + tile_row + tile_col;
        let tile_num = if tile_data == 0x8000 {
            mmu.read_byte(tile_addr) as u16
        } else {
            mmu.read_byte(tile_addr) as i8 as u16
        };

        let tile_loc = if tile_data == 0x8000 {
            tile_data + tile_num * 16
        } else {
            (tile_data as u32 + (tile_num as u32 + 128) * 16) as u16
        };

        let line = ((y % 8) * 2) as u16;
        let data1 = mmu.read_byte(tile_loc + line);
        let data2 = mmu.read_byte(tile_loc + line + 1);

        let pixel = x_pos % 8;

        let color = ((data1 >> (7 - pixel) & 1) << 1) | ((data2 >> (7 - pixel)) & 1);

        draw_color_at_pos(
            COLORS[mmu.get_background_palette(color)],
            x,
            mmu.get_ly(),
            frame,
        );
    }
}

fn draw_sprites(frame: &mut [u8], mmu: &mut Mmu) {
    let object_size = if lcdc::get_lcdc_n(2, mmu) { 8 } else { 16 };
    let object_size = 8;

    mmu.get_oam_slice()
        .chunks(4)
        .filter(|&sprite| {
            let y = match sprite[0].checked_sub(16) {
                Some(y) => y,
                None => return false,
            };
            y > mmu.get_ly() && y <= mmu.get_ly() + object_size
        })
        .for_each(|sprite| {
            let sprite_y = sprite[0] - 16;
            let sprite_x = sprite[1] - 8;
            let bg_over_obj = (sprite[3] >> 7) & 1 == 0;
            let obp0 = (sprite[3] >> 4) & 1 == 0;
            let y_flipped = (sprite[3] >> 6) & 1 == 1;
            let x_flipped = (sprite[3] >> 5) & 1 == 1;

            let tile_index = match object_size {
                8 => sprite[2] as u16,
                16 => (sprite[2] & 254) as u16,
                _ => unreachable!(),
            };
            let addr: u16 = 0x8000 + tile_index * 16;

            let current_object_y = sprite_y - mmu.get_ly();
            //let line = current_object_y as u16 * 2;

            let line = if y_flipped {
                (object_size - current_object_y) as u16 * 2
            } else {
                current_object_y as u16 * 2
            };

            let data1 = mmu.read_byte(addr + line);
            let data2 = mmu.read_byte(addr + line + 1);

            for x in 0..8 {
                let color = mmu.get_object_palette(
                    ((data1 >> (7 - x) & 1) << 1) | ((data2 >> (7 - x)) & 1),
                    obp0,
                );

                let index = (mmu.get_ly() as usize * WIDTH + x as usize) * 4;
                let actual_color = [
                    frame[index],
                    frame[index + 1],
                    frame[index + 2],
                    frame[index + 3],
                ];

                if color != 0 {
                    if bg_over_obj || (!bg_over_obj && actual_color == COLORS[0]) {
                        draw_color_at_pos(COLORS[color], sprite_x + x, mmu.get_ly(), frame);
                    }
                }
            }
        });
}

fn draw_color_at_pos(color: [u8; 4], x: u8, y: u8, frame: &mut [u8]) {
    let index = (y as usize * WIDTH + x as usize) * 4;
    frame[index] = color[0]; // R
    frame[index + 1] = color[1]; // G
    frame[index + 2] = color[2]; // B
    frame[index + 3] = color[3]; // A
}

const COLORS: [[u8; 4]; 4] = [
    [0xFF, 0xFF, 0xFF, 0xff],
    [0x55, 0x55, 0x55, 0xff],
    [0xAA, 0xAA, 0xAA, 0xff],
    [0x00, 0x00, 0x00, 0xff],
];
