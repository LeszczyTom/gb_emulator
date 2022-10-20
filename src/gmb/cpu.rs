pub struct LR35902 {
    a: u8,
    f: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    sp: u16,
    pc: u16,
    ime: bool,
}

impl LR35902 {
    pub fn new() -> LR35902 {
        LR35902 {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            f: 0,
            pc: 0, // Starting at cartridge first instruction bc no bios
            sp: 0,
            ime: false,
        }
    }

    pub fn _get_a(&self) -> u8 { self.a }
    pub fn _get_b(&self) -> u8 { self.b }
    pub fn _get_c(&self) -> u8 { self.c }
    pub fn _get_d(&self) -> u8 { self.d }
    pub fn _get_e(&self) -> u8 { self.e }
    pub fn _get_h(&self) -> u8 { self.h }
    pub fn _get_l(&self) -> u8 { self.l }
    pub fn _get_f(&self) -> u8 { self.f }
    pub fn get_sp(&self) -> u16 { self.sp }
    pub fn get_pc(&self) -> u16 { self.pc }

    pub fn _set_a(&mut self, val: u8) { self.a = val }
    pub fn _set_b(&mut self, val: u8) { self.b = val }
    pub fn _set_c(&mut self, val: u8) { self.c = val }
    pub fn _set_d(&mut self, val: u8) { self.d = val }
    pub fn _set_e(&mut self, val: u8) { self.e = val }
    pub fn _set_h(&mut self, val: u8) { self.h = val }
    pub fn _set_l(&mut self, val: u8) { self.l = val }
    pub fn _set_f(&mut self, val: u8) { self.f = val }
    pub fn set_sp(&mut self, val: u16) { self.sp = val }
    pub fn set_pc(&mut self, val: u16) { self.pc = val }

    pub fn get_af(&self) -> u16 { (self.a as u16) << 8 | self.f as u16 }
    pub fn get_bc(&self) -> u16 { (self.b as u16) << 8 | self.c as u16 }
    pub fn get_de(&self) -> u16 { (self.d as u16) << 8 | self.e as u16 }
    pub fn get_hl(&self) -> u16 { (self.h as u16) << 8 | self.l as u16 }

    pub fn set_af(&mut self, val: u16) {
        let bytes = val.to_be_bytes();
        self.a = bytes[0];
        self.f = bytes[1];
    }
    pub fn set_bc(&mut self, val: u16) {
        let bytes = val.to_be_bytes();
        self.b = bytes[0];
        self.c = bytes[1];
    }
    pub fn set_de(&mut self, val: u16) {
        let bytes = val.to_be_bytes();
        self.d = bytes[0];
        self.e = bytes[1];
    }
    pub fn set_hl(&mut self, val: u16) {
        let bytes = val.to_be_bytes();
        self.h = bytes[0];
        self.l = bytes[1];
    }
    
    pub fn get_r(&mut self, r: &str) -> u8 {
        match r {
            "a" => self.a,
            "b" => self.b,
            "c" => self.c,
            "d" => self.d,
            "e" => self.e,
            "h" => self.h,
            "l" => self.l,
            "f" => self.f,
            _ => panic!("Invalid register"),
        }
    }

    pub fn get_rr(&mut self, rr: &str) -> u16 {
        match rr {
            "af" => self.get_af(),
            "bc" => self.get_bc(),
            "de" => self.get_de(),
            "hl" => self.get_hl(),
            "sp" => self.sp,
            _ => panic!("Invalid register"),
        }
    }

    pub fn set_r(&mut self, r: &str, val: u8) {
        match r {
            "a" => self.a = val,
            "b" => self.b = val,
            "c" => self.c = val,
            "d" => self.d = val,
            "e" => self.e = val,
            "h" => self.h = val,
            "l" => self.l = val,
            "f" => self.f = val,
            _ => panic!("Invalid register"),
        }
    }

    pub fn set_rr(&mut self, rr: &str, val: u16) {
        match rr {
            "af" => self.set_af(val),
            "bc" => self.set_bc(val),
            "de" => self.set_de(val),
            "hl" => self.set_hl(val),
            "sp" => self.sp = val,
            _ => panic!("Invalid register"),
        }
    }

    pub fn set_flag(&mut self, flag :&str, value: bool) {
        if value {
            match flag {
                "z" => self.f |= 0b1000_0000,
                "n" => self.f |= 0b0100_0000,
                "h" => self.f |= 0b0010_0000,
                "c" => self.f |= 0b0001_0000,
                _ => panic!("Invalid flag"),
            }
        } else {
            match flag {
                "z" => self.f &= 0b0111_1111,
                "n" => self.f &= 0b1011_1111,
                "h" => self.f &= 0b1101_1111,
                "c" => self.f &= 0b1110_1111,
                _ => panic!("Invalid flag"),
            }
        }
    }

    pub fn get_flag(&self, flag: &str) -> bool {
        match flag {
            "z" => self.f & 0b10000000 != 0,
            "n" => self.f & 0b01000000 != 0,
            "h" => self.f & 0b00100000 != 0,
            "c" => self.f & 0b00010000 != 0,
            "nz" => !self.get_flag("z"),
            "nc" => !self.get_flag("c"),
            _ => panic!("Invalid flag"),
        }
    }

    pub fn reset(&mut self) {
        self.set_af(0x01b0);
        self.set_bc(0x0013);
        self.set_de(0x00d8);
        self.set_hl(0x014d);
        self.set_sp(0xfffe);    
        self.set_pc(0x0100);
    }

    pub fn set_ime(&mut self) { self.ime = true; }
    pub fn clear_ime(&mut self) { self.ime = false; }
    pub fn get_ime(&self) -> bool { self.ime }
}