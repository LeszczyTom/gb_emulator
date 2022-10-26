const MEM_SIZE: usize = 0x10000;

pub struct Memory {
    data: [u8; MEM_SIZE],
}

impl Memory {
    pub fn new() -> Self {
        let bios = match std::fs::read("resources/bios.bin") {
            Ok(bytes) => bytes,
            Err(e) => panic!("Error: {}", e)
        }; 

        let mut data = [0; MEM_SIZE];
        for i in 0..bios.len() {
            data[i] = bios[i];
        }

        Self {
            data,
        }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        self.data[addr as usize]
    }

    pub fn write_byte(&mut self, addr: u16, val: u8) {
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