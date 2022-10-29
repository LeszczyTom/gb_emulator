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
    _ime: bool,
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
            _ime: false,
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
    pub fn set_ime(&mut self, value: bool) { self._ime = value; }

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
    pub fn get_ime(&self) -> bool { self._ime }

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

    pub fn cycle(&mut self, memory: &mut Memory) -> u8{
        if self.halt {
            return 0;
        }
       
        let opcode =self.fetch_opcode(memory);
        self.exectute(opcode, memory)
    }

    fn exectute(&mut self, opcode: u8, memory: &mut Memory) -> u8 {
        match opcode {
            0x00 => 4,
            0x04 => inc_r(B, self),
            0x05 => dec_r(B, self),
            0x06 => ld_r_n(B, self, memory),
            0x0c => inc_r(C, self),
            0x0d => dec_r(C, self),
            0x0e => ld_r_n(C, self, memory),

            0x11 => ld_rr_nn(DE, self, memory),
            0x13 => inc_rr(DE, self),
            0x15 => dec_r(D, self),
            0x16 => ld_r_n(D, self, memory),
            0x17 => rla(self),
            0x18 => jr_n(self, memory),
            0x1a => ld_a_de(self, memory),
            0x1d => dec_r(E, self),
            0x1e => ld_r_n(E, self, memory),

            0x20 => jr_cc_n(NZero, self, memory),
            0x21 => ld_rr_nn(HL, self, memory),
            0x22 => ld_hli_a(self, memory),
            0x23 => inc_rr(HL, self),
            0x24 => inc_r(H, self), 
            0x28 => jr_cc_n(Zero, self, memory),
            0x2e => ld_r_n(L, self, memory),

            0x31 => ld_rr_nn(SP, self, memory),
            0x32 => ld_hld_a(self, memory),
            0x3d => dec_r(A, self),
            0x3e => ld_r_n(A, self, memory),

            0x4f => ld_r_r(C, A, self),

            0x57 => ld_r_r(D, A, self),

            0x67 => ld_r_r(H, A, self),

            0x77 => ld_rr_nn(HL, self, memory),
            0x78 => ld_r_r(A, B, self),
            0x7b => ld_r_r(A, E, self), 
            0x7c => ld_r_r(A, H, self),
            0x7d => ld_r_r(A, L, self),

            0x86 => add_a_hl(self, memory),

            0x90 => sub_r(B, self),
            0x93 => sub_r(E, self),
            0x96 => sub_hl(self, memory),

            0xaf => xor_r(A, self),
           
            0xbe => cp_hl(self, memory),

            0xc1 => pop_rr(BC, self, memory),
            0xc5 => push_rr(BC, self, memory),
            0xc9 => ret(self, memory),
            0xcb => self.cb_prefix(memory),
            0xcd => call_nn(self, memory),

            0xd6 => sub_n(self, memory),

            0xe0 => ldh_n_a(self, memory),
            0xe2 => ld_c_a(self, memory),
            0xea => ld_nn_a(self, memory),
            
            0xf0 => ldh_a_n(self, memory),
            0xfe => cp_n(self, memory),

            _ => panic!("Opcode {:02x} not implemented", opcode),
        }
    }

    fn cb_prefix(&mut self, memory: &mut Memory) -> u8 {
        let opcode = self.fetch_opcode(memory);
        match opcode {
            0x7c => bit_s(H, 7, self),
            0x11 => rl_r(C, self),
            _ => panic!("CB Opcode {:02x} not implemented", opcode),
        }
    }
}