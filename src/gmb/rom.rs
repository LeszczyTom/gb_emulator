use std::fs::read;

pub struct ROM {
    pub data: Vec<u8>,
}

impl ROM {
    pub fn new(path :&str) -> ROM {
        match read(path) {
            Ok(bytes) => { 
                ROM {
                    data: bytes,
                }
            },
            Err(e) => {
                panic!("Error: {}", e);
            }
        }
    }

    pub fn _fetch_rom_bank(&self) -> [u8; 0x8000] {
        let mut rom_bank = [0; 0x8000];
        for i in 0..0x8000 {
            rom_bank[i] = self.data[i];
        }
        rom_bank
    }
    
    fn _hex_dump(&self, start: usize, end: usize) {    
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

    pub fn _dump_header(&self) {
        print!("\n\nDumping header");
        self._hex_dump(0x0100, 0x0150);
    }

    pub fn _dump_logo(&self) {
        print!("\n\nDumping Nintendo logo");
        self._hex_dump(0x0104, 0x0134);
    }

    pub fn _dump_title(&self) {
        print!("\n\nDumping title");
        self._hex_dump(0x0134, 0x0144);
    }

    pub fn _dump_manufacturer_code(&self) {
        print!("\n\nDumping manufacturer code");
        self._hex_dump(0x013F, 0x0143);
    }

    pub fn _dump_cgb_flag(&self) {
        print!("\n\nDumping CGB flag");
        self._hex_dump(0x0143, 0x0144);
    }

    pub fn _dump_licence_code(&self) {
        print!("\n\nDumping licence code");
        self._hex_dump(0x0144, 0x0146);
    }

    pub fn _dump_sgb_flag(&self) {
        print!("\n\nDumping SGB flag");
        self._hex_dump(0x0146, 0x0147);
    }

    pub fn _dump_cartrige_type(&self) {
        print!("\n\nDumping cartrige type");
        self._hex_dump(0x0147, 0x0148);
    }

    pub fn _dump_rom_size(&self) {
        print!("\n\nDumping ROM size");
        self._hex_dump(0x0148, 0x0149);
    }

    pub fn _dump_ram_size(&self) {
        print!("\n\nDumping RAM size");
        self._hex_dump(0x0149, 0x014A);
    }

    pub fn _dump_destination_code(&self) {
        print!("\n\nDumping destination code");
        self._hex_dump(0x014A, 0x014B);
    }

    pub fn _dump_old_licensee_code(&self) {
        print!("\n\nDumping old licensee code");
        self._hex_dump(0x014B, 0x014C);
    }

    pub fn _dump_mask_rom_version_number(&self) {
        print!("\n\nDumping mask ROM version number");
        self._hex_dump(0x014C, 0x014D);
    }

    pub fn _dump_header_checksum(&self) {
        print!("\n\nDumping header checksum");
        self._hex_dump(0x014D, 0x014E);
    }
}

fn _header_checksum(data: &Vec<u8>) -> bool {
    let mut sum: i16 = 0;
    for i in 0x134..0x14D {
        sum = sum - i16::from(data[i])  - 1;
    }
    if data[0x14D] == i16::to_be_bytes(sum)[1] {
        return true;
    }
    false
}