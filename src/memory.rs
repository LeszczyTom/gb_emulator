// Memory

/*
  0000-3FFF   16KB ROM Bank 00     (in cartridge, fixed at bank 00)
  4000-7FFF   16KB ROM Bank 01..NN (in cartridge, switchable bank number)
  8000-9FFF   8KB Video RAM (VRAM) (switchable bank 0-1 in CGB Mode)
  A000-BFFF   8KB External RAM     (in cartridge, switchable bank, if any)
  C000-CFFF   4KB Work RAM Bank 0 (WRAM)
  D000-DFFF   4KB Work RAM Bank 1 (WRAM)  (switchable bank 1-7 in CGB Mode)
  E000-FDFF   Same as C000-DDFF (ECHO)    (typically not used)
  FE00-FE9F   Sprite Attribute Table (OAM)
  FEA0-FEFF   Not Usable
  FF00-FF7F   I/O Ports
  FF80-FFFE   High RAM (HRAM)
  FFFF        Interrupt Enable Register
 */

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

    pub fn read_byte(&self, address: u16) -> u8 {
        self.data[address as usize]
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        self.data[address as usize] = value;
    }

    pub fn load_rom(&mut self, rom: [u8; 0x8000]) {
        for (i, byte) in rom.iter().enumerate() {
            self.data[i] = *byte;
        }
    }

    fn dump_hex(&self, start: usize, end: usize) {
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

    pub fn dump_rom_bank_0(&self) {
        self.dump_hex(0x0000, 0x4000);
    }

    pub fn dump_rom_bank_1(&self) {
        self.dump_hex(0x4000, 0x8000);
    }

    pub fn dump_vram(&self) {
        self.dump_hex(0x8000, 0xA000);
    }

    pub fn dump_ext_ram(&self) {
        self.dump_hex(0xA000, 0xC000);
    }

    pub fn dump_wram_0(&self) {
        self.dump_hex(0xC000, 0xD000);
    }

    pub fn dump_wram_1(&self) {
        self.dump_hex(0xD000, 0xE000);
    }

    pub fn dump_echo(&self) {
        self.dump_hex(0xE000, 0xFE00);
    }

    pub fn dump_oam(&self) {
        self.dump_hex(0xFE00, 0xFEA0);
    }

    pub fn dump_io_ports(&self) {
        self.dump_hex(0xFF00, 0xFF80);
    }

    pub fn dump_hram(&self) {
        self.dump_hex(0xFF80, 0xFFFF);
    }

    pub fn dump_int_enable_reg(&self) {
        self.dump_hex(0xFFFF, 0x10000);
    }
}