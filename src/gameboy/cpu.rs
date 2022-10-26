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
                "z" => self.f |= 0x80,
                "n" => self.f |= 0x40,
                "h" => self.f |= 0x20,
                "c" => self.f |= 0x10,
                _ => panic!("Invalid flag"),
            }
        } else {
            match flag {
                "z" => self.f &= 0x7f,
                "n" => self.f &= 0xbf,
                "h" => self.f &= 0xdf,
                "c" => self.f &= 0xef,
                _ => panic!("Invalid flag"),
            }
        }
    }

    pub fn get_flag(&mut self, flag: &str) -> bool{
        match flag {
            "z" => self.f & 0x80 == 0x80,
            "n" => self.f & 0x40 == 0x40,
            "h" => self.f & 0x20 == 0x20,
            "c" => self.f & 0x10 == 0x10,
            _ => panic!("Invalid flag"),
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
            0x1a => self.ld_a_de(memory),
            0xcd => self.call_nn(memory),
            0x4f => self.ld_r_r("c", "a"),
            0x06 => self.ld_r_n("b", memory),
            0xc5 => self.push_rr("bc", memory),
            0x17 => self.rla(),
            0xc1 => self.pop_rr("bc", memory),
            0x05 => self.dec_r("b"),
            0x22 => self.ld_hli_a(memory),
            0x23 => self.inc_rr("hl"),
            0xc9 => self.ret(memory),
            0x13 => self.inc_rr("de"),
            0x7b => self.ld_r_r("a", "e"), 
            0xfe => self.cp_s("n", memory),
            0xea => self.ld_nn_a(memory),
            0x3d => self.dec_r("a"),
            0x28 => self.jr_cc_n("z", memory),
            0x0d => self.dec_r("c"),
            0x2e => self.ld_r_n("l", memory),
            0x18 => self.jr_n(memory),
            0x67 => self.ld_r_r("h", "a"),
            0x57 => self.ld_r_r("d", "a"),
            0x04 => self.inc_r("b"),
            0x1e => self.ld_r_n("e", memory),
            0xf0 => self.ldh_a_n(memory),
            0x1d => self.dec_r("e"),
            0x24 => self.inc_r("h"), 
            0x7c => self.ld_r_r("a", "h"),
            0x90 => self.sub_s("b", memory),
            0x93 => self.sub_s("e", memory),
            0xd6 => self.sub_s("n", memory),
            0x96 => self.sub_s("hl", memory),
            0x15 => self.dec_r("d"),
            0x16 => self.ld_r_n("d", memory),
            0xbe => self.cp_s("hl", memory),
            _ => panic!("Opcode {:02x} not implemented", opcode),
        }
    }

    fn cb_prefix(&mut self, memory: &mut Memory) -> u8 {
        let opcode = self.fetch_opcode(memory);
        match opcode {
            0x7c => self.bit_s(7, "h"),
            0x11 => self.rl_m("c", memory),
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
    
    /// Loads the contents specified by the contents of register pair DE into register A.
    /// ```rust	
    /// //Example: When (DE) = 0x5F,
    /// //LD A, (DE) ; A <- 0x5F
    /// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
    /// # let mut memory = gameboy::gameboy::memory::Memory::new();
    /// # memory.write_byte(0x00, 0x1a);
    /// # cpu.set_rr("de", 0x100);
    /// # memory.write_byte(0x100, 0x5f);
    /// cpu.cycle(&mut memory);
    /// assert_eq!(cpu.get_r("a"), 0x5f);
    /// ```
    fn ld_a_de(&mut self, memory: &mut Memory) -> u8 {
        self.a = memory.read_byte(self.get_rr("de"));
        8
    }

    /// In memory, pushes the PC value corresponding to the instruction at the address following that of the
    /// CALL instruction to the 2 bytes following the byte specified by the current SP.
    /// Operand nn is then loaded in the PC.
    /// ```rust
    /// //Examples: When PC = 0x8000 and SP = 0xFFFE,
    /// //CALL 0x1234; (0xFFDH) <- 0x80, (0xFFCH) <- 0x03, SP <- 0xFFCH, PC <- 0x1234
    /// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
    /// # let mut memory = gameboy::gameboy::memory::Memory::new();
    /// # memory.write_byte(0x8000, 0xcd);
    /// # cpu.set_rr("pc", 0x8000);
    /// # cpu.set_rr("sp", 0xfffe);
    /// # memory.write_byte(0x8001, 0x34);
    /// # memory.write_byte(0x8002, 0x12);
    /// cpu.cycle(&mut memory);
    /// assert_eq!(memory.read_byte(0xfffd), 0x80);
    /// assert_eq!(memory.read_byte(0xfffc), 0x03);
    /// assert_eq!(cpu.get_rr("sp"), 0xfffc);
    /// assert_eq!(cpu.get_rr("pc"), 0x1234);
    /// ```
    fn call_nn(&mut self, memory: &mut Memory) -> u8 {
        let addr = self.read_nn(memory);
        self.push_rr("pc", memory);
        self.pc = addr;
        24
    }

    /// Loads the contents of register r2 into register r1.
    /// ```rust
    /// //Examples:
    /// //LD A, B ; A <- B
    /// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
    /// # let mut memory = gameboy::gameboy::memory::Memory::new();
    /// # memory.write_byte(0x00, 0x78);
    /// # cpu.set_r("b", 0x5f);
    /// cpu.cycle(&mut memory);
    /// assert_eq!(cpu.get_r("a"), 0x5f);
    /// 
    /// // LD B, D ; B <- D
    /// memory.write_byte(0x01, 0x58);
    /// cpu.set_r("d", 0x12);
    /// cpu.cycle(&mut memory);
    /// assert_eq!(cpu.get_r("b"), 0x12);
    /// ```
    fn ld_r_r(&mut self, r1: &str, r2: &str) -> u8 {
        self.set_r(r1, self.get_r(r2));
        4
    }

    /// Pushes the contents of register pair rr onto the memory stack.
    /// ```rust
    /// //Example: When SP = 0xFFFE,
    /// //PUSH BC ; (0xFFFC) <- C, (0xFFFD) <- B, SP <- 0xFFFC
    /// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
    /// # let mut memory = gameboy::gameboy::memory::Memory::new();
    /// # memory.write_byte(0x00, 0xc5);
    /// # cpu.set_rr("sp", 0xfffe);
    /// # cpu.set_rr("bc", 0x1234);
    /// cpu.cycle(&mut memory);
    /// assert_eq!(memory.read_byte(0xfffd), 0x12);
    /// assert_eq!(memory.read_byte(0xfffc), 0x34);
    /// assert_eq!(cpu.get_rr("sp"), 0xfffc);
    /// ```
    fn push_rr(&mut self, rr: &str, memory: &mut Memory) -> u8 {
        let value = self.get_rr(rr).to_be_bytes();
        memory.write_byte(self.sp.wrapping_sub(1), value[0]);
        memory.write_byte(self.sp.wrapping_sub(2), value[1]);
        self.sp = self.sp.wrapping_sub(2);
        16
    }

    /// Rotates the contents of operand m to the left, r and (HL) are used for operand m.
    /// ```rust
    /// //Examples: When C = 0x80, (HL) = 0x11, and CY = 0,
    /// //RL C ; C <- 0x00, Z <- 1, N <- 0, H <- 0, CY <- 1
    /// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
    /// # let mut memory = gameboy::gameboy::memory::Memory::new();
    /// # memory.write_byte(0x00, 0xcb);
    /// # memory.write_byte(0x01, 0x11);
    /// # cpu.set_r("c", 0x80);
    /// # cpu.set_flag("c", false);
    /// cpu.cycle(&mut memory);
    /// assert_eq!(cpu.get_r("c"), 0x00);
    /// assert_eq!(cpu.get_r("f"), 0x90);
    /// //RL (HL) ; (HL) <- 0x22,  Z <- 0, N <- 0, H <- 0, CY <- 0
    /// # memory.write_byte(0x02, 0xcb);
    /// # memory.write_byte(0x03, 0x16);
    /// # cpu.set_rr("hl", 0x100);
    /// # memory.write_byte(0x100, 0x11);
    /// cpu.cycle(&mut memory);
    /// assert_eq!(memory.read_byte(0x100), 0x22);
    /// assert_eq!(cpu.get_r("f"), 0x00);
    /// ```
    fn rl_m(&mut self, m: &str, memory: &mut Memory) -> u8 {
        let value = match m {
            "hl" => memory.read_byte(self.get_rr("hl")),
            _ => self.get_r(m),
        };
        let carry = self.get_flag("c");
        let result = (value << 1) | carry as u8;

        self.set_flag("z", result == 0);
        self.set_flag("n", false);
        self.set_flag("h", false);
        self.set_flag("c", value >> 7 == 1);

        if m == "hl" {
            memory.write_byte(self.get_rr("hl"), result);
            return 16;
        } 
        
        self.set_r(m, result);
        8
    }

    /// Rotates the contents of register A to the left.
    /// ```rust	
    /// //Example: When A = 0x95 and CY = 1,
    /// //RLA ; A <- 0x2B, Z <- 0, N <- 0, H <- 0, C <- 1,
    /// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
    /// # let mut memory = gameboy::gameboy::memory::Memory::new();
    /// # memory.write_byte(0x00, 0x17);
    /// # cpu.set_r("a", 0x95);
    /// # cpu.set_flag("c", true);
    /// cpu.cycle(&mut memory);
    /// assert_eq!(cpu.get_r("a"), 0x2b);
    /// assert_eq!(cpu.get_r("f"), 0x10);
    /// ```
    fn rla(&mut self) -> u8 {
        let value = self.a;
        let carry = self.get_flag("c");
        let result = (value << 1) | carry as u8;

        self.set_flag("z", false);
        self.set_flag("n", false);
        self.set_flag("h", false);
        self.set_flag("c", value >> 7 == 1);

        self.a = result;
        4
    }

    /// Pops contents from the memory stack and into register pair rr.
    /// ```rust
    /// //Example: When SP = 0xFFFC, (0xFFFC) = 0x5F, and (0xFFFD) = 0x3C,
    /// //POP BC ; B <- 0x3C, C <- 0x5F, SP <- 0xFFFE
    /// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
    /// # let mut memory = gameboy::gameboy::memory::Memory::new();
    /// # memory.write_byte(0x00, 0xc1);
    /// # cpu.set_rr("sp", 0xfffc);
    /// # memory.write_byte(0xfffc, 0x5f);
    /// # memory.write_byte(0xfffd, 0x3c);
    /// cpu.cycle(&mut memory);
    /// assert_eq!(cpu.get_rr("bc"), 0x3c5f);
    /// assert_eq!(cpu.get_rr("sp"), 0xfffe);
    /// ```
    fn pop_rr(&mut self, rr: &str, memory: &mut Memory) -> u8 {
        let low = memory.read_byte(self.sp);
        let high = memory.read_byte(self.sp.wrapping_add(1));
        self.set_rr(rr, u16::from_be_bytes([high, low]));
        self.sp = self.sp.wrapping_add(2);
        
        12
    }

    /// Subtract 1 from the contents of register r by 1.
    /// ```rust
    /// //Example: When B = 0x01,
    /// //DEC B ; B <- 0, Z <- 1, N <— 1 H <- 0,
    /// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
    /// # let mut memory = gameboy::gameboy::memory::Memory::new();
    /// # memory.write_byte(0x00, 0x05);
    /// # cpu.set_r("b", 0x01);
    /// cpu.cycle(&mut memory);
    /// assert_eq!(cpu.get_r("b"), 0x00);
    /// assert_eq!(cpu.get_r("f"), 0xc0);
    /// ```
    fn dec_r(&mut self, r: &str) -> u8 {
        let value = self.get_r(r);
        let result = value.wrapping_sub(1);

        self.set_flag("z", result == 0);
        self.set_flag("n", true);
        self.set_flag("h", (value & 0x0f) == 0);

        self.set_r(r, result);
        4
    }

    /// Stores the contents of register A in the memory specified by register pair HL and simultaneously increments the contents of HL.
    /// ```rust
    /// //Example: When HL = 0xFFFF and A = 0x56,
    /// //LD (HLI), A ; (0xFFFF) <- 0x56, HL = 0x0000
    /// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
    /// # let mut memory = gameboy::gameboy::memory::Memory::new();
    /// # memory.write_byte(0x00, 0x22);
    /// # cpu.set_rr("hl", 0xffff);
    /// # cpu.set_r("a", 0x56);
    /// cpu.cycle(&mut memory);
    /// assert_eq!(memory.read_byte(0xffff), 0x56);
    /// assert_eq!(cpu.get_rr("hl"), 0x0000);
    /// ```
    fn ld_hli_a(&mut self, memory: &mut Memory) -> u8 {
        let hl = self.get_rr("hl");
        memory.write_byte(hl, self.a);
        self.set_rr("hl", hl.wrapping_add(1));

        8
    }

    /// Pops from the memory stack the PC value pushed when the subroutine was called, returning control to the source program.
    /// ```rust
    /// // Examples: When PC = 0x8000; (0x9000) = 0xc9;
    /// // CALL 0x9000; PC = 0x9000
    /// //RET ; Returns to address 0x8003
    /// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
    /// # let mut memory = gameboy::gameboy::memory::Memory::new();
    /// # memory.write_byte(0x8000, 0xcd);
    /// # memory.write_byte(0x8001, 0x00);
    /// # memory.write_byte(0x8002, 0x90);
    /// # memory.write_byte(0x9000, 0xc9);
    /// # cpu.set_rr("pc", 0x8000);
    /// cpu.cycle(&mut memory);
    /// assert_eq!(cpu.get_rr("pc"), 0x9000);
    /// cpu.cycle(&mut memory);
    /// println!("{:x}", memory.read_byte(cpu.get_rr("sp")));
    /// println!("{:x}", memory.read_byte(cpu.get_rr("sp") + 1));
    /// assert_eq!(cpu.get_rr("pc"), 0x8003);
    /// ```
    fn ret(&mut self, memory: &mut Memory) -> u8 {
        self.pop_rr("pc", memory);

        16
    }

    /// Loads the contents of register A to the internal RAM or register specified by 16-bit immediate operand nn.
    /// ```rust
    /// //Example: LD (0xFF44), A ; (LY) <- A
    /// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
    /// # let mut memory = gameboy::gameboy::memory::Memory::new();
    /// # memory.write_byte(0x00, 0xea);
    /// # memory.write_byte(0x01, 0x44);
    /// # memory.write_byte(0x02, 0xff);
    /// # cpu.set_r("a", 0x56);
    /// cpu.cycle(&mut memory);
    /// assert_eq!(memory.read_byte(0xff44), 0x56);
    /// // LD (0x8000), A ; (0x8000) <- A
    /// # memory.write_byte(0x03, 0xea);
    /// # memory.write_byte(0x04, 0x00);    
    /// # memory.write_byte(0x05, 0x80);
    /// cpu.cycle(&mut memory);
    /// assert_eq!(memory.read_byte(0x8000), 0x56);
    /// ```
    fn ld_nn_a(&mut self, memory: &mut Memory) -> u8 {
        let addr = self.read_nn(memory);
        memory.write_byte(addr, self.a);

        16
    }

    fn ldh_a_n(&mut self, memory: &mut Memory) -> u8 {
        let addr = 0xff00 + u16::from(self.read_n(memory));
        self.a = memory.read_byte(addr);

        12
    }

    /// Subtracts the contents of operand s from the contents of register A and stores the results in register A. r, n, and (HL) are used for operand s.
    /// ```rust
    /// //Examples: When A = 0x3E, E = 0x3E, and (HL) = 0x40,
    /// //SUB E ; A <- 0x00, Z <-1, N <- 1, H <- 0, CY <- 0
    /// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
    /// # let mut memory = gameboy::gameboy::memory::Memory::new();
    /// # memory.write_byte(0x00, 0x93);
    /// # cpu.set_r("a", 0x3e);
    /// # cpu.set_r("e", 0x3e);
    /// cpu.cycle(&mut memory);
    /// assert_eq!(cpu.get_r("a"), 0x00);
    /// assert_eq!(cpu.get_r("f"), 0xc0 );
    /// //SUB 0x0f ; A <- 0x2F, Z <- 0, N <- 1, H <- 1, CY <- 0
    /// # memory.write_byte(0x01, 0xd6);
    /// # memory.write_byte(0x02, 0x0f);
    /// # cpu.set_r("a", 0x3e);
    /// cpu.cycle(&mut memory);
    /// assert_eq!(cpu.get_r("a"), 0x2f);
    /// assert_eq!(cpu.get_r("f"), 0x60);
    /// //SUB (HL) ; A <- 0xFE, Z <- 0, N <- 1, H <- 0, CY <— 1
    /// # memory.write_byte(0x03, 0x96);
    /// # cpu.set_r("a", 0x3e);
    /// # cpu.set_rr("hl", 0x50);
    /// # memory.write_byte(0x50, 0x40);
    /// cpu.cycle(&mut memory);
    /// assert_eq!(cpu.get_r("a"), 0xfe);
    /// assert_eq!(cpu.get_r("f"), 0x50);
    /// ```
    fn sub_s(&mut self, s: &str, memory: &mut Memory) -> u8 {
        let value = match s {
            "hl" => memory.read_byte(self.get_rr("hl")),
            "n" => self.read_n(memory),
            _ => self.get_r(s),
        };
       
        let result = self.a.wrapping_sub(value);
        
        self.set_flag("z", result == 0);
        self.set_flag("n", true);
        self.set_flag("h", (self.a & 0xf).wrapping_sub(value & 0xf) & (0x10) != 0);
        self.set_flag("c", (self.a as u16) < (value as u16));

        self.a = result;

        if s == "hl" || s == "n" {
            return 8;
        }

        4
    }
}