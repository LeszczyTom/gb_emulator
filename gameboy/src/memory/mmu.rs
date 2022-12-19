const MEM_SIZE: usize = 0x10000;

pub struct Mmu {
    data: [u8; MEM_SIZE],
    bios: [u8; 0x100],
    bios_enabled: bool,
}

impl Mmu {
    pub fn new() -> Self {
        let bios = match std::fs::read("resources/bios.bin") {
            Ok(bytes) => bytes,
            Err(e) => panic!("Error: {}", e)
        }; 
        
        let rom = match std::fs::read("resources/dr_mario.gb") {
            Ok(bytes) => bytes,
            Err(e) => panic!("Error: {}", e)
        }; 
        
        let mut data = [0; MEM_SIZE];
        data[0..0x8000].copy_from_slice(&rom);

        Self {
            data,
            bios: bios.try_into()
                .unwrap_or_else(|v: Vec<u8>| panic!("Expected a Vec of length {} but it was {}", 100, v.len())),
            bios_enabled: true,
        }
    }

    pub fn set_bios_enabled(&mut self, enabled: bool) {
        self.bios_enabled = enabled;
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        if self.bios_enabled & (addr < 0x100) {
            return self.bios[addr as usize];
        } 
        self.data[addr as usize] 
    }

    pub fn write_byte(&mut self, addr: u16, val: u8) {
        self.data[addr as usize] = val;

        match addr {
            0xFF50 => self.set_bios_enabled(val == 0),
            0xFF04 => { // Reset timer registers if writting to DIV
                self.data[0xFF04] = 0;
                self.data[0xFF05] = 0;

            },
            0xFF01 => { // Serial debug
                print!("{}", val as char);
            },
            _ => ()
        }
    }

    pub fn get_ly(&self) -> u8 {
        self.data[0xFF44]
    }

    pub fn set_ly(&mut self, val: u8) {
        self.data[0xFF44] = val;
    }

    pub fn get_scy(&self) -> u8 {
        self.data[0xFF42]
    }

    pub fn get_scx(&self) -> u8 {
        self.data[0xFF43]
    }

    pub fn get_background_palette(&self, index :u8) -> usize {
        (self.data[0xFF47] >> (index * 2) & 0x3) as usize
    }

    pub fn set_stat_mode_flag(&mut self, mode: u8) {
        self.data[0xFF41] = (self.data[0xFF41] & 0xFC) | mode;
    }

    pub fn set_interrupt_flag(&mut self, flag: u8) {
        self.data[0xFF0F] |= flag;
    }

    pub fn increment_divider(&mut self) {
        if self.data[0xFF03] == 0xFF {
            if self.data[0xFF04] == 0xFF {
                self.data[0xFF04] = 0;
                self.data[0xFF03] = 0;
            } else {
                self.data[0xFF03] = 0;
                self.data[0xFF04] += 1;
            }
        } else {
            self.data[0xFF03] += 1;
        }
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

    pub fn get_tile(&self, index: usize) -> [u8; 16] {
        let mut tile = [0; 16];
        let start = 0x8000 + (index * 16);
        tile.copy_from_slice(&self.data[start..start+16]);
        tile
    }

    pub fn get_background_tile_id(&self, index: usize) -> u8 {
        self.data[0x9800 + index]
    }
}