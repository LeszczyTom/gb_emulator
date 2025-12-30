use std::path::PathBuf;

use crate::log;

const MEMORY_SIZE: usize = u16::MAX as usize + 1;

pub struct MMU {
    pub bios: [u8; 0x100],
    pub mem: [u8; MEMORY_SIZE],
    pub bios_mapped: bool,

    #[cfg(test)]
    pub(crate) serial_output: [u8; 6],
}

impl Default for MMU {
    fn default() -> MMU {
        let mut mem = [0x0; MEMORY_SIZE];
        mem[0..0x7FFF].copy_from_slice(&[0xFF; 0x7FFF]);

        MMU {
            bios: [0; 0x100],
            mem,
            bios_mapped: true,

            #[cfg(test)]
            serial_output: [0; 6],
        }
    }
}

impl MMU {
    pub fn get(&self, address: u16) -> u8 {
        // VRAM not accessible in mode 3
        if address >= 0x8000 && address <= 0x9FFF && self.ppu_mode() == 3 {
            return 0xFF;
        }

        // OAM not accessible in mode 2-3
        if address >= 0xFE00 && address <= 0xFE9F && (self.ppu_mode() == 2 || self.ppu_mode() == 3)
        {
            return 0xFF;
        }

        if self.bios_mapped && address <= 0xFF {
            return self.bios[address as usize];
        }

        return self.mem[address as usize];
    }

    pub fn set(&mut self, value: u8, address: u16) {
        match address {
            // VRAM
            0x8000..=0x9FFF => {
                // Ignore if mode 3
                if self.ppu_mode() == 3 {
                    return;
                }
            }
            0xFE00..=0xFE9F => {
                // Ignore if mode 2 or 3
                if self.ppu_mode() == 2 || self.ppu_mode() == 3 {
                    return;
                }
            }
            0xFF01 => self.handle_serial(value),
            0xFF04 => {
                // Reset timer registers if writting to DIV
                self.mem[0xFF04] = 0;
                self.mem[0xFF05] = 0;
            }
            0xFF44 => {
                // Reset LY register if writting to it
                self.mem[0xFF44] = 0;
            }
            0xFF50 => {
                log!("Bios unmapped: {}", value);
                if value == 1 {
                    self.bios_mapped = false;
                }
            }
            _ => (),
        }

        self.mem[address as usize] = value;
    }

    fn handle_serial(&mut self, value: u8) {
        print!("{}", value as char);

        #[cfg(test)]
        {
            self.serial_output.rotate_left(1);
            *self.serial_output.last_mut().unwrap() = value;
        }
    }

    pub fn set_interrupt_flag(&mut self, flag: u8) {
        self.mem[0xFF0F] |= 1 << flag;
    }

    pub fn scx(&self) -> u8 {
        self.get(0xFF43)
    }

    pub fn scy(&self) -> u8 {
        self.get(0xFF42)
    }
    pub fn wx(&self) -> u8 {
        self.get(0xFF4B)
    }
    pub fn wy(&self) -> u8 {
        self.get(0xFF4A)
    }

    pub fn ly(&self) -> u8 {
        return self.mem[0xFF44];
    }

    pub fn lyc(&self) -> u8 {
        return self.mem[0xFF45];
    }

    pub fn lyc_int_select(&self) -> bool {
        return self.lcds() >> 6 & 1 == 1;
    }

    pub fn mode_2_int_select(&self) -> bool {
        return self.lcds() >> 5 & 1 == 1;
    }

    pub fn mode_1_int_select(&self) -> bool {
        return self.lcds() >> 4 & 1 == 1;
    }

    pub fn mode_0_int_select(&self) -> bool {
        return self.lcds() >> 3 & 1 == 1;
    }
    pub fn set_ppu_mode(&mut self, mode: u8) {
        if match mode {
            0b00 => self.mode_0_int_select(),
            0b01 => {
                self.set_interrupt_flag(0);
                self.mode_1_int_select()
            }
            0b10 => self.mode_2_int_select(),
            0b11 => false,
            _ => unreachable!(),
        } {
            self.set_interrupt_flag(1);
        }

        self.mem[0xFF41] = (self.mem[0xFF41] & 0b1111_1100) | mode;
        // log!("Mode: {} - {:08b}", self.mem[0xFF41], self.mem[0xFF41]);
    }

    pub fn ppu_mode(&self) -> u8 {
        return self.lcds() & 0b11;
    }

    pub fn ly_eq_ly(&self) -> bool {
        return (self.mem[0xFF41] >> 2) & 1 == 1;
    }

    pub fn set_lyc_eq_ly(&mut self) {
        self.mem[0xFF41] |= 0b0000_0100;
    }

    pub fn unset_lyc_eq_ly(&mut self) {
        self.mem[0xFF41] &= 0b1111_1011;
    }

    pub fn lcdc(&self) -> u8 {
        return self.mem[0xFF40];
    }

    pub fn lcds(&self) -> u8 {
        return self.mem[0xFF41];
    }

    pub fn bg_tile_map_area(&self) -> bool {
        return (self.lcdc() >> 3) & 1 == 1;
    }

    pub fn bg_window_tile(&self) -> bool {
        return (self.lcdc() >> 4) & 1 == 1;
    }

    pub fn window_enabled(&self) -> bool {
        return (self.lcdc() >> 5) & 1 == 1;
    }

    pub fn window_tile_map(&self) -> bool {
        return (self.lcdc() >> 6) & 1 == 1;
    }

    pub fn lcd_enabled(&self) -> bool {
        return (self.lcdc() >> 7) & 1 == 1;
    }

    pub fn load_file(
        &mut self,
        file_path: PathBuf,
    ) -> Result<(), Box<dyn std::error::Error + 'static>> {
        let rom = std::fs::read(file_path)?;
        self.mem[0..0x8000].copy_from_slice(&rom);
        Ok(())
    }
}
