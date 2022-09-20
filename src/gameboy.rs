use super::memory::MEMORY;
use super::rom::ROM;
use super::cpu::LR35902;

pub struct GAMEBOY {
    cpu: LR35902,
    memory: MEMORY,
    rom: ROM,
}

impl GAMEBOY {
    pub fn new() -> GAMEBOY {
        GAMEBOY {
            cpu: LR35902::new(),
            memory: MEMORY::new(),
            rom: ROM::new(),
        }
    }

    pub fn load_rom(&mut self, rom_path: &str) {
        self.rom.load_rom(rom_path);
        self.memory.load_rom(self.rom.fetch_rom_bank());
    }

    pub fn fetch_opcode(&mut self) -> u16 {
        self.rom.fetch_opcode(self.cpu.get_pc())
    }

    pub fn execute_opcode(&mut self, opcode: u16) -> u8 {
        match opcode {
            0x00 => return 4, // NOP
            0x01 => { // LD BC, d16
                let d16 = self.read_d16();
                self.cpu.set_bc(d16);
                return 12
            },
            0x02 => { // LD (BC), A
                self.memory.write_byte(self.cpu.get_bc(), self.cpu.get_a());
                return 8
            },
            0x03 => { // INC BC
                self.cpu.increase_register("bc");
                return 8
            },
            0x04 => { // INC B
                self.cpu.increase_register("b");
                return 4
            },
            0x05 => { // DEC B
                self.cpu.decrease_register("b");
                return 4
            },
            0x06 => { // LD B, d8
                let d8 = self.read_d8();
                self.cpu.set_b(d8);
                return 8
            },
            0x07 => { return self.cpu.rotate_a_through_carry(false) }, // RLCA 
            0x08 => { // LD (a16), SP
                let d16 = self.read_d16();
                let sp:[u8; 2] = self.cpu.get_sp().to_be_bytes();
                self.memory.write_byte(d16, sp[0]);
                self.memory.write_byte(d16 + 1, sp[1]);
                return 20
            },
            0x09 => { return self.cpu.add_to_hl("bc") }, // ADD HL, BC  
            0x0A => { // LD A, (BC)
                self.cpu.set_a(self.memory.read_byte(self.cpu.get_bc()));
                return 8
            },
            0x0B => { // DEC BC
                self.cpu.decrease_register("bc");
                return 8
            },
            0x0C => { // INC C
                self.cpu.increase_register("c");
                return 4
            },
            0x0D => { // DEC C
                self.cpu.decrease_register("c");
                return 4
            },
            0x0E => { // LD C, d8
                let d8 = self.read_d8();
                self.cpu.set_c(d8);
                return 8
            },
            0x0F => { return self.cpu.rotate_a_through_carry(true) }, // RRCA


            0x10 => { // STOP
                return 4
            },
            0x11 => { // LD DE, d16
                let d16 = self.read_d16();
                self.cpu.set_de(d16);
                return 12
            },
            0x12 => { // LD (DE), A
                self.memory.write_byte(self.cpu.get_de(), self.cpu.get_a());
                return 8
            },
            0x13 => { // INC DE
                self.cpu.increase_register("de");
                return 8
            },
            0x14 => { // INC D
                self.cpu.increase_register("d");
                return 4
            },
            0x15 => { // DEC D
                self.cpu.decrease_register("d");
                return 4
            },
            0x16 => { // LD D, d8
                let d8 = self.read_d8();
                self.cpu.set_d(d8);
                return 8
            },
            0x17 => { return self.cpu.rotate_a(false) }, // RLA
            0x18 => { // JR r8
                let d8 = self.read_d8();
                return self.cpu.relative_jump(d8) 
            },
            0x19 => { return self.cpu.add_to_hl("de") }, // ADD HL, DE
            0x1A => { // LD A, (DE)
                self.cpu.set_a(self.memory.read_byte(self.cpu.get_de()));
                return 8
            },
            0x1B => { // DEC DE
                self.cpu.decrease_register("de");
                return 8
            },
            0x1C => { // INC E
                self.cpu.increase_register("e");
                return 4
            },
            0x1D => { // DEC E
                self.cpu.decrease_register("e");
                return 4
            },
            0x1E => { // LD E, d8
                let d8 = self.read_d8();
                self.cpu.set_e(d8);
                return 8
            },
            0x1F => { return self.cpu.rotate_a(true) }, // RRA

            0x20 => { // JR NZ, r8
                let d8 = self.read_d8();
                return self.cpu.conditional_relative_jump("nz", d8)
            },
            0x21 => { // LD HL, d16
                let d16 = self.read_d16();
                self.cpu.set_hl(d16);
                return 12
            },
            0x22 => { // LD (HL+), A or LDI (HL),A
                self.memory.write_byte(self.cpu.get_hl(), self.cpu.get_a());
                self.cpu.increase_register("hl");
                return 8
            },
            0x23 => { // INC HL
                self.cpu.increase_register("hl");
                return 8
            },
            0x24 => { // INC H
                self.cpu.increase_register("h");
                return 4
            },
            0x25 => { // DEC H
                self.cpu.decrease_register("h");
                return 4
            },
            0x26 => { // LD H, d8
                let d8 = self.read_d8();
                self.cpu.set_h(d8);
                return 8
            },
            0x27 => { return self.cpu.daa() },
            0x28 => { // JR Z, r8
                let d8 = self.read_d8();
                return self.cpu.conditional_relative_jump("z", d8)
            },
            0x29 => { return self.cpu.add_to_hl("hl") }, // ADD HL, HL
            0x2A => { // LD A, (HL+) or LDI A, (HL)
                self.cpu.set_a(self.memory.read_byte(self.cpu.get_hl()));
                self.cpu.increase_register("hl");
                return 8
            },
            0x2B => { // DEC HL
                self.cpu.decrease_register("hl");
                return 8
            },
            0x2C => { // INC L
                self.cpu.increase_register("l");
                return 4
            },
            0x2D => { // DEC L
                self.cpu.decrease_register("l");
                return 4
            },
            0x2E => { // LD L, d8
                let d8 = self.read_d8();
                self.cpu.set_l(d8);
                return 8
            },
            0x2F => { return self.cpu.cpl() }, // CPL


            0x30 => { // JR NC, r8
                let d8 = self.read_d8();
                return self.cpu.conditional_relative_jump("nc", d8)
            },
            0x31 => { // LD SP, d16
                let d16 = self.read_d16();
                self.cpu.set_sp(d16);
                return 12
            },
            0x32 => { // LD (HL-), A or LDD (HL),A
                self.memory.write_byte(self.cpu.get_hl(), self.cpu.get_a());
                self.cpu.decrease_register("hl");
                return 8
            },
            0x33 => { // INC SP
                self.cpu.increase_register("sp");
                return 8
            },
            0x34 => { // INC (HL)
                let hl = self.cpu.get_hl();
                let value = self.memory.read_byte(hl);
                self.memory.write_byte(hl, value + 1);
                return 12
            },
            0x35 => { // DEC (HL)
                let hl = self.cpu.get_hl();
                let value = self.memory.read_byte(hl);
                self.memory.write_byte(hl, value - 1);
                return 12
            },
            0x36 => { // LD (HL), d8
                let d8 = self.read_d8();
                self.memory.write_byte(self.cpu.get_hl(), d8);
                return 12
            },
            0x37 => { return self.cpu.scf() }, // SCF
            0x38 => { // JR C, r8
                let d8 = self.read_d8();
                return self.cpu.conditional_relative_jump("c", d8)
            },
            0x39 => { return self.cpu.add_to_hl("sp") }, // ADD HL, SP
            0x3A => { // LD A, (HL-) or LDD A, (HL)
                self.cpu.set_a(self.memory.read_byte(self.cpu.get_hl()));
                self.cpu.decrease_register("hl");
                return 8
            },
            0x3B => { // DEC SP
                self.cpu.decrease_register("sp");
                return 8
            },
            0x3C => { // INC A
                self.cpu.increase_register("a");
                return 4
            },
            0x3D => { // DEC A
                self.cpu.decrease_register("a");
                return 4
            },
            0x3E => { // LD A, d8
                let d8 = self.read_d8();
                self.cpu.set_a(d8);
                return 8
            },
            0x3F => { return self.cpu.ccf() }, // CCF


            0x40 => { // LD B, B
                self.cpu.set_b(self.cpu.get_b()); // ???
                return 4 
            }, 
            0x41 => { // LD B, C
                self.cpu.set_b(self.cpu.get_c());
                return 4
            },
            0x42 => { // LD B, D
                self.cpu.set_b(self.cpu.get_d());
                return 4
            },
            0x43 => { // LD B, E
                self.cpu.set_b(self.cpu.get_e());
                return 4
            },
            0x44 => { // LD B, H
                self.cpu.set_b(self.cpu.get_h());
                return 4
            },
            0x45 => { // LD B, L
                self.cpu.set_b(self.cpu.get_l());
                return 4
            },
            0x46 => { // LD B, (HL)
                self.cpu.set_b(self.memory.read_byte(self.cpu.get_hl()));
                return 8
            },
            0x47 => { // LD B, A
                self.cpu.set_b(self.cpu.get_a());
                return 4
            },
            0x48 => { // LD C, B
                self.cpu.set_c(self.cpu.get_b());
                return 4
            },
            0x49 => { // LD C, C
                self.cpu.set_c(self.cpu.get_c());
                return 4
            },
            0x4A => { // LD C, D
                self.cpu.set_c(self.cpu.get_d());
                return 4
            },
            0x4B => { // LD C, E
                self.cpu.set_c(self.cpu.get_e());
                return 4
            },
            0x4C => { // LD C, H
                self.cpu.set_c(self.cpu.get_h());
                return 4
            },
            0x4D => { // LD C, L
                self.cpu.set_c(self.cpu.get_l());
                return 4
            },
            0x4E => { // LD C, (HL)
                self.cpu.set_c(self.memory.read_byte(self.cpu.get_hl()));
                return 8
            },
            0x4F => { // LD C, A
                self.cpu.set_c(self.cpu.get_a());
                return 4
            },


            0x50 => { // LD D, B
                self.cpu.set_d(self.cpu.get_b());
                return 4
            },
            0x51 => { // LD D, C
                self.cpu.set_d(self.cpu.get_c());
                return 4
            },
            0x52 => { // LD D, D
                self.cpu.set_d(self.cpu.get_d());
                return 4
            },
            0x53 => { // LD D, E
                self.cpu.set_d(self.cpu.get_e());
                return 4
            },
            0x54 => { // LD D, H
                self.cpu.set_d(self.cpu.get_h());
                return 4
            },
            0x55 => { // LD D, L
                self.cpu.set_d(self.cpu.get_l());
                return 4
            },
            0x56 => { // LD D, (HL)
                self.cpu.set_d(self.memory.read_byte(self.cpu.get_hl()));
                return 8
            },
            0x57 => { // LD D, A
                self.cpu.set_d(self.cpu.get_a());
                return 4
            },
            0x58 => { // LD E, B
                self.cpu.set_e(self.cpu.get_b());
                return 4
            },
            0x59 => { // LD E, C
                self.cpu.set_e(self.cpu.get_c());
                return 4
            },
            0x5A => { // LD E, D
                self.cpu.set_e(self.cpu.get_d());
                return 4
            },
            0x5B => { // LD E, E
                self.cpu.set_e(self.cpu.get_e());
                return 4
            },
            0x5C => { // LD E, H
                self.cpu.set_e(self.cpu.get_h());
                return 4
            },
            0x5D => { // LD E, L
                self.cpu.set_e(self.cpu.get_l());
                return 4
            },
            0x5E => { // LD E, (HL)
                self.cpu.set_e(self.memory.read_byte(self.cpu.get_hl()));
                return 8
            },
            0x5F => { // LD E, A
                self.cpu.set_e(self.cpu.get_a());
                return 4
            },


            0x60 => { // LD H, B
                self.cpu.set_h(self.cpu.get_b());
                return 4
            },
            0x61 => { // LD H, C
                self.cpu.set_h(self.cpu.get_c());
                return 4
            },
            0x62 => { // LD H, D
                self.cpu.set_h(self.cpu.get_d());
                return 4
            },
            0x63 => { // LD H, E
                self.cpu.set_h(self.cpu.get_e());
                return 4
            },
            0x64 => { // LD H, H
                self.cpu.set_h(self.cpu.get_h());
                return 4
            },
            0x65 => { // LD H, L
                self.cpu.set_h(self.cpu.get_l());
                return 4
            },
            0x66 => { // LD H, (HL)
                self.cpu.set_h(self.memory.read_byte(self.cpu.get_hl()));
                return 8
            },
            0x67 => { // LD H, A
                self.cpu.set_h(self.cpu.get_a());
                return 4
            },
            0x68 => { // LD L, B
                self.cpu.set_l(self.cpu.get_b());
                return 4
            },
            0x69 => { // LD L, C
                self.cpu.set_l(self.cpu.get_c());
                return 4
            },
            0x6A => { // LD L, D
                self.cpu.set_l(self.cpu.get_d());
                return 4
            },
            0x6B => { // LD L, E
                self.cpu.set_l(self.cpu.get_e());
                return 4
            },
            0x6C => { // LD L, H
                self.cpu.set_l(self.cpu.get_h());
                return 4
            },
            0x6D => { // LD L, L
                self.cpu.set_l(self.cpu.get_l());
                return 4
            },
            0x6E => { // LD L, (HL)
                self.cpu.set_l(self.memory.read_byte(self.cpu.get_hl()));
                return 8
            },
            0x6F => { // LD L, A
                self.cpu.set_l(self.cpu.get_a());
                return 4
            },

            0x70 => { // LD (HL), B
                self.memory.write_byte(self.cpu.get_hl(), self.cpu.get_b());
                return 8
            },
            0x71 => { // LD (HL), C
                self.memory.write_byte(self.cpu.get_hl(), self.cpu.get_c());
                return 8
            },
            0x72 => { // LD (HL), D
                self.memory.write_byte(self.cpu.get_hl(), self.cpu.get_d());
                return 8
            },
            0x73 => { // LD (HL), E
                self.memory.write_byte(self.cpu.get_hl(), self.cpu.get_e());
                return 8
            },
            0x74 => { // LD (HL), H
                self.memory.write_byte(self.cpu.get_hl(), self.cpu.get_h());
                return 8
            },
            0x75 => { // LD (HL), L
                self.memory.write_byte(self.cpu.get_hl(), self.cpu.get_l());
                return 8
            },
            0x76 => { return self.cpu.halt() }, // HALT
            0x77 => { // LD (HL), A
                self.memory.write_byte(self.cpu.get_hl(), self.cpu.get_a());
                return 8
            },
            0x78 => { // LD A, B
                self.cpu.set_a(self.cpu.get_b());
                return 4
            },
            0x79 => { // LD A, C
                self.cpu.set_a(self.cpu.get_c());
                return 4
            },
            0x7A => { // LD A, D
                self.cpu.set_a(self.cpu.get_d());
                return 4
            },
            0x7B => { // LD A, E
                self.cpu.set_a(self.cpu.get_e());
                return 4
            },
            0x7C => { // LD A, H
                self.cpu.set_a(self.cpu.get_h());
                return 4
            },
            0x7D => { // LD A, L
                self.cpu.set_a(self.cpu.get_l());
                return 4
            },
            0x7E => { // LD A, (HL)
                self.cpu.set_a(self.memory.read_byte(self.cpu.get_hl()));
                return 8
            },
            0x7F => { // LD A, A
                self.cpu.set_a(self.cpu.get_a());
                return 4
            },


            0x80 => { // ADD A, B
                self.cpu.add(self.cpu.get_b()); 
                return 4
            }, 
            0x81 => { // ADD A, C
                self.cpu.add(self.cpu.get_c());
                return 4
            },
            0x82 => { // ADD A, D
                self.cpu.add(self.cpu.get_d());
                return 4
            },
            0x83 => { // ADD A, E
                self.cpu.add(self.cpu.get_e());
                return 4
            },
            0x84 => { // ADD A, H
                self.cpu.add(self.cpu.get_h());
                return 4
            },
            0x85 => { // ADD A, L
                self.cpu.add(self.cpu.get_l());
                return 4
            },
            0x86 => { // ADD A, (HL)
                self.cpu.add(self.memory.read_byte(self.cpu.get_hl()));
                return 8
            },
            0x87 => { // ADD A, A
                self.cpu.add(self.cpu.get_a());
                return 4
            },
            0x88 => { // ADC A, B
                self.cpu.adc(self.cpu.get_b());
                return 4
            },
            0x89 => { // ADC A, C
                self.cpu.adc(self.cpu.get_c());
                return 4
            },
            0x8A => { // ADC A, D
                self.cpu.adc(self.cpu.get_d());
                return 4
            },
            0x8B => { // ADC A, E
                self.cpu.adc(self.cpu.get_e());
                return 4
            },
            0x8C => { // ADC A, H
                self.cpu.adc(self.cpu.get_h());
                return 4
            },
            0x8D => { // ADC A, L
                self.cpu.adc(self.cpu.get_l());
                return 4
            },
            0x8E => { // ADC A, (HL)
                self.cpu.adc(self.memory.read_byte(self.cpu.get_hl()));
                return 8
            },
            0x8F => { // ADC A, A
                self.cpu.adc(self.cpu.get_a());
                return 4
            },


            0x90 => { // SUB B
                self.cpu.sub(self.cpu.get_b());
                return 4
            },
            0x91 => { // SUB C
                self.cpu.sub(self.cpu.get_c());
                return 4
            },
            0x92 => { // SUB D
                self.cpu.sub(self.cpu.get_d());
                return 4
            },
            0x93 => { // SUB E
                self.cpu.sub(self.cpu.get_e());
                return 4
            },
            0x94 => { // SUB H
                self.cpu.sub(self.cpu.get_h());
                return 4
            },
            0x95 => { // SUB L
                self.cpu.sub(self.cpu.get_l());
                return 4
            },
            0x96 => { // SUB (HL)
                self.cpu.sub(self.memory.read_byte(self.cpu.get_hl()));
                return 8
            },
            0x97 => { // SUB A
                self.cpu.sub(self.cpu.get_a());
                return 4
            },
            0x98 => { // SBC A, B
                self.cpu.sbc(self.cpu.get_b());
                return 4
            },
            0x99 => { // SBC A, C
                self.cpu.sbc(self.cpu.get_c());
                return 4
            },
            0x9A => { // SBC A, D
                self.cpu.sbc(self.cpu.get_d());
                return 4
            },
            0x9B => { // SBC A, E
                self.cpu.sbc(self.cpu.get_e());
                return 4
            },
            0x9C => { // SBC A, H
                self.cpu.sbc(self.cpu.get_h());
                return 4
            },
            0x9D => { // SBC A, L
                self.cpu.sbc(self.cpu.get_l());
                return 4
            },
            0x9E => { // SBC A, (HL)
                self.cpu.sbc(self.memory.read_byte(self.cpu.get_hl()));
                return 8
            },
            0x9F => { // SBC A, A
                self.cpu.sbc(self.cpu.get_a());
                return 4
            },


            0xA0 => { // AND B
                self.cpu.and(self.cpu.get_b());
                return 4
            },
            0xA1 => { // AND C
                self.cpu.and(self.cpu.get_c());
                return 4
            },
            0xA2 => { // AND D
                self.cpu.and(self.cpu.get_d());
                return 4
            },
            0xA3 => { // AND E
                self.cpu.and(self.cpu.get_e());
                return 4
            },
            0xA4 => { // AND H
                self.cpu.and(self.cpu.get_h());
                return 4
            },
            0xA5 => { // AND L
                self.cpu.and(self.cpu.get_l());
                return 4
            },
            0xA6 => { // AND (HL)
                self.cpu.and(self.memory.read_byte(self.cpu.get_hl()));
                return 8
            },
            0xA7 => { // AND A
                self.cpu.and(self.cpu.get_a());
                return 4
            },
            0xA8 => { // XOR B
                self.cpu.xor(self.cpu.get_b());
                return 4
            },
            0xA9 => { // XOR C
                self.cpu.xor(self.cpu.get_c());
                return 4
            },
            0xAA => { // XOR D
                self.cpu.xor(self.cpu.get_d());
                return 4
            },
            0xAB => { // XOR E
                self.cpu.xor(self.cpu.get_e());
                return 4
            },
            0xAC => { // XOR H
                self.cpu.xor(self.cpu.get_h());
                return 4
            },
            0xAD => { // XOR L
                self.cpu.xor(self.cpu.get_l());
                return 4
            },
            0xAE => { // XOR (HL)
                self.cpu.xor(self.memory.read_byte(self.cpu.get_hl()));
                return 8
            },
            0xAF => { // XOR A
                self.cpu.xor(self.cpu.get_a());
                return 4
            },


            0xB0 => { // OR B
                self.cpu.or(self.cpu.get_b());
                return 4
            },
            0xB1 => { // OR C
                self.cpu.or(self.cpu.get_c());
                return 4
            },
            0xB2 => { // OR D
                self.cpu.or(self.cpu.get_d());
                return 4
            },
            0xB3 => { // OR E
                self.cpu.or(self.cpu.get_e());
                return 4
            },
            0xB4 => { // OR H
                self.cpu.or(self.cpu.get_h());
                return 4
            },
            0xB5 => { // OR L
                self.cpu.or(self.cpu.get_l());
                return 4
            },
            0xB6 => { // OR (HL)
                self.cpu.or(self.memory.read_byte(self.cpu.get_hl()));
                return 8
            },
            0xB7 => { // OR A
                self.cpu.or(self.cpu.get_a());
                return 4
            },
            0xB8 => { // CP B
                self.cpu.cp(self.cpu.get_b());
                return 4
            },
            0xB9 => { // CP C
                self.cpu.cp(self.cpu.get_c());
                return 4
            },
            0xBA => { // CP D
                self.cpu.cp(self.cpu.get_d());
                return 4
            },
            0xBB => { // CP E
                self.cpu.cp(self.cpu.get_e());
                return 4
            },
            0xBC => { // CP H
                self.cpu.cp(self.cpu.get_h());
                return 4
            },
            0xBD => { // CP L
                self.cpu.cp(self.cpu.get_l());
                return 4
            },
            0xBE => { // CP (HL)
                self.cpu.cp(self.memory.read_byte(self.cpu.get_hl()));
                return 8
            },
            0xBF => { // CP A
                self.cpu.cp(self.cpu.get_a());
                return 4
            },


            0xC0 => { // RET NZ
                if !self.cpu.get_z_flag() {
                    let value = self.pop_stack();
                    self.cpu.set_pc(value);
                    return 20;
                }
                8
            }, 
            0xC1 => { // POP BC
                let value = self.pop_stack();
                self.cpu.set_bc(value);
                return 12
            },
            0xC2 => { // JP NZ, a16
                if !self.cpu.get_z_flag() {
                    let d16 = self.read_d16();
                    self.cpu.set_pc(d16);
                }
                12
            },
            0xC3 => { // JP a16
                let d16 = self.read_d16();
                self.cpu.set_pc(d16);
                return 16
            },
            0xC4 => { // CALL NZ, a16
                if !self.cpu.get_z_flag() {
                    let pc = self.cpu.get_pc();
                    self.push_stack(pc);
                    let d16 = self.read_d16();  
                    self.cpu.set_pc(d16);
                    return 24;
                }
                12
            },
            0xC5 => { // PUSH BC
                self.push_stack(self.cpu.get_bc());
                return 16
            },
            0xC6 => { // ADD A, d8
                let d8 = self.read_d8();
                self.cpu.add(d8);
                return 8
            },
            _ => panic!("Unimplemented opcode: {:X}", opcode),
        }
    }

    fn read_d16(&mut self) -> u16 {
        let mut bytes: [u8; 2] = [0,0];
        for i in 0..2 {
            bytes[i] = self.memory.read_byte(self.cpu.get_pc());
        }

        u16::from_be_bytes(bytes)
    }

    fn read_d8(&mut self) -> u8 {
        self.memory.read_byte(self.cpu.get_pc())
    }

    pub fn pop_stack(&mut self) -> u16 {
        let mut bytes: [u8; 2] = [0,0];
        for i in 0..2 {
            bytes[i] = self.memory.read_byte(self.cpu.get_sp());
            let sp = self.cpu.get_sp();
            self.cpu.set_sp(sp + 1);
        }
        u16::from_be_bytes(bytes)
    }

    pub fn push_stack(&mut self, value: u16) {
        let bytes = value.to_be_bytes();
        for i in 0..2 {
            let sp = self.cpu.get_sp();
            self.cpu.set_sp(sp - 1);
            self.memory.write_byte(sp, bytes[i]);
        }
    }
}