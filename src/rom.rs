use std::{fs::File, io::{BufReader, Read}};

pub struct ROM {
    data: Vec<u8>,
}

impl ROM {
    pub fn new() -> ROM {
        ROM {
            data: Vec::new(),
        }
    }

    pub fn load_rom(&mut self, rom_path: &str) {
        match File::open(rom_path) {
            Ok(file) => {
                let mut reader = BufReader::new(file);
                let mut buffer = Vec::new();
                match reader.read_to_end(&mut buffer) {
                    Ok(_) => {
                        self.data = buffer;
                        if !self.header_checksum() {
                            panic!("Error: checksum failed!");
                        }
                    },
                    Err(_) => panic!("Error reading file"),
                }
            },
            Err(_) => panic!("Error opening file"),
        }
    }

    fn header_checksum(&self) -> bool {
        let mut sum: i16 = 0;
        for i in 0x134..0x14D {
            sum = sum - i16::from(self.data[i])  - 1;
        }
        if self.data[0x14D] == i16::to_be_bytes(sum)[1] {
            return true;
        }
        false
    }

    pub fn fetch_opcode(&self, pc: u16) -> u16 {
        self.data[pc as usize] as u16
    }
    
    fn hex_dump(&self, start: usize, end: usize) {    
        let mut cpt = 0;
        for i in start..end {
            if cpt == 0 {
                print!("\n{:04X} |", i);
                cpt = 16;
            }
            print!("{:02X} ", self.data[i]); 
            cpt -= 1;
        }
    }

    fn dump_header(&self) {
        print!("\n\nDumping header");
        self.hex_dump(0x0100, 0x0150);
    }

    fn dump_logo(&self) {
        print!("\n\nDumping Nintendo logo");
        self.hex_dump(0x0104, 0x0134);
    }

    fn dump_title(&self) {
        print!("\n\nDumping title");
        self.hex_dump(0x0134, 0x0144);
    }

    fn dump_manufacturer_code(&self) {
        print!("\n\nDumping manufacturer code");
        self.hex_dump(0x013F, 0x0143);
    }

    fn dump_cgb_flag(&self) {
        print!("\n\nDumping CGB flag");
        self.hex_dump(0x0143, 0x0144);
    }

    fn dump_licence_code(&self) {
        print!("\n\nDumping licence code");
        self.hex_dump(0x0144, 0x0146);
    }

    fn dump_sgb_flag(&self) {
        print!("\n\nDumping SGB flag");
        self.hex_dump(0x0146, 0x0147);
    }

    fn dump_cartrige_type(&self) {
        print!("\n\nDumping cartrige type");
        self.hex_dump(0x0147, 0x0148);
    }

    fn dump_rom_size(&self) {
        print!("\n\nDumping ROM size");
        self.hex_dump(0x0148, 0x0149);
    }

    fn dump_ram_size(&self) {
        print!("\n\nDumping RAM size");
        self.hex_dump(0x0149, 0x014A);
    }

    fn dump_destination_code(&self) {
        print!("\n\nDumping destination code");
        self.hex_dump(0x014A, 0x014B);
    }

    fn dump_old_licensee_code(&self) {
        print!("\n\nDumping old licensee code");
        self.hex_dump(0x014B, 0x014C);
    }

    fn dump_mask_rom_version_number(&self) {
        print!("\n\nDumping mask ROM version number");
        self.hex_dump(0x014C, 0x014D);
    }

    fn dump_header_checksum(&self) {
        print!("\n\nDumping header checksum");
        self.hex_dump(0x014D, 0x014E);
    }

    pub fn dump_cartrige_info(&self) {
        self.dump_header();
        self.dump_logo();
        self.dump_title();
        self.dump_manufacturer_code();
        self.dump_cgb_flag();
        self.dump_licence_code();
        self.dump_sgb_flag();
        self.dump_cartrige_type();
        self.dump_rom_size();
        self.dump_ram_size();
        self.dump_destination_code();
        self.dump_old_licensee_code();
        self.dump_mask_rom_version_number();
        self.dump_header_checksum();
    }
}

