pub struct LR35902 {
    // Registers
    a: u8, // Accumulator
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8, 
    f: u8, // Flags
    pub pc: u16, // Program counter
    sp: u16, // Stack pointer
}

impl LR35902 {
    pub fn new() -> LR35902 {
        LR35902 {
            a: 0x01,
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xD8,
            h: 0x01,
            l: 0x4D,
            f: 0xB0,
            pc: 0x0100, // Starting at cartridge first instruction bc no bios
            sp: 0xFFFE,
        }
    }

    pub fn get_r(&self, r: &str) -> u8 {
        match r {
            "a" => self.a,
            "b" => self.b,
            "c" => self.c,
            "d" => self.d,
            "e" => self.e,
            "h" => self.h,
            "l" => self.l,
            _ => panic!("Invalid register"),
        }
    }

    pub fn get_rr(&self, rr: &str) -> u16 {
        match rr {
            "af" => ((self.a as u16) << 8) | (self.f as u16),
            "bc" => ((self.b as u16) << 8) | (self.c as u16),
            "de" => ((self.d as u16) << 8) | (self.e as u16),
            "hl" => ((self.h as u16) << 8) | (self.l as u16),
            _ => panic!("Invalid register"),
        }
    }

    pub fn set_register(&mut self, r: &str, value: u8) {
        match r {
            "a" => self.a = value,
            "b" => self.b = value,
            "c" => self.c = value,
            "d" => self.d = value,
            "e" => self.e = value,
            "h" => self.h = value,
            "l" => self.l = value,
            _ => panic!("Invalid register"),
        }
    }


    pub fn debug(&self) {
        println!("A: {:02X}, B: {:02x}, C: {:02x}, D: {:02x}, E: {:02x}, F: {:02x}, H: {:02x}, L: {:02x}, pc: {:04x}, sp: {:04x}", self.a, self.b, self.c, self.d, self.e, self.f, self.h, self.l, self.pc, self.sp);
    }
    pub fn get_af(&self) -> u16 { u16::from_be_bytes([self.a, self.f]) }
    pub fn get_bc(&self) -> u16 { u16::from_be_bytes([self.b, self.c]) }
    pub fn get_de(&self) -> u16 { u16::from_be_bytes([self.d, self.e]) }
    pub fn get_hl(&self) -> u16 { u16::from_be_bytes([self.h, self.l]) }
    
    pub fn get_a(&self) -> u8 { self.a }
    pub fn get_b(&self) -> u8 { self.b }
    pub fn get_c(&self) -> u8 { self.c }
    pub fn get_d(&self) -> u8 { self.d }
    pub fn get_e(&self) -> u8 { self.e }
    pub fn get_h(&self) -> u8 { self.h }
    pub fn get_l(&self) -> u8 { self.l }
    pub fn get_f(&self) -> u8 { self.f }
    pub fn get_z_flag(&self) -> bool { self.f & 0b10000000 != 0 }
    pub fn get_n_flag(&self) -> bool { self.f & 0b01000000 != 0 }
    pub fn get_h_flag(&self) -> bool { self.f & 0b00100000 != 0 }
    pub fn get_c_flag(&self) -> bool { self.f & 0b00010000 != 0 }
    pub fn get_pc(&mut self) -> u16 {
        self.pc += 1;
        self.pc - 1
    }
    pub fn get_sp(&mut self) -> u16 {
        self.sp
    }

    pub fn set_pc(&mut self, pc: u16) { self.pc = pc }
    pub fn set_sp(&mut self, sp: u16) { self.sp = sp }
    pub fn set_af(&mut self, value: u16) {
        let bytes = value.to_be_bytes();
        self.a = bytes[0];
        self.f = bytes[1];
    }

    pub fn set_bc(&mut self, value: u16) {
        let bytes = value.to_be_bytes();
        self.b = bytes[0];
        self.c = bytes[1];
    }

    pub fn set_de(&mut self, value: u16) {
        let bytes = value.to_be_bytes();
        self.d = bytes[0];
        self.e = bytes[1];
    }

    pub fn set_hl(&mut self, value: u16) {
        let bytes = value.to_be_bytes();
        self.h = bytes[0];
        self.l = bytes[1];
    }

    pub fn set_a(&mut self, value: u8) {
        let mut af: [u8; 2] = self.get_af().to_be_bytes();
        af[0] = value;
        self.set_af(u16::from_be_bytes(af));
    }

    pub fn set_b(&mut self, value: u8) {
        let mut bc: [u8; 2] = self.get_bc().to_be_bytes();
        bc[0] = value;
        self.set_bc(u16::from_be_bytes(bc));
    }

    pub fn set_c(&mut self, value: u8) {
        let mut bc: [u8; 2] = self.get_bc().to_be_bytes();
        bc[1] = value;
        self.set_bc(u16::from_be_bytes(bc));
    }

    pub fn set_d(&mut self, value: u8) {
        let mut de: [u8; 2] = self.get_de().to_be_bytes();
        de[0] = value;
        self.set_de(u16::from_be_bytes(de));
    }

    pub fn set_e(&mut self, value: u8) {
        let mut de: [u8; 2] = self.get_de().to_be_bytes();
        de[1] = value;
        self.set_de(u16::from_be_bytes(de));
    }

    pub fn set_h(&mut self, value: u8) {
        let mut hl: [u8; 2] = self.get_hl().to_be_bytes();
        hl[0] = value;
        self.set_hl(u16::from_be_bytes(hl));
    }

    pub fn set_l(&mut self, value: u8) {
        let mut hl: [u8; 2] = self.get_hl().to_be_bytes();
        hl[1] = value;
        self.set_hl(u16::from_be_bytes(hl));
    }

    pub fn flag_zero(&mut self, value: bool) {
        if value {
            self.f |= 0b1000_0000;
        } else {
            self.f &= 0b0111_1111;
        }    
    }
    
    pub fn flag_substract(&mut self, value: bool) {        
        if value {
            self.f |= 0b0100_0000;
        } else {
            self.f &= 0b1011_1111;
        }
    }
    
    pub fn flag_half_carry(&mut self, value: bool) {        
        if value {
            self.f |= 0b0010_0000;
        } else {
            self.f &= 0b1101_1111;
        } 
    }
    
    pub fn flag_carry(&mut self, value: bool) {     
        if value {
            self.f |= 0b0001_0000;
        } else {
            self.f &= 0b1110_1111;
        }   
    }

    pub fn increase_register(&mut self, register: &str) {
        match register {
            "bc" => {
                self.set_bc(self.get_bc() + 1);
                return
            },
            "de" => {
                self.set_de(self.get_de() + 1);
                return
            },
            "hl" => {
                self.set_hl(self.get_hl() + 1);
                return
            },
            "sp" => {
                self.set_sp(self.sp + 1);
                return
            },
            _ => ()
        }

        let r: u16 = match register {
            "a" => self.a as u16,
            "b" => self.b as u16,
            "c" => self.c as u16,
            "d" => self.d as u16,
            "e" => self.e as u16,
            "h" => self.h as u16,
            "l" => self.l as u16,
            _ => panic!("Invalid register"),
        };
            
        let result: u16 = r + 1;
        self.flag_half_carry(((r&0xf) + 1)&0x10 == 0x10); 
        self.flag_carry(((r&0xff) + 1)&0x100 == 0x100);
        self.flag_zero(r == 0);
        self.flag_substract(false);
        
        match register {
            "a" => self.a = result.to_be_bytes()[0],
            "b" => self.b = result.to_be_bytes()[0],
            "c" => self.c = result.to_be_bytes()[0],
            "d" => self.d = result.to_be_bytes()[0],
            "e" => self.e = result.to_be_bytes()[0],
            "h" => self.h = result.to_be_bytes()[0],
            "l" => self.l = result.to_be_bytes()[0],
            _ => panic!("Invalid register"),
        };
    }

    pub fn decrease_register(&mut self, register: &str) {
        match register {
            "bc" => {
                self.set_bc(self.get_bc() - 1);
                return
            },
            "de" => {
                self.set_de(self.get_de() - 1);
                return
            },
            "hl" => {
                self.set_hl(self.get_hl() - 1);
                return
            },
            "sp" => {
                self.set_sp(self.sp + 1);
                return
            },
            _ => ()
        }

        let r: i16 = match register {
            "a" => self.a as i16,
            "b" => self.b as i16,
            "c" => self.c as i16,
            "d" => self.d as i16,
            "e" => self.e as i16,
            "h" => self.h as i16,
            "l" => self.l as i16,
            _ => panic!("Invalid register"),
        };
            
        let result: i16 = r - 1;
        self.flag_half_carry((r&0xf)  - 1 < 0); 
        self.flag_carry((r&0xff)  - 1 < 0);
        self.flag_zero(r == 0);
        self.flag_substract(true);
        
        match register {
            "a" => self.a = result.to_be_bytes()[0],
            "b" => self.b = result.to_be_bytes()[0],
            "c" => self.c = result.to_be_bytes()[0],
            "d" => self.d = result.to_be_bytes()[0],
            "e" => self.e = result.to_be_bytes()[0],
            "h" => self.h = result.to_be_bytes()[0],
            "l" => self.l = result.to_be_bytes()[0],
            _ => panic!("Invalid register"),
        };
    }

    /** rotation -> false left / true right */ 
    pub fn rotate_a_through_carry(&mut self, rotation: bool) -> u8 {
        let mut result;
        if rotation {
            result = self.a.rotate_left(1);
            if self.get_c_flag() {
                result |= 0b0000_0001;
            } else {
                result &= 0b1111_1110;
            }
            self.flag_carry(result & 1 == 1);
        } else {
            result = self.a.rotate_right(1);
            if self.get_c_flag() {
                result |= 0b1000_0000;
            } else {
                result &= 0b0111_1111;
            }
            self.flag_carry(result & 0b1000_0000 == 0b1000_0000);
        }
        
        self.flag_zero(false);
        self.flag_substract(false);
        self.flag_half_carry(false);
        self.a = result;

        4
    }

    /** rotation -> false left / true right */ 
    pub fn rotate_a(&mut self, rotation: bool) -> u8{
        if rotation {
            self.set_a(self.a.rotate_left(1));
            self.flag_carry(self.a & 1 == 1)
        } else {
            self.set_a(self.a.rotate_right(1));
            self.flag_carry(self.a & 0b1000_0000 == 0b1000_0000)
        }
        self.flag_zero(false);
        self.flag_substract(false);
        self.flag_half_carry(false);
        
        4
    }

    pub fn add_to_hl(&mut self, register: &str) -> u8 {
        let to_add = match register {
            "bc" => self.get_bc(),
            "de" => self.get_de(),
            "hl" => self.get_hl(),
            "sp" => self.sp,
            _ => panic!("Invalid register"),
        };
        let res = self.get_hl().overflowing_add(to_add);
        self.set_hl(res.0);
        self.flag_substract(false);
        self.flag_half_carry((res.0 & 0x1000) == 0x1000);
        self.flag_carry(res.1);
        
        8
    }

    pub fn relative_jump(&mut self, value: u8) -> u8 {
        self.pc += value as u16;
        12
    }

    pub fn conditional_relative_jump(&mut self, condition: &str, value: u8) -> u8 {
        let jump = match condition {
            "z" => self.get_z_flag(),
            "nz" => !self.get_z_flag(),
            "c" => self.get_c_flag(),
            "nc" => !self.get_c_flag(),
            _ => panic!("Invalid condition"),
        };
        if jump {
            self.relative_jump(value);
            12
        } else {
            8
        }
    }

    // TODO ??
    pub fn daa(&self) -> u8 {
        4
    }

    pub fn cpl(&mut self) -> u8 {
        self.set_a(self.get_a() ^ 0xFF);
        self.flag_substract(true);
        self.flag_half_carry(true);
        4
    }

    pub fn scf(&mut self) -> u8 {
        self.flag_carry(true);
        self.flag_substract(false);
        self.flag_half_carry(false);
        4
    }

    pub fn ccf(&mut self) -> u8 {
        self.flag_carry(!self.get_c_flag());
        self.flag_substract(false);
        self.flag_half_carry(false);
        4
    }

    // TODO ???
    pub fn halt(&mut self) -> u8 {
        4
    }

    pub fn add_sp(&mut self, value: u8) -> u8 {
        let res = self.sp.overflowing_add(value as u16);

        self.sp = res.0;
        self.flag_zero(false);
        self.flag_substract(false);
        self.flag_half_carry((((self.sp as u16 & 0xFFF) + (value as u16 & 0xFFF)) & 0x1000) == 0x1000);
        self.flag_carry(res.1);
        16
    }

    pub fn add(&mut self, value: u8) {
        let res = self.a.overflowing_add(value);
        self.flag_zero(res.0 == 0);
        self.flag_substract(false);
        self.flag_half_carry((self.a as i16 & 0xF) + (value as i16 & 0xF) > 0xF); 
        self.flag_carry(res.1);
        self.a = res.0;
    }

    pub fn adc(&mut self, value: u8) {
        let carry = if self.get_c_flag() { 1 } else { 0 };
        let result = self.a.overflowing_add(value.overflowing_add(carry).0);
        self.set_a(result.0);
        self.flag_zero(result.0 == 0);
        self.flag_substract(false);
        self.flag_half_carry((self.a as i16 & 0xF) + (value as i16 + carry as i16 & 0xF) > 0xF);
        self.flag_carry(result.1);
    }

    pub fn sub(&mut self, value: u8) {
        let res = self.a.overflowing_sub(value);
        self.flag_zero(res.0 == 0);
        self.flag_substract(true);
        self.flag_half_carry((self.a as i16 &0xf) - ( value as i16 &0xf) < 0); 
        self.flag_carry(res.1);
        self.a = res.0;
    }

    pub fn sbc(&mut self, value: u8) {
        let carry = if self.get_c_flag() { 1 } else { 0 };
        let result = self.a.overflowing_sub(value.overflowing_sub(carry).0);
        self.set_a(result.0);
        self.flag_zero(result.0 == 0);
        self.flag_substract(true);
        self.flag_half_carry((self.a as i16 &0xf) - ((value as i16 - carry as i16)&0xf) < 0);
        self.flag_carry(result.1);
    }

    pub fn and(&mut self, value: u8) {
        let result = self.a & value;
        self.flag_zero(result == 0);
        self.flag_substract(false);
        self.flag_half_carry(true);
        self.flag_carry(false);
        self.a = result;
    }

    pub fn xor(&mut self, value: u8) {
        let result = self.a ^ value;
        self.flag_zero(result == 0);
        self.flag_substract(false);
        self.flag_half_carry(false);
        self.flag_carry(false);
        self.a = result;
    }

    pub fn or(&mut self, value: u8) {
        let result = self.a | value;
        self.flag_zero(result == 0);
        self.flag_substract(false);
        self.flag_half_carry(false);
        self.flag_carry(false);
        self.a = result;
    }

    pub fn cp(&mut self, value: u8) {
        let res = self.a.overflowing_sub(value);

        self.flag_zero(res.0 == 0);
        self.flag_substract(true);
        self.flag_half_carry((self.a as i16 &0xf) - ( value as i16 &0xf) < 0);
        self.flag_carry(res.1);
    }

    pub fn rst(&mut self, value: u8) -> u8 {
        self.pc = value as u16;
        16
    }

    pub fn rlc(&mut self, registre: &str) {
        let value = match registre {
            "a" => self.get_a(),
            "b" => self.get_b(),
            "c" => self.get_c(),
            "d" => self.get_d(),
            "e" => self.get_e(),
            "h" => self.get_h(),
            "l" => self.get_l(),
            _ => panic!("Invalid register"),
        };
        let result = value.rotate_left(1);
        self.flag_zero(result == 0);
        self.flag_substract(false);
        self.flag_half_carry(false);
        self.flag_carry(result & 1 == 1);
        match registre {
            "a" => self.set_a(result),
            "b" => self.set_b(result),
            "c" => self.set_c(result),
            "d" => self.set_d(result),
            "e" => self.set_e(result),
            "h" => self.set_h(result),
            "l" => self.set_l(result),
            _ => panic!("Invalid register"),
        };
    }
    
    pub fn rrc(&mut self, registre: &str) {
        let value = match registre {
            "a" => self.get_a(),
            "b" => self.get_b(),
            "c" => self.get_c(),
            "d" => self.get_d(),
            "e" => self.get_e(),
            "h" => self.get_h(),
            "l" => self.get_l(),
            _ => panic!("Invalid register"),
        };
        let result = value.rotate_right(1);
        self.flag_zero(result == 0);
        self.flag_substract(false);
        self.flag_half_carry(false);
        self.flag_carry(result & 0b1000_0000 == 0b1000_0000);
        match registre {
            "a" => self.set_a(result),
            "b" => self.set_b(result),
            "c" => self.set_c(result),
            "d" => self.set_d(result),
            "e" => self.set_e(result),
            "h" => self.set_h(result),
            "l" => self.set_l(result),
            _ => panic!("Invalid register"),
        };
    }

    pub fn rl(&mut self, registre: &str) {
        let value = match registre {
            "a" => self.get_a(),
            "b" => self.get_b(),
            "c" => self.get_c(),
            "d" => self.get_d(),
            "e" => self.get_e(),
            "h" => self.get_h(),
            "l" => self.get_l(),
            _ => panic!("Invalid register"),
        };
        let mut result = value.rotate_left(1);
        if self.get_c_flag() {
            result |= 0b0000_0001;
        } else {
            result &= 0b1111_1110;
        }
        self.flag_zero(result == 0);
        self.flag_substract(false);
        self.flag_half_carry(false);
        self.flag_carry(result & 1 == 1);
        match registre {
            "a" => self.set_a(result),
            "b" => self.set_b(result),
            "c" => self.set_c(result),
            "d" => self.set_d(result),
            "e" => self.set_e(result),
            "h" => self.set_h(result),
            "l" => self.set_l(result),
            _ => panic!("Invalid register"),
        };
    }

    pub fn rr(&mut self, registre: &str) {
        let value = match registre {
            "a" => self.get_a(),
            "b" => self.get_b(),
            "c" => self.get_c(),
            "d" => self.get_d(),
            "e" => self.get_e(),
            "h" => self.get_h(),
            "l" => self.get_l(),
            _ => panic!("Invalid register"),
        };
        let mut result = value.rotate_right(1);
        if self.get_c_flag() {
            result |= 0b1000_0000;
        } else {
            result &= 0b0111_1111;
        }
        self.flag_zero(result == 0);
        self.flag_substract(false);
        self.flag_half_carry(false);
        self.flag_carry(result & 0b1000_0000 == 0b1000_0000);
        match registre {
            "a" => self.set_a(result),
            "b" => self.set_b(result),
            "c" => self.set_c(result),
            "d" => self.set_d(result),
            "e" => self.set_e(result),
            "h" => self.set_h(result),
            "l" => self.set_l(result),
            _ => panic!("Invalid register"),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_a_through_carry() {
        let mut cpu = LR35902::new();
        cpu.set_a(0b0100_0001);
        cpu.rotate_a_through_carry(true);
        assert_eq!(cpu.get_a(), 0b1000_0010);
        assert_eq!(cpu.f, 0b0000_0000);
        cpu.rotate_a_through_carry(true);
        assert_eq!(cpu.get_a(), 0b0000_0101);
        assert_eq!(cpu.f, 0b0001_0000);
        
        cpu.set_a(0b0100_0001);
        cpu.rotate_a_through_carry(false);
        assert_eq!(cpu.get_a(), 0b1010_0000);
        assert_eq!(cpu.f, 0b0001_0000);
        cpu.rotate_a_through_carry(false);
        assert_eq!(cpu.get_a(), 0b0101_0000);
        assert_eq!(cpu.f, 0b0000_0000);
    }

    #[test]
    fn test_decrease_register() {
        let mut cpu = LR35902::new();
        cpu.set_a(0b0100_0001);
        cpu.decrease_register("a");
        assert_eq!(cpu.get_a(), 0b0100_0000);
        assert_eq!(cpu.f, 0b0100_0000);
        cpu.decrease_register("a");
        assert_eq!(cpu.get_a(), 0b0011_1111);
        assert_eq!(cpu.f, 0b0100_0000);
    }

    #[test]
    fn test_increase_register() {
        let mut cpu = LR35902::new();
        cpu.set_a(0b0100_0001);
        cpu.increase_register("a");
        assert_eq!(cpu.get_a(), 0b0100_0010);
        assert_eq!(cpu.f, 0b0000_0000);
        cpu.increase_register("a");
        assert_eq!(cpu.get_a(), 0b0100_0011);
        assert_eq!(cpu.f, 0b0000_0000);
    }
}