use std::path::PathBuf;

const MEM_SIZE: usize = 0x10000;

pub struct Mmu {
    data: [u8; MEM_SIZE],
    bios: [u8; 0x100],
    bios_enabled: bool,
}

impl Default for Mmu {
    fn default() -> Self {
        let bios = match std::fs::read("resources/bios.bin") {
            Ok(bytes) => bytes,
            Err(e) => panic!("Error: {}", e),
        };

        Self {
            data: [0; MEM_SIZE],
            bios: bios.try_into().unwrap_or_else(|v: Vec<u8>| {
                panic!("Expected a Vec of length {} but it was {}", 100, v.len())
            }),
            bios_enabled: true,
        }
    }
}

impl Mmu {
    pub fn new(rom_path: PathBuf) -> Self {
        let rom = match std::fs::read(rom_path) {
            Ok(bytes) => bytes,
            Err(e) => {
                println!("Error: {}", e);
                return Self {
                    ..Default::default()
                };
            }
        };

        let mut data = [0; MEM_SIZE];
        data[0..0x8000].copy_from_slice(&rom);

        Self {
            data,
            ..Default::default()
        }
    }

    pub fn get_data(&self) -> [u8; MEM_SIZE] {
        self.data
    }

    pub fn get_slice_data(&self, addr: usize) -> Vec<u8> {
        self.data[addr..addr + 16].to_vec()
    }

    pub fn set_bios_enabled(&mut self, enabled: bool) {
        self.bios_enabled = enabled;
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        if self.bios_enabled & (addr < 0x100) {
            return self.bios[addr as usize];
        }
        // if addr == 0xFF00 {}
        self.data[addr as usize]
    }

    pub fn write_byte(&mut self, addr: u16, val: u8) {
        self.data[addr as usize] = val;

        match addr {
            0xFF50 => self.set_bios_enabled(val == 0),
            0xFF04 => {
                // Reset timer registers if writting to DIV
                self.data[0xFF03] = 0;
                self.data[0xFF04] = 0;
                self.data[0xFF05] = 0;
            }
            0xFF44 => {
                // Reset LY register if writting to it
                self.data[0xFF44] = 0;
            }
            0xFF01 => {
                // Serial debug
                print!("{}", val as char);
            }
            _ => (),
        }
    }

    pub fn get_ly(&self) -> u8 {
        self.data[0xFF44]
    }

    pub fn get_scy(&self) -> usize {
        self.data[0xFF42] as usize
    }

    pub fn increment_ly(&mut self) {
        self.data[0xFF44] += 1;
    }

    pub fn get_scx(&self) -> usize {
        self.data[0xFF43] as usize
    }

    pub fn get_wx(&self) -> usize {
        self.data[0xFF4B] as usize
    }

    pub fn get_wy(&self) -> usize {
        self.data[0xFF4A] as usize
    }

    pub fn get_background_palette(&self, index: u8) -> usize {
        (self.data[0xFF47] >> (index * 2) & 0x3) as usize
    }

    pub fn get_object_palette(&self, index: u8, opb0: bool) -> usize {
        if opb0 {
            return (self.data[0xFF48] >> (index * 2) & 0x3) as usize;
        }
        return (self.data[0xFF49] >> (index * 2) & 0x3) as usize;
    }

    pub fn set_stat_mode_flag(&mut self, mode: u8) {
        self.data[0xFF41] = (self.data[0xFF41] & 0xFC) | mode;
    }

    pub fn set_interrupt_flag(&mut self, flag: u8) {
        self.data[0xFF0F] |= 1 << flag;
    }

    pub fn increment_div_high(&mut self) {
        self.data[0xFF04] = self.data[0xFF04].checked_add(1).unwrap_or(0);
    }

    pub fn reset_div_low(&mut self) {
        self.data[0xFF03] = 0;
    }

    pub fn reset_div_high(&mut self) {
        self.data[0xFF04] = 0;
    }

    pub fn increment_div_low(&mut self) {
        self.data[0xFF03] += 1;
    }

    pub fn _dump_hex(&self, start: usize, end: usize) {
        let mut cpt = 0;
        for i in start..end {
            if cpt == 0 {
                print!("|\n{:04X}: | ", i);
                cpt = 16;
            }
            print!("{:02X} ", self.data[i]);
            cpt -= 1;
        }
        println!("|");
    }

    pub fn get_oam_slice(&self) -> &[u8] {
        &self.data[0xFE00..0xFEA0]
    }

    pub fn get_tile(&self, index: usize) -> [u8; 16] {
        let mut tile = [0; 16];
        let start = 0x8000 + (index * 16);
        tile.copy_from_slice(&self.data[start..start + 16]);
        tile
    }

    pub fn get_background_tile_id(&self, index: usize) -> u8 {
        self.data[0x9800 + index]
    }
}
