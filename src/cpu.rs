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
    pc: u16, // Program counter
    sp: u16, // Stack pointer
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
            pc: 0x0100, // Starting at cartridge first instruction bc no bios
            sp: 0,
        }
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

    // todo overflow? half carry?
    pub fn increase_register(&mut self, register: &str) {
        match register {
            "a" => {
                //(a + a + 1 > 0xFF) -->> carry / overflow
                self.a += 1;
                self.flag_zero(self.a == 0);
                self.flag_substract(false);
            },
            "b" => {
                self.b += 1;
                self.flag_zero(self.b == 0);
                self.flag_substract(false);
            },
            "c" => {
                self.c += 1;
                self.flag_zero(self.c == 0);
                self.flag_substract(false);
            },
            "d" => {
                self.d += 1;
                self.flag_zero(self.d == 0);
                self.flag_substract(false);
            },
            "e" => {
                self.e += 1;
                self.flag_zero(self.e == 0);
                self.flag_substract(false);
            },
            "bc" => {
                self.set_bc(self.get_bc() + 1);
            },
            "de" => {
                self.set_de(self.get_de() + 1);
            },
            "hl" => {
                self.set_hl(self.get_hl() + 1);
            },
            "sp" => self.sp += 1,
            _ => panic!("Invalid register"),
        }
    }

    // todo overflow? half carry?
    pub fn decrease_register(&mut self, register: &str) {
        match register {
            "a" => {
                self.a -= 1;
                self.flag_zero(self.a == 0);
                self.flag_substract(true);
            },
            "b" => {
                self.b -= 1;
                self.flag_zero(self.b == 0);
                self.flag_substract(true);
            },
            "c" => {
                self.c -= 1;
                self.flag_zero(self.c == 0);
                self.flag_substract(true);
            },
            "d" => {
                self.d -= 1;
                self.flag_zero(self.d == 0);
                self.flag_substract(true);
            },
            "e" => {
                self.e -= 1;
                self.flag_zero(self.e == 0);
                self.flag_substract(true);
            },
            "bc" => {
                self.set_bc(self.get_bc() - 1);
            },
            "de" => {
                self.set_de(self.get_de() - 1);
            },
            "hl" => {
                self.set_hl(self.get_hl() - 1);
            },
            "sp" => self.sp -= 1,
            _ => panic!("Invalid register"),
        }
    }

    /** rotation -> false left / true right */ 
    pub fn rotate_a_through_carry(&mut self, rotation: bool) -> u8 {
        if rotation {
            let furthest_left_bit = (0b1000_0000 & self.get_a()) > 0;
            self.flag_carry(furthest_left_bit);
            self.set_a(self.get_a().rotate_left(1));
        } else {
            let furthest_right_bit = (0b0000_0001 & self.get_a()) > 0;
            self.flag_carry(furthest_right_bit);
            self.set_a(self.get_a().rotate_right(1));
        }
        self.flag_zero(false);
        self.flag_substract(false);
        self.flag_half_carry(false);
        
        4
    }

    /**
     * rotation -> false left / true right
     * TODO half carry 
    */ 
    pub fn rotate_a(&mut self, rotation: bool) -> u8{
        if rotation {
            self.set_a(self.get_a().rotate_left(1));
        } else {
            self.set_a(self.get_a().rotate_right(1));
        }
        self.flag_zero(false);
        self.flag_substract(false);
        self.flag_half_carry(false);
        
        4
    }

    // todo overflow? half carry?
    pub fn add_to_hl(&mut self, register: &str) -> u8 {
        let to_add = match register {
            "bc" => self.get_bc(),
            "de" => self.get_de(),
            "hl" => self.get_hl(),
            "sp" => self.sp,
            _ => panic!("Invalid register"),
        };
        self.set_hl(self.get_hl() +  to_add);
        self.flag_substract(false);
        
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

    // TODO overflow? half carry?
    pub fn add(&mut self, value: u8) {
        let result = self.get_a() + value;
        self.set_a(result);
        self.flag_zero(result == 0);
        self.flag_half_carry((self.get_a() & 0xF) + (value & 0xF) > 0xF);
        self.flag_substract(false);
    }

    pub fn adc(&mut self, value: u8) {
        let carry = if self.get_c_flag() { 1 } else { 0 };
        let result = self.get_a() + value + carry;
        self.set_a(result);
        self.flag_zero(result == 0);
        self.flag_half_carry((self.get_a() & 0xF) + (value & 0xF) + carry > 0xF);
        self.flag_substract(false);
    }

    pub fn sub(&mut self, value: u8) {
        let result = self.get_a() - value;
        self.set_a(result);
        self.flag_zero(result == 0);
        self.flag_half_carry((self.get_a() & 0xF) < (value & 0xF));
        self.flag_substract(true);
    }

    pub fn sbc(&mut self, value: u8) {
        let carry = if self.get_c_flag() { 1 } else { 0 };
        let result = self.get_a() - value - carry;
        self.set_a(result);
        self.flag_zero(result == 0);
        self.flag_half_carry((self.get_a() & 0xF) < (value & 0xF) + carry);
        self.flag_substract(true);
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

    // TODO carry?
    pub fn cp(&mut self, value: u8) {
        let result = self.get_a() - value;
        self.flag_zero(result == 0);
        self.flag_substract(true);
        self.flag_half_carry((self.get_a() & 0xF) < (value & 0xF));
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