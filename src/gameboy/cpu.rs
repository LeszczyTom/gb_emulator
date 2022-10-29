mod gmb_8_bit_loadcommands;
mod gmb_16_bit_loadcommands;
mod gmb_8_bit_arithmetic_logical_commands;
mod gmb_16_bit_arithmetic_logical_commands;
mod gmb_rotate_and_shift_commands;
mod gmb_singlebit_operation_commands;
mod gmb_cpu_controlcommands;
mod gmb_jumpcommands;

use crate::gameboy::memory::Memory;
use crate::gameboy::cpu::{ 
    gmb_8_bit_loadcommands::*, 
    gmb_16_bit_loadcommands::*,
    gmb_8_bit_arithmetic_logical_commands::*,
    gmb_16_bit_arithmetic_logical_commands::*,
    gmb_rotate_and_shift_commands::*,
    gmb_singlebit_operation_commands::*,
    gmb_jumpcommands::*,
    gmb_cpu_controlcommands::*,
};
use Register::*;
use RegisterPair::*;
use Flag::*;

#[derive(Clone)]
pub enum Register {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    F,
}

#[derive(Clone)]
pub enum RegisterPair {
    AF,
    BC,
    DE,
    HL,
    SP,
    PC,
}

pub enum Flag {
    Zero,
    Subtract,
    HalfCarry,
    Carry,
    NCarry,
    NZero,
}

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

    pub fn set_a(&mut self, value: u8) { self.a = value; }
    pub fn set_b(&mut self, value: u8) { self.b = value; }
    pub fn set_c(&mut self, value: u8) { self.c = value; }
    pub fn set_d(&mut self, value: u8) { self.d = value; }
    pub fn set_e(&mut self, value: u8) { self.e = value; }
    pub fn set_h(&mut self, value: u8) { self.h = value; }
    pub fn set_l(&mut self, value: u8) { self.l = value; }
    pub fn set_f(&mut self, value: u8) { self.f = value; }
    pub fn set_pc(&mut self, value: u16) { self.pc = value; }
    pub fn set_sp(&mut self, value: u16) { self.sp = value; }
    pub fn set_ime(&mut self, value: bool) { self.ime = value; }

    pub fn get_a(&self) -> u8 { self.a }
    pub fn get_b(&self) -> u8 { self.b }
    pub fn get_c(&self) -> u8 { self.c }
    pub fn get_d(&self) -> u8 { self.d }
    pub fn get_e(&self) -> u8 { self.e }
    pub fn get_h(&self) -> u8 { self.h }
    pub fn get_l(&self) -> u8 { self.l }
    pub fn get_f(&self) -> u8 { self.f }
    pub fn get_af(&self) -> u16 { self.get_rr(AF) }
    pub fn get_bc(&self) -> u16 { self.get_rr(BC) }
    pub fn get_de(&self) -> u16 { self.get_rr(DE) }
    pub fn get_hl(&self) -> u16 { self.get_rr(HL) }
    pub fn get_pc(&self) -> u16 { self.pc }
    pub fn get_sp(&self) -> u16 { self.sp }
    pub fn get_ime(&self) -> bool { self.ime }

    pub fn get_r(&self, r: Register) -> u8 {
        match r {
            A => self.a,
            B => self.b,
            C => self.c,
            D => self.d,
            E => self.e,
            H => self.h,
            L => self.l,
            F => self.f,
        }
    }

    pub fn set_r(&mut self, r: Register, value: u8) {
        match r {
            A => self.a = value,
            B => self.b = value,
            C => self.c = value,
            D => self.d = value,
            E => self.e = value,
            H => self.h = value,
            L => self.l = value,
            F => self.f = value,
        }
    }

    pub fn get_rr(&self, rr: RegisterPair) -> u16 {
        match rr {
            AF => u16::from_be_bytes([self.a, self.f]),
            BC => u16::from_be_bytes([self.b, self.c]),
            DE => u16::from_be_bytes([self.d, self.e]),
            HL => u16::from_be_bytes([self.h, self.l]),
            SP => self.sp,
            PC => self.pc,
        }
    }

    pub fn set_rr(&mut self, rr: RegisterPair, value: u16) {
        let bytes = value.to_be_bytes();
        match rr {
            AF => {
                self.a = bytes[0];
                self.f = bytes[1];
            },
            BC => {
                self.b = bytes[0];
                self.c = bytes[1];
            },
            DE => {
                self.d = bytes[0];
                self.e = bytes[1];
            },
            HL => {
                self.h = bytes[0];
                self.l = bytes[1];
            },
            SP => self.sp = value,
            PC => self.pc = value,
        }
    }

    pub fn set_flag(&mut self, flag: Flag, value: bool) {
        if value {      
            match flag {
                Zero => self.f |= 0x80,
                Subtract => self.f |= 0x40,
                HalfCarry => self.f |= 0x20,
                Carry => self.f |= 0x10,
                _ => panic!("Invalid flag"),
            }
        } else {
            match flag {
                Zero => self.f &= 0x7f,
                Subtract => self.f &= 0xbf,
                HalfCarry => self.f &= 0xdf,
                Carry => self.f &= 0xef,
                _ => panic!("Invalid flag"),
            }
        }
    }       

    pub fn get_flag(&mut self, flag: Flag) -> bool{
        match flag {
            Zero => self.f & 0x80 == 0x80,
            Subtract => self.f & 0x40 == 0x40,
            HalfCarry => self.f & 0x20 == 0x20,
            Carry => self.f & 0x10 == 0x10,
            NCarry => self.get_flag(Carry) == false,
            NZero => self.get_flag(Zero) == false,
        }
    }

    pub fn get_halt(&self) -> bool {
        self.halt
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

    pub fn cycle(&mut self, memory: &mut Memory) -> u8 {
        let opcode =self.fetch_opcode(memory);
        self.exectute(opcode, memory)
    }

    fn exectute(&mut self, opcode: u8, memory: &mut Memory) -> u8 {
        match opcode {
            0x00 => 4,
            0x01 => ld_rr_nn(BC, self, memory),
            0x02 => ld_bc_a(self, memory),
            0x03 => inc_rr(BC, self),
            0x04 => inc_r(B, self),
            0x05 => dec_r(B, self),
            0x06 => ld_r_n(B, self, memory),
            0x07 => rlca(self),
            0x08 => ld_nn_sp(self, memory),
            0x09 => add_hl_rr(BC, self),
            0x0a => ld_a_bc(self, memory),
            0x0b => dec_rr(BC, self),
            0x0c => inc_r(C, self),
            0x0d => dec_r(C, self),
            0x0e => ld_r_n(C, self, memory),
            0x0f => rrca(self),

            0x10 => stop(),
            0x11 => ld_rr_nn(DE, self, memory),
            0x12 => ld_de_a(self, memory),
            0x13 => inc_rr(DE, self),
            0x14 => inc_r(D, self),
            0x15 => dec_r(D, self),
            0x16 => ld_r_n(D, self, memory),
            0x17 => rla(self),
            0x18 => jr_n(self, memory),
            0x19 => add_hl_rr(DE, self),
            0x1a => ld_a_de(self, memory),
            0x1b => dec_rr(DE, self),
            0x1c => inc_r(E, self),
            0x1d => dec_r(E, self),
            0x1e => ld_r_n(E, self, memory),
            0x1f => rra(self),

            0x20 => jr_cc_n(NZero, self, memory),
            0x21 => ld_rr_nn(HL, self, memory),
            0x22 => ld_hli_a(self, memory),
            0x23 => inc_rr(HL, self),
            0x24 => inc_r(H, self), 
            0x25 => dec_r(H, self),
            0x26 => ld_r_n(H, self, memory),
            0x27 => daa(self),
            0x28 => jr_cc_n(Zero, self, memory),
            0x29 => add_hl_rr(HL, self),
            0x2a => ld_a_hli(self, memory),
            0x2b => dec_rr(HL, self),
            0x2c => inc_r(L, self),
            0x2d => dec_r(L, self),
            0x2e => ld_r_n(L, self, memory),
            0x2f => cpl(self),

            0x30 => jr_cc_n(NCarry, self, memory),
            0x31 => ld_rr_nn(SP, self, memory),
            0x32 => ld_hld_a(self, memory),
            0x33 => inc_rr(SP, self),
            0x34 => inc_hl(self, memory),
            0x35 => dec_hl(self, memory),
            0x36 => ld_hl_n(self, memory),
            0x37 => scf(self),
            0x38 => jr_cc_n(Carry, self, memory),
            0x39 => add_hl_rr(SP, self),
            0x3a => ld_a_hld(self, memory),
            0x3b => dec_rr(SP, self),
            0x3c => inc_r(A, self),
            0x3d => dec_r(A, self),
            0x3e => ld_r_n(A, self, memory),
            0x3f => ccf(self),

            0x40 => ld_r_r(B, B, self),
            0x41 => ld_r_r(B, C, self),
            0x42 => ld_r_r(B, D, self),
            0x43 => ld_r_r(B, E, self),
            0x44 => ld_r_r(B, H, self),
            0x45 => ld_r_r(B, L, self),
            0x46 => ld_r_hl(B, self, memory),
            0x47 => ld_r_r(B, A, self),
            0x48 => ld_r_r(C, B, self),
            0x49 => ld_r_r(C, C, self),
            0x4a => ld_r_r(C, D, self),
            0x4b => ld_r_r(C, E, self),
            0x4c => ld_r_r(C, H, self),
            0x4d => ld_r_r(C, L, self),
            0x4e => ld_r_hl(C, self, memory),
            0x4f => ld_r_r(C, A, self),

            0x50 => ld_r_r(D, B, self),
            0x51 => ld_r_r(D, C, self),
            0x52 => ld_r_r(D, D, self),
            0x53 => ld_r_r(D, E, self),
            0x54 => ld_r_r(D, H, self),
            0x55 => ld_r_r(D, L, self),
            0x56 => ld_r_hl(D, self, memory),
            0x57 => ld_r_r(D, A, self),
            0x58 => ld_r_r(E, B, self),    
            0x59 => ld_r_r(E, C, self),
            0x5a => ld_r_r(E, D, self),
            0x5b => ld_r_r(E, E, self),
            0x5c => ld_r_r(E, H, self),
            0x5d => ld_r_r(E, L, self),
            0x5e => ld_r_hl(E, self, memory),
            0x5f => ld_r_r(E, A, self),

            0x60 => ld_r_r(H, B, self),
            0x61 => ld_r_r(H, C, self),
            0x62 => ld_r_r(H, D, self),
            0x63 => ld_r_r(H, E, self),
            0x64 => ld_r_r(H, H, self),
            0x65 => ld_r_r(H, L, self),
            0x66 => ld_r_hl(H, self, memory),
            0x67 => ld_r_r(H, A, self),
            0x68 => ld_r_r(L, B, self),
            0x69 => ld_r_r(L, C, self),
            0x6a => ld_r_r(L, D, self),
            0x6b => ld_r_r(L, E, self),
            0x6c => ld_r_r(L, H, self),
            0x6d => ld_r_r(L, L, self),
            0x6e => ld_r_hl(L, self, memory),
            0x6f => ld_r_r(L, A, self),

            0x70 => ld_hl_r(B, self, memory),
            0x71 => ld_hl_r(C, self, memory),
            0x72 => ld_hl_r(D, self, memory),
            0x73 => ld_hl_r(E, self, memory),
            0x74 => ld_hl_r(H, self, memory),
            0x75 => ld_hl_r(L, self, memory),
            0x76 => halt(self),
            0x77 => ld_hl_r(A, self, memory),
            0x78 => ld_r_r(A, B, self),
            0x79 => ld_r_r(A, C, self),
            0x7a => ld_r_r(A, D, self),
            0x7b => ld_r_r(A, E, self), 
            0x7c => ld_r_r(A, H, self),
            0x7d => ld_r_r(A, L, self),
            0x7e => ld_r_hl(A, self, memory),
            0x7f => ld_r_r(A, A, self),

            0x80 => add_a_r(B, self),
            0x81 => add_a_r(C, self),
            0x82 => add_a_r(D, self),
            0x83 => add_a_r(E, self),
            0x84 => add_a_r(H, self),
            0x85 => add_a_r(L, self),
            0x86 => add_a_hl(self, memory),
            0x87 => add_a_r(A, self),
            0x88 => adc_r(B, self),
            0x89 => adc_r(C, self),
            0x8a => adc_r(D, self),
            0x8b => adc_r(E, self),
            0x8c => adc_r(H, self),
            0x8d => adc_r(L, self),
            0x8e => adc_hl(self, memory),
            0x8f => adc_r(A, self),

            0x90 => sub_r(B, self),
            0x91 => sub_r(C, self),
            0x92 => sub_r(D, self),
            0x93 => sub_r(E, self),
            0x94 => sub_r(H, self),
            0x95 => sub_r(L, self),
            0x96 => sub_hl(self, memory),
            0x97 => sub_r(A, self),
            0x98 => sbc_r(B, self),
            0x99 => sbc_r(C, self),
            0x9a => sbc_r(D, self),
            0x9b => sbc_r(E, self),
            0x9c => sbc_r(H, self),
            0x9d => sbc_r(L, self),
            0x9e => sbc_hl(self, memory),
            0x9f => sbc_r(A, self),

            0xa0 => and_r(B, self),
            0xa1 => and_r(C, self),
            0xa2 => and_r(D, self),
            0xa3 => and_r(E, self),
            0xa4 => and_r(H, self),
            0xa5 => and_r(L, self),
            0xa6 => and_hl(self, memory),
            0xa7 => and_r(A, self),
            0xa8 => xor_r(B, self),
            0xa9 => xor_r(C, self),
            0xaa => xor_r(D, self),
            0xab => xor_r(E, self),
            0xac => xor_r(H, self),
            0xad => xor_r(L, self),
            0xae => xor_hl(self, memory),
            0xaf => xor_r(A, self),
           
            0xb0 => or_r(B, self),
            0xb1 => or_r(C, self),
            0xb2 => or_r(D, self),
            0xb3 => or_r(E, self),
            0xb4 => or_r(H, self),
            0xb5 => or_r(L, self),
            0xb6 => or_hl(self, memory),
            0xb7 => or_r(A, self),
            0xb8 => cp_r(B, self),
            0xb9 => cp_r(C, self),
            0xba => cp_r(D, self),
            0xbb => cp_r(E, self),
            0xbc => cp_r(H, self),
            0xbd => cp_r(L, self),
            0xbe => cp_hl(self, memory),
            0xbf => cp_r(A, self),

            0xc0 => ret_cc(NZero, self, memory),
            0xc1 => pop_rr(BC, self, memory),
            0xc2 => jp_cc_nn(NZero, self, memory),
            0xc3 => jp_nn(self, memory),
            0xc4 => call_cc_nn(NZero, self, memory),
            0xc5 => push_rr(BC, self, memory),
            0xc6 => add_a_n(self, memory),
            0xc7 => rst(0x00, self, memory),
            0xc8 => ret_cc(Zero, self, memory),
            0xc9 => ret(self, memory),
            0xca => jp_cc_nn(Zero, self, memory),
            0xcb => self.cb_prefix(memory),
            0xcc => call_cc_nn(Zero, self, memory),
            0xcd => call_nn(self, memory),
            0xce => adc_n(self, memory),
            0xcf => rst(0x08, self, memory),

            0xd0 => ret_cc(NCarry, self, memory),
            0xd1 => pop_rr(DE, self, memory),
            0xd2 => jp_cc_nn(NCarry, self, memory),
            0xd3 => panic!("Not valid opcode"),
            0xd4 => call_cc_nn(NCarry, self, memory), 
            0xd5 => push_rr(DE, self, memory),
            0xd6 => sub_n(self, memory),
            0xd7 => rst(0x10, self, memory),
            0xd8 => ret_cc(Carry, self, memory),
            0xd9 => reti(self, memory),
            0xda => jp_cc_nn(Carry, self, memory),
            0xdb => panic!("Not valid opcode"),
            0xdc => call_cc_nn(Carry, self, memory),
            0xdd => panic!("Not valid opcode"),
            0xde => sbc_n(self, memory),
            0xdf => rst(0x18, self, memory),

            0xe0 => ldh_n_a(self, memory),
            0xe1 => pop_rr(HL, self, memory),
            0xe2 => ld_c_a(self, memory),
            0xe3 => panic!("Not valid opcode"),
            0xe4 => panic!("Not valid opcode"),
            0xe5 => push_rr(HL, self, memory),
            0xe6 => and_n(self, memory),
            0xe7 => rst(0x20, self, memory),
            0xe8 => add_sp_n(self, memory),
            0xe9 => jp_hl(self),
            0xea => ld_nn_a(self, memory),
            0xeb => panic!("Not valid opcode"),
            0xec => panic!("Not valid opcode"),
            0xed => panic!("Not valid opcode"),
            0xee => xor_n(self, memory),
            0xef => rst(0x28, self, memory),

            0xf0 => ldh_a_n(self, memory),
            0xf1 => pop_rr(AF, self, memory),
            0xf2 => ld_a_c(self, memory),
            0xf3 => di(self),
            0xf4 => panic!("Not valid opcode"),
            0xf5 => push_rr(AF, self, memory),
            0xf6 => or_n(self, memory),
            0xf7 => rst(0x30, self, memory),
            0xf8 => ldhl_sp_n(self, memory),
            0xf9 => ld_sp_hl(self),
            0xfa => ld_a_nn(self, memory),
            0xfb => ei(self),
            0xfc => panic!("Not valid opcode"),
            0xfd => panic!("Not valid opcode"),
            0xfe => cp_n(self, memory),
            0xff => rst(0x38, self, memory),
        }
    }

    fn cb_prefix(&mut self, memory: &mut Memory) -> u8 {
        let opcode = self.fetch_opcode(memory);
        match opcode {
            0x00 => rlc_r(B, self),
            0x01 => rlc_r(C, self),
            0x02 => rlc_r(D, self),
            0x03 => rlc_r(E, self),
            0x04 => rlc_r(H, self),
            0x05 => rlc_r(L, self),
            0x06 => rlc_hl(self, memory),
            0x07 => rlc_r(A, self),
            0x08 => rrc_r(B, self),
            0x09 => rrc_r(C, self),
            0x0a => rrc_r(D, self),
            0x0b => rrc_r(E, self),
            0x0c => rrc_r(H, self),
            0x0d => rrc_r(L, self),
            0x0e => rrc_hl(self, memory),
            0x0f => rrc_r(A, self),

            0x10 => rl_r(B, self),
            0x11 => rl_r(C, self),
            0x12 => rl_r(D, self),
            0x13 => rl_r(E, self),
            0x14 => rl_r(H, self),
            0x15 => rl_r(L, self),
            0x16 => rl_hl(self, memory),
            0x17 => rl_r(A, self),
            0x18 => rr_r(B, self),
            0x19 => rr_r(C, self),
            0x1a => rr_r(D, self),
            0x1b => rr_r(E, self),
            0x1c => rr_r(H, self),
            0x1d => rr_r(L, self),
            0x1e => rr_hl(self, memory),
            0x1f => rr_r(A, self),

            0x40 => bit_r(B, 0, self),
            0x41 => bit_r(C, 0, self),
            0x42 => bit_r(D, 0, self),
            0x43 => bit_r(E, 0, self),
            0x44 => bit_r(H, 0, self),
            0x45 => bit_r(L, 0, self),
            0x46 => bit_hl(0, self, memory),
            0x47 => bit_r(A, 0, self),
            0x48 => bit_r(B, 1, self),
            0x49 => bit_r(C, 1, self),
            0x4a => bit_r(D, 1, self),
            0x4b => bit_r(E, 1, self),
            0x4c => bit_r(H, 1, self),
            0x4d => bit_r(L, 1, self),
            0x4e => bit_hl(1, self, memory),
            0x4f => bit_r(A, 1, self),

            0x50 => bit_r(B, 2, self),
            0x51 => bit_r(C, 2, self),
            0x52 => bit_r(D, 2, self),
            0x53 => bit_r(E, 2, self),
            0x54 => bit_r(H, 2, self),
            0x55 => bit_r(L, 2, self),  
            0x56 => bit_hl(2, self, memory),
            0x57 => bit_r(A, 2, self),
            0x58 => bit_r(B, 3, self),
            0x59 => bit_r(C, 3, self),
            0x5a => bit_r(D, 3, self),
            0x5b => bit_r(E, 3, self),
            0x5c => bit_r(H, 3, self),
            0x5d => bit_r(L, 3, self),
            0x5e => bit_hl(3, self, memory),
            0x5f => bit_r(A, 3, self),

            0x60 => bit_r(B, 4, self),
            0x61 => bit_r(C, 4, self),
            0x62 => bit_r(D, 4, self),
            0x63 => bit_r(E, 4, self),
            0x64 => bit_r(H, 4, self),
            0x65 => bit_r(L, 4, self),
            0x66 => bit_hl(4, self, memory),
            0x67 => bit_r(A, 4, self),
            0x68 => bit_r(B, 5, self),
            0x69 => bit_r(C, 5, self),
            0x6a => bit_r(D, 5, self),
            0x6b => bit_r(E, 5, self),  
            0x6c => bit_r(H, 5, self),
            0x6d => bit_r(L, 5, self),
            0x6e => bit_hl(5, self, memory),
            0x6f => bit_r(A, 5, self),

            0x70 => bit_r(B, 6, self),
            0x71 => bit_r(C, 6, self),
            0x72 => bit_r(D, 6, self),
            0x73 => bit_r(E, 6, self),
            0x74 => bit_r(H, 6, self),
            0x75 => bit_r(L, 6, self),
            0x76 => bit_hl(6, self, memory),
            0x77 => bit_r(A, 6, self),
            0x78 => bit_r(B, 7, self),
            0x79 => bit_r(C, 7, self),
            0x7a => bit_r(D, 7, self),
            0x7b => bit_r(E, 7, self),
            0x7c => bit_r(H, 7, self),
            0x7d => bit_r(L, 7, self),
            0x7e => bit_hl(7, self, memory),
            0x7f => bit_r(A, 7, self),

            _ => panic!("CB Opcode {:02x} not implemented", opcode),
        }
    }
}