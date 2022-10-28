use crate::gameboy::{ cpu::Cpu, memory::Memory };
use crate::gameboy::cpu::Register;
use crate::gameboy::cpu::RegisterPair::HL;
use crate::gameboy::cpu::Flag::*;

/// Adds the contents of operand s and CY to the contents of register A and stores the results in register A. r, n, and (HL) are used for operand s.
/// ``` rust
/// //Examples: When A = 0xE1, E = 0x0f, (HL) = 0x1e, and CY = 1 
/// //ADC A, E ; A <- 0xf1, Z <- 0, H <- 1 , CY <- 0 
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Memory::new();
/// # memory.write_byte(0x00, 0x8b);
/// # cpu.set_a(0xe1);
/// # cpu.set_e(0x0f);
/// # cpu.set_f(0x10);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_a(), 0xf1);
/// assert_eq!(cpu.get_f(), 0x20);
/// //ADC A, 0x3B ; A <- 0x1D, Z <- 0, H <- 0, CY <- 1 
/// # cpu.set_pc(0x00);
/// # cpu.set_a(0xe1);
/// # memory.write_byte(0x00, 0xce);
/// # memory.write_byte(0x01, 0x3b);
/// # cpu.set_f(0x10);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_a(), 0x1d);
/// assert_eq!(cpu.get_f(), 0x10);
/// //ADC A, (HL) ; A <- 0x00, Z <- 1, H <- 1, CY <- 1
/// # cpu.set_pc(0x00);
/// # cpu.set_a(0xe1);
/// # cpu.set_h(0x01);
/// # cpu.set_l(0x00);
/// # memory.write_byte(0x100, 0x1e);
/// # memory.write_byte(0x00, 0x8e);
/// # cpu.set_f(0x10);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_a(), 0x00);
/// assert_eq!(cpu.get_f(), 0xb0);
/// ```
fn _adc_s(value: u8, cpu: &mut Cpu) {
    let carry = (cpu.f & 0x10 == 0x10) as u8 ;
    let result = cpu.a.wrapping_add(value).wrapping_add(carry);

    cpu.set_flag(Zero, result == 0);
    cpu.set_flag(Subtract, false);
    cpu.set_flag(HalfCarry, (cpu.a & 0xf) + (value & 0xf) + carry > 0xf);
    cpu.set_flag(Carry, cpu.a as u16 + value as u16 + carry as u16 > 0xff);

    cpu.a = result;
}

/// Adds the contents of operand r and CY to the contents of register A and stores the results in register A.
pub fn _adc_r(r: Register, cpu: &mut Cpu) -> u8 {
    _adc_s(cpu.get_r(r), cpu);

    4
}

/// Adds the contents of operand (HL) and CY to the contents of register A and stores the results in register A.
pub fn _adc_hl(cpu: &mut Cpu, memory: &mut Memory) -> u8 {
    let value = memory.read_byte(cpu.get_rr(HL));
    _adc_s(value, cpu);

    8
}

/// Adds the contents of operand n and CY to the contents of register A and stores the results in register A.
pub fn _adc_n(cpu: &mut Cpu, memory: &mut Memory) -> u8 {
    _adc_s(cpu.read_n(memory), cpu);

    8
}

/// Subtracts the contents of operand s from the contents of register A and stores the results in register A. r, n, and (HL) are used for operand s.
/// ```rust
/// //Examples: When A = 0x3E, E = 0x3E, and (HL) = 0x40,
/// //SUB E ; A <- 0x00, Z <-1, N <- 1, H <- 0, CY <- 0
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Memory::new();
/// # memory.write_byte(0x00, 0x93);
/// # cpu.set_a(0x3e);
/// # cpu.set_e(0x3e);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_a(), 0x00);
/// assert_eq!(cpu.get_f(), 0xc0 );
/// //SUB 0x0f ; A <- 0x2F, Z <- 0, N <- 1, H <- 1, CY <- 0
/// # memory.write_byte(0x01, 0xd6);
/// # memory.write_byte(0x02, 0x0f);
/// # cpu.set_a(0x3e);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_a(), 0x2f);
/// assert_eq!(cpu.get_f(), 0x60);
/// //SUB (HL) ; A <- 0xFE, Z <- 0, N <- 1, H <- 0, CY <— 1
/// # memory.write_byte(0x03, 0x96);
/// # cpu.set_a(0x3e);
/// # cpu.set_h(0x00);
/// # cpu.set_l(0x50);
/// # memory.write_byte(0x50, 0x40);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_a(), 0xfe);
/// assert_eq!(cpu.get_f(), 0x50);
/// ```
pub fn sub_s(value: u8, cpu: &mut Cpu) {
    let result = cpu.a.wrapping_sub(value);
    
    cpu.set_flag(Zero, result == 0);
    cpu.set_flag(Subtract, true);
    cpu.set_flag(HalfCarry, (cpu.a & 0xf).wrapping_sub(value & 0xf) & (0x10) != 0);
    cpu.set_flag(Carry, (cpu.a as u16) < (value as u16));

    cpu.a = result;
}

/// Subtracts the contents of operand r from the contents of register A and stores the results in register A.
pub fn sub_r(r: Register, cpu: &mut Cpu) -> u8 {
    sub_s(cpu.get_r(r), cpu);

    4
}

/// Subtracts the contents of operand (HL) from the contents of register A and stores the results in register A.
pub fn sub_hl(cpu: &mut Cpu, memory: &mut Memory) -> u8 {
    let value = memory.read_byte(cpu.get_rr(HL));
    sub_s(value, cpu);

    8
}

/// Subtracts the contents of operand n from the contents of register A and stores the results in register A.
pub fn sub_n(cpu: &mut Cpu, memory: &mut Memory) -> u8 {
    sub_s(cpu.read_n(memory), cpu);

    8
}

/// Compares the contents of operand s and register A and sets the flag if they are equal, r, n, and (HL) are used for operand s.
/// ```rust
/// //Examples: When A = 0x3C, B = 0x2F, and (HL) = 0x40,
/// //CP B ; Z <- 0, N <- 1, H <- 1, CY <- 0
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Memory::new();
/// # memory.write_byte(0x00, 0xb8);
/// # cpu.set_a(0x3c);
/// # cpu.set_b(0x2f);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_f(), 0x60);
/// //CP 0x3C ; Z <- 1, N <- 1, H <- 0, CY <- 0
/// # cpu.set_pc(0x00);
/// # memory.write_byte(0x00, 0xfe);
/// # memory.write_byte(0x01, 0x3c);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_f(), 0xc0);
/// //CP (HL) ; Z <- 0, N <- 1, H <- 0 , CY <- 1
/// # cpu.set_pc(0x00);
/// # memory.write_byte(0x00, 0xbe);
/// # cpu.set_h(0x01);
/// # cpu.set_l(0x00);
/// # memory.write_byte(0x100, 0x40);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_f(), 0x50);
/// ```
fn cp_s(value: u8, cpu: &mut Cpu) {
    let result = cpu.a.wrapping_sub(value);
    cpu.set_flag(Zero, result == 0);
    cpu.set_flag(Subtract, true);
    cpu.set_flag(HalfCarry, value & 0xf > cpu.a & 0xf);
    cpu.set_flag(Carry, value > cpu.a);
}

/// Compares the contents of operand r and register A and sets the flag if they are equal.
pub fn _cp_r(r: Register, cpu: &mut Cpu) -> u8 {
    cp_s(cpu.get_r(r), cpu);

    4
}

/// Compares the contents of operand (HL) and register A and sets the flag if they are equal.
pub fn cp_hl(cpu: &mut Cpu, memory: &mut Memory) -> u8 {
    let value = memory.read_byte(cpu.get_rr(HL));
    cp_s(value, cpu);

    8
}

/// Compares the contents of operand n and register A and sets the flag if they are equal.
pub fn cp_n(cpu: &mut Cpu, memory: &mut Memory) -> u8 {
    cp_s(cpu.read_n(memory), cpu);

    8
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
/// # cpu.set_a(0xff);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_a(), 0x00);
/// assert_eq!(cpu.get_f(), 0x80);
/// 
/// //XOR 0x0F ; A <- 0xF0, Z <- 0
/// # cpu.set_pc(0x00);
/// # memory.write_byte(0x00, 0xee);
/// # cpu.set_a(0xff);
/// # memory.write_byte(0x01, 0x0f);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_a(), 0xf0);
/// assert_eq!(cpu.get_f(), 0x00);
/// 
/// //XOR (HL) ; A <- 75h, Z <- 0
/// # cpu.set_pc(0x00);
/// # memory.write_byte(0x00, 0xae);
/// # cpu.set_a(0xff);
/// # cpu.set_h(0x10);
/// # cpu.set_l(0x00);
/// # memory.write_byte(0x1000, 0x8a);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_a(), 0x75);
/// assert_eq!(cpu.get_f(), 0x00);
/// ```
fn xor_s(s: u8, cpu: &mut Cpu) {
    cpu.a ^= s;
    cpu.set_flag(Zero, cpu.a == 0);
    cpu.set_flag(Subtract, false);
    cpu.set_flag(HalfCarry, false);
    cpu.set_flag(Carry, false);
}

/// Takes the logical exclusive-OR for each bit of the contents of operand r and register A.
/// And stores the results in register A.
pub fn xor_r(r: Register, cpu: &mut Cpu) -> u8 {
    xor_s(cpu.get_r(r), cpu);

    4
}

/// Takes the logical exclusive-OR for each bit of the contents of operand (HL) and register A.
/// And stores the results in register A.
pub fn _xor_hl(cpu: &mut Cpu, memory: &mut Memory) -> u8 {
    let value = memory.read_byte(cpu.get_rr(HL));
    xor_s(value, cpu);

    8
}

/// Takes the logical exclusive-OR for each bit of the contents of operand n and register A.
/// And stores the results in register A.
pub fn _xor_n(cpu: &mut Cpu, memory: &mut Memory) -> u8 {
    xor_s(cpu.read_n(memory), cpu);

    8
}

/// Takes the one’s complement of the contents of register A.
/// ``` rust
/// //Example: When A = 0x35,
/// //CPL ; A <- 0xCA
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Memory::new();
/// # memory.write_byte(0x00, 0x2f);
/// # cpu.set_a(0x35);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_a(), 0xca);
/// assert_eq!(cpu.get_f(), 0x60);
/// ```
fn _cpl(cpu: &mut Cpu) -> u8 {
    cpu.a = !cpu.a;
    cpu.set_flag(Subtract, true);
    cpu.set_flag(HalfCarry, true);
    
    4
}

/// Increments the contents of register r by 1 .
/// ``` rust
/// //Example: When A = 0xFF,
/// //INC A ; A <- 0, Z <- 1, N <- 0, H <- 1
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Memory::new();
/// # memory.write_byte(0x00, 0x3c);
/// # cpu.set_a(0xff);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_a(), 0);
/// assert_eq!(cpu.get_f(), 0xa0);
/// ```
pub fn inc_r(r: Register, cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r(r.clone());
    let result = value.wrapping_add(1);
    cpu.set_r(r, result);

    cpu.set_flag(Zero, result == 0);
    cpu.set_flag(Subtract, false);
    cpu.set_flag(HalfCarry, value & 0xf == 0xf);

    4
}