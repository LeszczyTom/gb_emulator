use std::collections::HashMap;

use crate::{
    log,
    mmu::MMU,
    ppu::{Palette, bg_fifo::BGFifo, oam_fifo::OamFifo, object::OamObject},
};

pub struct PPU {
    dots: u32,
    objects: HashMap<u8, OamObject>,
    bg_fifo: BGFifo,
    oam_fifo: OamFifo,
    oam_scanned: bool,
    object_drawn: u8,
    last_drawn_x: u8,
    x: u8,
}

impl Default for PPU {
    fn default() -> Self {
        Self {
            dots: 0,
            objects: HashMap::new(),
            bg_fifo: BGFifo::default(),
            oam_fifo: OamFifo::default(),
            oam_scanned: false,
            object_drawn: 0,
            last_drawn_x: 0,
            x: 0,
        }
    }
}

impl PPU {
    pub fn cycle(&mut self, mmu: &mut MMU) -> Option<u8> {
        if !mmu.lcd_enabled() {
            self.dots = 0;
            mmu.mem[0xFF44] = 0;
            self.x = 0;
            return None;
        }

        if mmu.ly() == mmu.lyc() {
            if !mmu.ly_eq_ly() && mmu.lyc_int_select() {
                mmu.set_interrupt_flag(1);
            }

            mmu.set_lyc_eq_ly();
        } else {
            mmu.unset_lyc_eq_ly();
        }

        let result = match mmu.ppu_mode() {
            0 => self.mode_0(mmu),
            1 => self.mode_1(mmu),
            2 => self.mode_2(mmu),
            3 => self.mode_3(mmu),
            _ => unreachable!(),
        };

        self.dots += 1;

        if self.dots % 456 == 0 {
            mmu.mem[0xFF44] = mmu.ly().wrapping_add(1);
            self.x = 0;
        }

        return result;
    }

    fn mode_0(&mut self, mmu: &mut MMU) -> Option<u8> {
        if mmu.ly() == 144 {
            mmu.set_ppu_mode(1);
        } else if self.dots % 456 == 0 {
            mmu.set_ppu_mode(2);
        }

        None
    }

    fn mode_1(&mut self, mmu: &mut MMU) -> Option<u8> {
        if mmu.ly() == 154 {
            mmu.set_ppu_mode(2);
            self.dots = 0;
            mmu.mem[0xFF44] = 0;
            self.bg_fifo.window_y = 0;
        }

        None
    }

    fn mode_2(&mut self, mmu: &mut MMU) -> Option<u8> {
        if self.dots % 456 == 80 {
            mmu.set_ppu_mode(3);
            self.oam_scanned = false;
            self.object_drawn = 0;
            self.last_drawn_x = 0;
            self.bg_fifo.reset();
            self.oam_fifo.reset();
        }

        if !self.oam_scanned {
            self.objects.clear();

            mmu.mem[0xFE00..=0xFE9F].chunks(4).for_each(|obj: &[u8]| {
                if obj[0] >= 16 {
                    let obj_size = if mmu.obj_size() { 16 } else { 8 };
                    let obj_y = obj[0] - 16;

                    if mmu.ly() >= obj_y
                        && mmu.ly() < obj_y + obj_size
                        // && self.objects.len() < 10
                        && obj[1] >= 8
                    {
                        let x = obj[1] - 8;

                        if !self.objects.contains_key(&x) {
                            self.objects.insert(x, OamObject::new(obj, obj_size == 16));
                        }
                    }
                }
            });

            self.oam_scanned = true;
        }

        None
    }

    fn mode_3(&mut self, mmu: &mut MMU) -> Option<u8> {
        if self.object_drawn < 10 && self.objects.contains_key(&self.x) {
            if let Some(object) = self.objects.remove(&self.x)
                && object.x() - self.last_drawn_x >= 8
            {
                self.oam_fifo.object = Some(object);
                self.object_drawn += 1;
                self.last_drawn_x = self.oam_fifo.object.unwrap().x();
            }
        }

        self.oam_fifo.tick(mmu);

        if self.oam_fifo.fetching() {
            return None;
        }

        self.bg_fifo.tick(mmu, self.in_window(mmu));

        if let Some(bg_pixel) = self.bg_fifo.get_pixel() {
            self.x += 1;

            if self.x == 160 {
                self.x = 0;
                mmu.set_ppu_mode(0);
            }

            if let Some(pixel) = self.oam_fifo.get_pixel()
                && pixel.priority
                && pixel.data != 0
            {
                let colors = match pixel.palette {
                    Palette::OBP0 => mmu.obp0(),
                    Palette::OBP1 => mmu.obp1(),
                };

                return Some(match pixel.data {
                    1 => (colors >> 2) & 0b11,
                    2 => (colors >> 4) & 0b11,
                    3 => (colors >> 6) & 0b11,
                    _ => unreachable!(),
                });
            };

            return Some(bg_pixel);
        }

        return None;
    }

    fn in_window(&self, mmu: &MMU) -> bool {
        mmu.window_enabled() && self.x >= mmu.wx() - 7 && mmu.ly() >= mmu.wy()
    }
}
