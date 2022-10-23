const MEM_SIZE: usize = 0x10000;

pub struct MEMORY {
    data: [u8; MEM_SIZE],
}

impl MEMORY {
    pub fn new() -> MEMORY {
        MEMORY {
            data: [0; MEM_SIZE],
        }
    }

    pub fn reset(&mut self) {
        self.data[0xFF05] = 0x00;
        self.data[0xFF06] = 0x00;
        self.data[0xFF07] = 0x00;
        self.data[0xFF10] = 0x80;
        self.data[0xFF11] = 0xBF;
        self.data[0xFF12] = 0xF3;
        self.data[0xFF14] = 0xBF;
        self.data[0xFF16] = 0x3F;
        self.data[0xFF17] = 0x00;
        self.data[0xFF19] = 0xBF;
        self.data[0xFF1A] = 0x7F;
        self.data[0xFF1B] = 0xFF;
        self.data[0xFF1C] = 0x9F;
        self.data[0xFF1E] = 0xBF;
        self.data[0xFF20] = 0xFF;
        self.data[0xFF21] = 0x00;
        self.data[0xFF22] = 0x00;
        self.data[0xFF23] = 0xBF;
        self.data[0xFF24] = 0x77;
        self.data[0xFF25] = 0xF3;
        self.data[0xFF26] = 0xF1; //0xF1 -GB, 0xF0 -SGB
        self.data[0xFF40] = 0x91;
        self.data[0xFF42] = 0x00;
        self.data[0xFF43] = 0x00;
        self.data[0xFF45] = 0x00;
        self.data[0xFF47] = 0xFC;
        self.data[0xFF48] = 0xFF;
        self.data[0xFF49] = 0xFF;
        self.data[0xFF4A] = 0x00;
        self.data[0xFF4B] = 0x00;
        self.data[0xFFFF] = 0x00;
    }

    pub fn load_bios(&mut self) {
        match std::fs::read("resources/bios.bin") {
            Ok(bytes) => { 
                for i in 0..bytes.len() {
                    self.data[i] = bytes[i];
                }
            },
            Err(e) => {
                panic!("Error: {}", e);
            }
        }        
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        self.data[addr as usize]
    }

    pub fn read_word(&self, addr: u16) -> u16 { // TODO: check if it's correct
        let mut bytes = [0,0];
        bytes[1] = self.data[addr as usize];
        let addr1 = addr.wrapping_add(1);
        bytes[0] = self.data[addr1 as usize]; //?
        u16::from_be_bytes(bytes)
    }

    pub fn write_byte(&mut self, addr: u16, val: u8) {
        self.data[addr as usize] = val;
    }

    pub fn load_rom(&mut self, rom: Vec<u8>) {
        for i in 0..rom.len() {
            self.data[i] = rom[i];
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