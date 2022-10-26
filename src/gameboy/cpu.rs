use super::memory::Memory;

pub struct Cpu {
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
    halt: bool,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            f: 0,
            pc: 0,
            sp: 0,
            ime: false,
            halt: false,
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
            "f" => self.f,
            _ => panic!("Invalid register"),
        }
    }

    pub fn set_r(&mut self, r: &str, value: u8) {
        match r {
            "a" => self.a = value,
            "b" => self.b = value,
            "c" => self.c = value,
            "d" => self.d = value,
            "e" => self.e = value,
            "h" => self.h = value,
            "l" => self.l = value,
            "f" => self.f = value,
            _ => panic!("Invalid register"),
        }
    }

    pub fn get_rr(&mut self, rr: &str) -> u16 {
        match rr {
            "af" => ((self.a as u16) << 8) | self.f as u16,
            "bc" => ((self.b as u16) << 8) | self.c as u16,
            "de" => ((self.d as u16) << 8) | self.e as u16,
            "hl" => ((self.h as u16) << 8) | self.l as u16,
            "sp" => self.sp,
            "pc" => self.pc,
            _ => panic!("Invalid register"),
        }
    }

    pub fn set_rr(&mut self, rr: &str, value: u16) {
        let bytes = value.to_be_bytes();
        match rr {
            "af" => {
                self.a = bytes[0];
                self.f = bytes[1];
            },
            "bc" => {
                self.b = bytes[0];
                self.c = bytes[1];
            },
            "de" => {
                self.d = bytes[0];
                self.e = bytes[1];
            },
            "hl" => {
                self.h = bytes[0];
                self.l = bytes[1];
            },
            "sp" => self.sp = value,
            "pc" => self.pc = value,
            _ => panic!("Invalid register"),
        }
    }

    pub fn set_flag(&mut self, flag: &str, value: bool) {
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

    fn read_n(&mut self, memory: &mut Memory) -> u8 {
        let value = memory.read_byte(self.pc);
        self.pc = self.pc.wrapping_add(1);
        value
    }

    fn read_nn(&mut self, memory: &mut Memory) -> u16 {
        let low = self.read_n(memory);
        let high = self.read_n(memory);
        ((high as u16) << 8) | low as u16
    }

    pub fn fetch_opcode(&mut self, memory: &mut Memory) -> u8 {
        let opcode = memory.read_byte(self.pc);
        self.pc = self.pc.wrapping_add(1);
        opcode
    }

    pub fn cycle(&mut self, memory: &mut Memory) {
        if self.halt {
            return;
        }
       
        let opcode =self.fetch_opcode(memory);
        self.exectute(opcode, memory);
    }

    fn exectute(&mut self, opcode: u8, memory: &mut Memory) -> u8 {
        match opcode {
            0x31 => self.ld_rr_nn("sp", memory),
            0xaf => self.xor_s("a", memory),
            0x21 => self.ld_rr_nn("hl", memory),
            0x32 => self.ld_hld_a(memory),
            0xcb => self.cb_prefix(memory),
            0x20 => self.jr_cc_n("nz", memory),
            0x0e => self.ld_r_n("c", memory),
            0x3e => self.ld_r_n("a", memory),
            0xe2 => self.ld_c_a(memory),
            0x0c => self.inc_r("c"),
            0x77 => self.ld_rr_nn("hl", memory),
            0xe0 => self.ldh_n_a(memory),
            0x11 => self.ld_rr_nn("de", memory),
            0xa1 => self.and_s("a", memory),
            _ => panic!("Opcode {:02x} not implemented", opcode),
        }
    }

    fn cb_prefix(&mut self, memory: &mut Memory) -> u8 {
        let opcode = self.fetch_opcode(memory);
        match opcode {
            0x7c => self.bit_s(7, "h"),
            _ => panic!("CB Opcode {:02x} not implemented", opcode),
        }
    }

    /// Loads 2 bytes of immediate data to register pair rr.
    /// 
    /// ``` rust
    /// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
    /// # let mut memory = gameboy::gameboy::memory::Memory::new();
    /// # memory.write_byte(0x00, 0x21);
    /// # memory.write_byte(0x01, 0x5b);
    /// # memory.write_byte(0x02, 0x3a);
    /// //Example: LD HL, 0x3A5B ; H <- 0x3A, L <- 0x5B
    /// 
    /// cpu.cycle(&mut memory);
    /// assert_eq!(cpu.get_r("h"), 0x3a);
    /// assert_eq!(cpu.get_r("l"), 0x5b);
    /// ```
    fn ld_rr_nn(&mut self, rr: &str, memory: &mut Memory) -> u8 {
        let nn = self.read_nn(memory);
        self.set_rr(rr, nn);
        12
    }

    /// Takes the logical exclusive-OR for each bit of the contents of operand s and register A.
    /// And stores the results in register A. r, n, and (HL) are used for operand s.
    /// Z = _, N = 0, H = 0, C = 0
    /// ``` rust
    /// //Example: When A = 0xFF and (HL) = 0x8A
    /// //XOR A ; A <- 0x00, Z <- 1 
    /// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
    /// # let mut memory = gameboy::gameboy::memory::Memory::new();
    /// # memory.write_byte(0x00, 0xaf);
    /// # cpu.set_r("a", 0xff);
    /// cpu.cycle(&mut memory);
    /// assert_eq!(cpu.get_r("a"), 0x00);
    /// assert_eq!(cpu.get_r("f"), 0x80);
    /// 
    /// //XOR 0x0F ; A <- 0xF0, Z <- 0
    /// # cpu.set_rr("pc", 0x00);
    /// # memory.write_byte(0x00, 0xee);
    /// # cpu.set_r("a", 0xff);
    /// # memory.write_byte(0x01, 0x0f);
    /// cpu.cycle(&mut memory);
    /// assert_eq!(cpu.get_r("a"), 0xf0);
    /// assert_eq!(cpu.get_r("f"), 0x00);
    /// 
    /// //XOR (HL) ; A <- 75h, Z <- 0
    /// # cpu.set_rr("pc", 0x00);
    /// # memory.write_byte(0x00, 0xae);
    /// # cpu.set_r("a", 0xff);
    /// # cpu.set_rr("hl", 0x1000);
    /// # memory.write_byte(0x1000, 0x8a);
    /// cpu.cycle(&mut memory);
    /// assert_eq!(cpu.get_r("a"), 0x75);
    /// assert_eq!(cpu.get_r("f"), 0x00);
    /// ```
    fn xor_s(&mut self, s: &str, memory: &mut Memory) -> u8 {
        let value = match s {
            "hl" => memory.read_byte(self.get_rr("hl")),
            "n" => self.read_n(memory),
            _ => self.get_r(s),
        };

        self.a ^= value;
        self.set_flag("z", self.a == 0);
        self.set_flag("n", false);
        self.set_flag("h", false);
        self.set_flag("c", false);
    
        4
    }

    /// Stores the contents of register A in the memory specified by register pair HL and simultaneously decrements the contents of HL.
    /// ``` rust
    /// //Example: HL = 0x4000 and A = 0x05,
    /// //LD (HLD), A ; (0x4000) <- 0x05, HL = 0x3FFF
    /// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
    /// # let mut memory = gameboy::gameboy::memory::Memory::new();
    /// # memory.write_byte(0x00, 0x32);
    /// # cpu.set_r("a", 0x05);
    /// # cpu.set_rr("hl", 0x4000);
    /// cpu.cycle(&mut memory);
    /// assert_eq!(memory.read_byte(0x4000), 0x05);
    /// assert_eq!(cpu.get_rr("hl"), 0x3fff);
    /// ```
    fn ld_hld_a(&mut self, memory: &mut Memory) -> u8 {
        memory.write_byte(self.get_rr("hl"), self.a);
        let hl = self.get_rr("hl");
        self.set_rr("hl", hl.wrapping_sub(1));

        8
    }

    /// Copies the complement of the contents of the specified bit in register r to the Z flag of the program status word (PSW).
    /// The codes for b and r are as follows.
    /// 
    /// ``` rust
    /// //Examples: When A = 0x80 and L = 0xEF
    /// //BIT 7, A ; Z <- 0, N <- 0, H <- 1
    /// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
    /// # let mut memory = gameboy::gameboy::memory::Memory::new();
    /// # memory.write_byte(0x00, 0xcb);
    /// # memory.write_byte(0x01, 0x7f);
    /// # cpu.set_r("a", 0x80);
    /// cpu.cycle(&mut memory);
    /// assert_eq!(cpu.get_r("f"), 0x20);
    /// 
    /// //BIT 4, L ; Z <- 1, N <- 0, H <- 1
    /// # cpu.set_rr("pc", 0x00);
    /// # cpu.set_r("l", 0xef);
    /// # memory.write_byte(0x00, 0xcb);
    /// # memory.write_byte(0x01, 0x65);
    /// cpu.cycle(&mut memory);
    /// assert_eq!(cpu.get_r("f"), 0xa0);
    /// ```
    fn bit_s(&mut self, b: u8, s: &str) -> u8 {

        self.set_flag("z", self.get_r(s) & (1 << b) == 0);
        self.set_flag("n", false);
        self.set_flag("h", true);

        8
    }

    /// Jumps -127 to +129 steps from the current address.
    /// ``` rust
    /// //JR NZ, 0x05 ; PC <- PC + 0x05
    /// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
    /// # let mut memory = gameboy::gameboy::memory::Memory::new();
    /// # memory.write_byte(0x00, 0x20);
    /// # memory.write_byte(0x01, 0x05);
    /// # cpu.set_r("f", 0b0000);
    /// cpu.cycle(&mut memory);
    /// assert_eq!(cpu.get_rr("pc"), 0x07);
    /// 
    /// //JR C, 0x80 ; PC <- PC - 127
    /// # cpu.set_rr("pc", 0x100);
    /// # memory.write_byte(0x100, 0x20);
    /// # memory.write_byte(0x101, 0x80);
    /// # cpu.set_r("f", 0b0001);
    /// cpu.cycle(&mut memory);
    /// assert_eq!(cpu.get_rr("pc"), 0x082);
    /// ```
    fn jr_cc_n(&mut self, cc: &str, memory: &mut Memory) -> u8 {
        let condition = match cc {
            "nz" => self.f & 0x80 == 0,
            "z" => self.f & 0x80 != 0,
            "nc" => self.f & 0x10 == 0,
            "c" => self.f & 0x10 != 0,
            _ => panic!("Invalid condition"),
        };
        
        let n = self.read_n(memory) as i8;
        let result = self.pc.wrapping_add(n as u16);
        
        if condition {
            self.pc = result;
            12
        } else {
            8
        }
        
    }

    fn jr_n(&mut self, memory: &mut Memory) -> u8 {
        let n = self.read_n(memory) as i8;
        let pc = self.pc;
        self.pc = pc.wrapping_add(n as u16);
        12
    }

    /// Sets the interrupt master enable flag and enables maskable interrupts.
    /// This instruction can be used in an interrupt routine to enable higher-order interrupts.
    /// ``` rust
    ///     //EI ; IME <- 1
    /// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
    /// # let mut memory = gameboy::gameboy::memory::Memory::new();
    /// # memory.write_byte(0x00, 0xfb);
    /// cpu.cycle(&mut memory);
    /// assert_eq!(cpu.get_ime(), true);
    /// ```
    fn ei(&mut self) -> u8 {
        self.ime = true;
        4
    }

    /// Loads 8-bit immediate data n into register r.
    /// ``` rust
    /// //Example: LD B, 0x24 ; B <- 0x24
    /// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
    /// # let mut memory = gameboy::gameboy::memory::Memory::new();
    /// # memory.write_byte(0x00, 0x06);
    /// # memory.write_byte(0x01, 0x24);
    /// cpu.cycle(&mut memory);
    /// assert_eq!(cpu.get_r("b"), 0x24);
    /// ```
    fn ld_r_n(&mut self, r: &str, memory: &mut Memory) -> u8 {
        let n = self.read_n(memory);
        self.set_r(r, n);
        8
    }

    /// Loads the contents of register A in the internal RAM, port register, or mode register at the address in the range FFOOh-FFFFh specified by register C.
    /// ``` rust
    /// //Example: When C = 0x9F, A = 0x24
    /// //LD (C), A ; (0xFF9F) <- A
    /// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
    /// # let mut memory = gameboy::gameboy::memory::Memory::new();
    /// # memory.write_byte(0x00, 0xe2);
    /// # cpu.set_r("c", 0x9f);
    /// # cpu.set_r("a", 0x24);
    /// cpu.cycle(&mut memory);
    /// assert_eq!(memory.read_byte(0xff9f), 0x24);
    /// ```
    fn ld_c_a(&mut self, memory: &mut Memory) -> u8 {
        memory.write_byte(0xff00 + self.get_r("c") as u16, self.get_r("a"));
        8
    }

    /// Increments the contents of register r by 1 .
    /// ``` rust
    /// //Example: When A = 0xFF,
    /// //INC A ; A <- 0, Z <- 1, N <- 0, H <- 1
    /// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
    /// # let mut memory = gameboy::gameboy::memory::Memory::new();
    /// # memory.write_byte(0x00, 0x3c);
    /// # cpu.set_r("a", 0xff);
    /// cpu.cycle(&mut memory);
    /// assert_eq!(cpu.get_r("a"), 0);
    /// assert_eq!(cpu.get_r("f"), 0xa0);
    /// ```
    fn inc_r(&mut self, r: &str) -> u8 {
        let value = self.get_r(r);
        let result = value.wrapping_add(1);
        self.set_r(r, result);

        self.set_flag("z", result == 0);
        self.set_flag("n", false);
        self.set_flag("h", value & 0xf == 0xf);

        4
    }

    /// Takes the one’s complement of the contents of register A.
    /// ``` rust
    /// //Example: When A = 0x35,
    /// //CPL ; A <- 0xCA
    /// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
    /// # let mut memory = gameboy::gameboy::memory::Memory::new();
    /// # memory.write_byte(0x00, 0x2f);
    /// # cpu.set_r("a", 0x35);
    /// cpu.cycle(&mut memory);
    /// assert_eq!(cpu.get_r("a"), 0xca);
    /// assert_eq!(cpu.get_r("f"), 0x60);
    /// ```
    fn cpl(&mut self) -> u8 {
        let value = self.get_r("a");
        self.set_r("a", !value);
        self.set_flag("n", true);
        self.set_flag("h", true);
        4
    }

    /// Stores the contents of register A in the memory specified by register pair DE.
    /// ``` rust
    /// // Example: When DE = 0x205 and A = 0x00,
    /// // LD (DE) , A ; (0x205) <- 0xOO
    /// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
    /// # let mut memory = gameboy::gameboy::memory::Memory::new();
    /// # memory.write_byte(0x00, 0x12);
    /// # cpu.set_rr("de", 0x205);
    /// # cpu.set_r("a", 0x00);
    /// cpu.cycle(&mut memory);
    /// assert_eq!(memory.read_byte(0x205), 0x00);
    /// ```
    fn ld_de_a(&mut self, memory: &mut Memory) -> u8 {
        memory.write_byte(self.get_rr("de"), self.get_r("a"));
        8
    }

    /// Compares the contents of operand s and register A and sets the flag if they are equal, r, n, and (HL) are used for operand s.
    /// ```rust
    /// //Examples: When A = 0x3C, B = 0x2F, and (HL) = 0x40,
    /// //CP B ; Z <- 0, N <- 1, H <- 1, CY <- 0
    /// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
    /// # let mut memory = gameboy::gameboy::memory::Memory::new();
    /// # memory.write_byte(0x00, 0xb8);
    /// # cpu.set_r("a", 0x3c);
    /// # cpu.set_r("b", 0x2f);
    /// cpu.cycle(&mut memory);
    /// assert_eq!(cpu.get_r("f"), 0x60);
    /// //CP 0x3C ; Z <- 1, N <- 1, H <- 0, CY <- 0
    /// # cpu.set_rr("pc", 0x00);
    /// # memory.write_byte(0x00, 0xfe);
    /// # memory.write_byte(0x01, 0x3c);
    /// cpu.cycle(&mut memory);
    /// assert_eq!(cpu.get_r("f"), 0xc0);
    /// //CP (HL) ; Z <- 0, N <- 1, H <- 0 , CY <- 1
    /// # cpu.set_rr("pc", 0x00);
    /// # memory.write_byte(0x00, 0xbe);
    /// # cpu.set_rr("hl", 0x100);
    /// # memory.write_byte(0x100, 0x40);
    /// cpu.cycle(&mut memory);
    /// assert_eq!(cpu.get_r("f"), 0x50);
    /// ```
    fn cp_s(&mut self, s: &str, memory: &mut Memory) -> u8 {
        let value = match s {
            "hl" => memory.read_byte(self.get_rr("hl")),
            "n" => self.read_n(memory),
            _ => self.get_r(s),
        };

        let result = self.a.wrapping_sub(value);
        self.set_flag("z", result == 0);
        self.set_flag("n", true);
        self.set_flag("h", value & 0xf > self.a & 0xf);
        self.set_flag("c", value > self.a);

        4
    }

    /// Adds the contents of operand s and CY to the contents of register A and stores the results in register A. r, n, and (HL) are used for operand s.
    /// ``` rust
    /// //Examples: When A = 0xE1, E = 0x0f, (HL) = 0x1e, and CY = 1 
    /// //ADC A, E ; A <- 0xf1, Z <- 0, H <- 1 , CY <- 0 
    /// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
    /// # let mut memory = gameboy::gameboy::memory::Memory::new();
    /// # memory.write_byte(0x00, 0x8b);
    /// # cpu.set_r("a", 0xe1);
    /// # cpu.set_r("e", 0x0f);
    /// # cpu.set_flag("c", true);
    /// cpu.cycle(&mut memory);
    /// assert_eq!(cpu.get_r("a"), 0xf1);
    /// assert_eq!(cpu.get_r("f"), 0x20);
    /// //ADC A, 0x3B ; A <- 0x1D, Z <- 0, H <- 0, CY <- 1 
    /// # cpu.set_rr("pc", 0x00);
    /// # cpu.set_r("a", 0xe1);
    /// # memory.write_byte(0x00, 0xce);
    /// # memory.write_byte(0x01, 0x3b);
    /// # cpu.set_flag("c", true);
    /// cpu.cycle(&mut memory);
    /// assert_eq!(cpu.get_r("a"), 0x1d);
    /// assert_eq!(cpu.get_r("f"), 0x10);
    /// //ADC A, (HL) ; A <- 0x00, Z <- 1, H <- 1, CY <- 1
    /// # cpu.set_rr("pc", 0x00);
    /// # cpu.set_r("a", 0xe1);
    /// # cpu.set_rr("hl", 0x100);
    /// # memory.write_byte(0x100, 0x1e);
    /// # memory.write_byte(0x00, 0x8e);
    /// # cpu.set_flag("c", true);
    /// cpu.cycle(&mut memory);
    /// assert_eq!(cpu.get_r("a"), 0x00);
    /// assert_eq!(cpu.get_r("f"), 0xb0);
    /// ```
    fn adc_s(&mut self, s: &str, memory: &mut Memory) -> u8 {
        let value: u8 = match s {
            "hl" => memory.read_byte(self.get_rr("hl")),
            "n" => self.read_n(memory),
            _ => self.get_r(s),
        };

        let carry = (self.f & 0x10 == 0x10) as u8 ;
        let result = self.a.wrapping_add(value).wrapping_add(carry);

        self.set_flag("z", result == 0);
        self.set_flag("n", false);
        self.set_flag("h", (self.a & 0xf) + (value & 0xf) + carry > 0xf);
        self.set_flag("c", self.a as u16 + value as u16 + carry as u16 > 0xff);

        self.a = result;

        4
    }

    /// Increments the contents of register pair rr by 1.
    /// ```rust
    /// //Example: When DE = 0x235f
    /// //INC DE ; DE <- 0x2360
    /// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
    /// # let mut memory = gameboy::gameboy::memory::Memory::new();
    /// # memory.write_byte(0x00, 0x13);
    /// # cpu.set_rr("de", 0x235f);
    /// cpu.cycle(&mut memory);
    /// assert_eq!(cpu.get_rr("de"), 0x2360);
    /// ```
    fn inc_rr(&mut self, rr: &str) -> u8 {
        let value = self.get_rr(rr);
        self.set_rr(rr, value.wrapping_add(1));
        8
    }

    /// Loads into 0xffnn the contents of the register A.
    /// ```rust
    /// //Example: When n = 0x12, A = 0x34
    /// //LDH (d8), A ; (0xFF12) <- A
    /// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
    /// # let mut memory = gameboy::gameboy::memory::Memory::new();
    /// # memory.write_byte(0x00, 0xe0);
    /// # memory.write_byte(0x01, 0x12);
    /// # cpu.set_r("a", 0x34);
    /// cpu.cycle(&mut memory);
    /// assert_eq!(memory.read_byte(0xff12), 0x34);
    /// ```
    fn ldh_n_a(&mut self, memory: &mut Memory) -> u8 {
        let addr = u16::from_be_bytes([0xff, self.read_n(memory)]);
        memory.write_byte(addr, self.a);

        12
    }
    
    fn and_s(&mut self, s: &str, memory: &mut Memory) -> u8 {
        let value = match s {
            "hl" => memory.read_byte(self.get_rr("hl")),
            "n" => self.read_n(memory),
            _ => self.get_r(s),
        };

        self.a &= value;
        self.set_flag("z", self.a == 0);
        self.set_flag("n", false);
        self.set_flag("h", true);
        self.set_flag("c", false);

        4
    }
}