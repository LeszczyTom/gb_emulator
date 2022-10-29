const MEM_SIZE: usize = 0x10000;

pub struct Memory {
    data: [u8; MEM_SIZE],
    bios: [u8; 0x100],
    bios_enabled: bool,
}

impl Memory {
    pub fn new() -> Self {
        let bios = match std::fs::read("resources/bios.bin") {
            Ok(bytes) => bytes,
            Err(e) => panic!("Error: {}", e)
        }; 

        let rom = match std::fs::read("resources/tetris.gb") {
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
        if addr == 0xff50 {
            self.set_bios_enabled(val == 0);
        }
        self.data[addr as usize] = val;
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

    pub fn set_stat_mode_flag(&mut self, mode: u8) {
        self.data[0xFF41] = (self.data[0xFF41] & 0xFC) | mode;
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
}