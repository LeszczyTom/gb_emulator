use crate::{
    log,
    mmu::MMU,
    ppu::{bg_fifo::BGFifo, oam_fifo::OamFifo, object::OamObject},
};

pub struct PPU {
    dots: u32,
    objects: Vec<OamObject>,
    bg_fifo: BGFifo,
    oam_fifo: OamFifo,
    oam_scanned: bool,
    x: u8,
}

impl Default for PPU {
    fn default() -> Self {
        Self {
            dots: 0,
            objects: vec![],
            bg_fifo: BGFifo::default(),
            oam_fifo: OamFifo::default(),
            oam_scanned: false,
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
        }

        if !self.oam_scanned {
            self.objects.clear();

            mmu.mem[0xFE00..=0xFE9F].chunks(4).for_each(|obj: &[u8]| {
                if obj[0] >= 16 {
                    let obj_size = if mmu.obj_size() { 16 } else { 8 };
                    let obj_y = obj[0] - 16;

                    if mmu.ly() >= obj_y && mmu.ly() < obj_y + obj_size && self.objects.len() < 10 {
                        self.objects.push(OamObject::new(obj, obj_size == 16));
                    }
                }
            });

            self.oam_scanned = true;
        }

        None
    }

    fn mode_3(&mut self, mmu: &mut MMU) -> Option<u8> {
        let object_index = self.objects.iter().enumerate().find_map(|(index, object)| {
            if object.x() >= 8 && object.x() - 8 == self.oam_fifo.fetcher_x * 8 {
                return Some(index);
            }

            return None;
        });

        if let Some(index) = object_index {
            self.oam_fifo.object = Some(self.objects.remove(index));
        }

        let in_window = self.in_window(mmu);
        let obj_fetching = self.oam_fifo.fetching();

        self.oam_fifo.tick(mmu);
        let bg_pixel = self.bg_fifo.tick(mmu, obj_fetching, in_window);

        let output_pixel = if bg_pixel.is_some()
            && let Some((pixel, prio)) = self.oam_fifo.get_pixel()
        // && prio
        // && pixel != 0
        {
            Some(pixel)
        } else {
            bg_pixel
        };

        if bg_pixel.is_some() {
            self.x += 1;

            if self.x == 160 {
                self.x = 0;
                self.bg_fifo.reset();
                self.oam_fifo.reset();
                mmu.set_ppu_mode(0);
            }
        }

        output_pixel
    }

    fn in_window(&self, mmu: &MMU) -> bool {
        mmu.window_enabled() && self.x >= mmu.wx() - 7 && mmu.ly() >= mmu.wy()
    }
}
