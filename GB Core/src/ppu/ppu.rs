use crate::{log, mmu::MMU, ppu::fifo::FIFO};

pub struct PPU {
    dots: u32,
    fifo: FIFO,
}

impl Default for PPU {
    fn default() -> Self {
        Self {
            dots: 0,
            fifo: FIFO::default(),
        }
    }
}

impl PPU {
    pub fn cycle(&mut self, mmu: &mut MMU) -> Option<u8> {
        if !mmu.lcd_enabled() {
            self.dots = 0;
            mmu.mem[0xFF44] = 0;
            return None;
        }

        let result = match mmu.ppu_mode() {
            0 => {
                if mmu.ly() == 144 {
                    mmu.set_ppu_mode(1);
                } else if self.dots % 456 == 0 {
                    mmu.set_ppu_mode(2);
                }

                None
            } // mode 0
            1 => {
                if mmu.ly() == 154 {
                    mmu.set_ppu_mode(2);
                    self.dots = 0;
                }
                None
            } // mode 1
            2 => {
                if self.dots % 456 == 80 {
                    mmu.set_ppu_mode(3);
                }
                None
            } // mode 2
            3 => {
                let result = self.fifo.tick(mmu);
                // log!("tick: {}", self.dots);

                if self.fifo.line_done() {
                    // log!("Reset");
                    self.fifo.reset();
                    mmu.set_ppu_mode(0);
                }

                result
            } // mode 3
            _ => unreachable!(),
        };

        self.dots += 1;

        if self.dots % 456 == 0 {
            self.increment_ly(mmu);
        }

        if mmu.ly() == mmu.lyc() {
            if !mmu.ly_eq_ly() && mmu.lyc_int_select() {
                mmu.set_interrupt_flag(1);
            }

            mmu.set_lyc_eq_ly();
        } else {
            mmu.unset_lyc_eq_ly();
        }

        return result;
    }

    fn increment_ly(&mut self, mmu: &mut MMU) {
        let ly = mmu.ly().wrapping_add(1);

        if ly == 154 {
            mmu.mem[0xFF44] = 0
        } else {
            mmu.mem[0xFF44] = ly;
        }
    }
}
