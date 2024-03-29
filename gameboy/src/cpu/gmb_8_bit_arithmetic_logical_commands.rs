use crate::cpu::cpu::{Cpu, Flag::*, Register, RegisterPair::HL};
use crate::memory::mmu::Mmu;

/// Adds the contents of operand s and CY to the contents of register A and stores the results in register A. r, n, and (HL) are used for operand s.
/// ``` rust
/// //Examples: When A = 0xE1, E = 0x0f, (HL) = 0x1e, and CY = 1
/// //ADC A, E ; A <- 0xf1, Z <- 0, H <- 1 , CY <- 0
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Mmu::new();
/// # memory.set_bios_enabled(false);
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
fn adc_s(value: u8, cpu: &mut Cpu) {
    let carry = (cpu.f & 0x10 == 0x10) as u8;
    let result = cpu.a.wrapping_add(value).wrapping_add(carry);

    cpu.set_flag(Zero, result == 0);
    cpu.set_flag(Subtract, false);
    cpu.set_flag(HalfCarry, (cpu.a & 0xf) + (value & 0xf) + carry > 0xf);
    cpu.set_flag(Carry, cpu.a as u16 + value as u16 + carry as u16 > 0xff);

    cpu.a = result;
}

/// Adds the contents of operand r and CY to the contents of register A and stores the results in register A.
pub fn adc_r(r: Register, cpu: &mut Cpu) -> u8 {
    adc_s(cpu.get_r(r), cpu);

    4
}

/// Adds the contents of operand (HL) and CY to the contents of register A and stores the results in register A.
pub fn adc_hl(cpu: &mut Cpu, memory: &mut Mmu) -> u8 {
    let value = memory.read_byte(cpu.get_rr(HL));
    adc_s(value, cpu);

    8
}

/// Adds the contents of operand n and CY to the contents of register A and stores the results in register A.
pub fn adc_n(cpu: &mut Cpu, memory: &mut Mmu) -> u8 {
    adc_s(cpu.read_n(memory), cpu);

    8
}

/// Subtracts the contents of operand s from the contents of register A and stores the results in register A. r, n, and (HL) are used for operand s.
/// ```rust
/// //Examples: When A = 0x3E, E = 0x3E, and (HL) = 0x40,
/// //SUB E ; A <- 0x00, Z <-1, N <- 1, H <- 0, CY <- 0
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Mmu::new();
/// # memory.set_bios_enabled(false);
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
    cpu.set_flag(
        HalfCarry,
        (cpu.a & 0xf).wrapping_sub(value & 0xf) & (0x10) != 0,
    );
    cpu.set_flag(Carry, (cpu.a as u16) < (value as u16));

    cpu.a = result;
}

/// Subtracts the contents of operand r from the contents of register A and stores the results in register A.
pub fn sub_r(r: Register, cpu: &mut Cpu) -> u8 {
    sub_s(cpu.get_r(r), cpu);

    4
}

/// Subtracts the contents of operand (HL) from the contents of register A and stores the results in register A.
pub fn sub_hl(cpu: &mut Cpu, memory: &mut Mmu) -> u8 {
    let value = memory.read_byte(cpu.get_rr(HL));
    sub_s(value, cpu);

    8
}

/// Subtracts the contents of operand n from the contents of register A and stores the results in register A.
pub fn sub_n(cpu: &mut Cpu, memory: &mut Mmu) -> u8 {
    sub_s(cpu.read_n(memory), cpu);

    8
}

/// Compares the contents of operand s and register A and sets the flag if they are equal, r, n, and (HL) are used for operand s.
/// ```rust
/// //Examples: When A = 0x3C, B = 0x2F, and (HL) = 0x40,
/// //CP B ; Z <- 0, N <- 1, H <- 1, CY <- 0
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Mmu::new();
/// # memory.set_bios_enabled(false);
/// # memory.write_byte(0x100, 0xb8);
/// # cpu.set_pc(0x100);
/// # cpu.set_a(0x3c);
/// # cpu.set_b(0x2f);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_f(), 0x60);
/// //CP 0x3C ; Z <- 1, N <- 1, H <- 0, CY <- 0
/// # cpu.set_pc(0x100);
/// # memory.write_byte(0x100, 0xfe);
/// # memory.write_byte(0x101, 0x3c);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_f(), 0xc0);
/// //CP (HL) ; Z <- 0, N <- 1, H <- 0 , CY <- 1
/// # cpu.set_pc(0x100);
/// # memory.write_byte(0x100, 0xbe);
/// # cpu.set_h(0x10);
/// # cpu.set_l(0x00);
/// # memory.write_byte(0x1000, 0x40);
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
pub fn cp_r(r: Register, cpu: &mut Cpu) -> u8 {
    cp_s(cpu.get_r(r), cpu);

    4
}

/// Compares the contents of operand (HL) and register A and sets the flag if they are equal.
pub fn cp_hl(cpu: &mut Cpu, memory: &mut Mmu) -> u8 {
    let value = memory.read_byte(cpu.get_rr(HL));
    cp_s(value, cpu);

    8
}

/// Compares the contents of operand n and register A and sets the flag if they are equal.
pub fn cp_n(cpu: &mut Cpu, memory: &mut Mmu) -> u8 {
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
/// # let mut memory = gameboy::gameboy::memory::Mmu::new();
/// # memory.set_bios_enabled(false);
/// # memory.write_byte(0x100, 0xaf);
/// # cpu.set_pc(0x100);
/// # cpu.set_a(0xff);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_a(), 0x00);
/// assert_eq!(cpu.get_f(), 0x80);
///
/// //XOR 0x0F ; A <- 0xF0, Z <- 0
/// # cpu.set_pc(0x100);
/// # memory.write_byte(0x100, 0xee);
/// # cpu.set_a(0xff);
/// # memory.write_byte(0x101, 0x0f);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_a(), 0xf0);
/// assert_eq!(cpu.get_f(), 0x00);
///
/// //XOR (HL) ; A <- 75h, Z <- 0
/// # cpu.set_pc(0x100);
/// # memory.write_byte(0x100, 0xae);
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
pub fn xor_hl(cpu: &mut Cpu, memory: &mut Mmu) -> u8 {
    let value = memory.read_byte(cpu.get_rr(HL));
    xor_s(value, cpu);

    8
}

/// Takes the logical exclusive-OR for each bit of the contents of operand n and register A.
/// And stores the results in register A.
pub fn xor_n(cpu: &mut Cpu, memory: &mut Mmu) -> u8 {
    xor_s(cpu.read_n(memory), cpu);

    8
}

/// Takes the one’s complement of the contents of register A.
/// ``` rust
/// //Example: When A = 0x35,
/// //CPL ; A <- 0xCA
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Mmu::new();
/// # memory.set_bios_enabled(false);
/// # memory.write_byte(0x100, 0x2f);
/// # cpu.set_pc(0x100);
/// # cpu.set_a(0x35);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_a(), 0xca);
/// assert_eq!(cpu.get_f(), 0x60);
/// ```
pub fn cpl(cpu: &mut Cpu) -> u8 {
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
/// # let mut memory = gameboy::gameboy::memory::Mmu::new();
/// # memory.set_bios_enabled(false);
/// # memory.write_byte(0x100, 0x3c);
/// # cpu.set_pc(0x100);
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

/// Adds the contents of operand s to those of register A and stores the results in register A. r, n, and (HL) are used for operand s.
fn add_s(value: u8, cpu: &mut Cpu) {
    cpu.a = cpu.a.wrapping_add(value);

    cpu.set_flag(Zero, cpu.a == 0);
    cpu.set_flag(Subtract, false);
    cpu.set_flag(HalfCarry, (cpu.a & 0xf) < (value & 0xf));
    cpu.set_flag(Carry, (cpu.a as u16) < (value as u16));
}

/// Adds the contents of memory specified by the contents of register pair HL to the contents of register A and stores the results in register A.
/// ``` rust
/// //Example: When A = 0x3C and (HL) = 0x12,
/// //ADD A, (HL) ; A <- 0x4E, Z <- 0, N <- 0, H <- 0, CY <- 0
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Mmu::new();
/// # memory.set_bios_enabled(false);
/// # memory.write_byte(0x100, 0x86);
/// # cpu.set_pc(0x100);
/// # cpu.set_a(0x3c);
/// # cpu.set_h(0x10);
/// # cpu.set_l(0x00);
/// # memory.write_byte(0x1000, 0x12);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_a(), 0x4e);
/// assert_eq!(cpu.get_f(), 0x00);
/// ```
pub fn add_a_hl(cpu: &mut Cpu, memory: &mut Mmu) -> u8 {
    let value = memory.read_byte(cpu.get_rr(HL));
    add_s(value, cpu);

    8
}

/// Adds the contents of register r to those of register A and stores the results in register A.
/// ``` rust
/// //Example: When A = 0x3A and B = 0xC6,
/// //ADD A, B ; A <- 0, Z <- 1 , N <- 0, H <- 1 , CY <- 1
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Mmu::new();
/// # memory.set_bios_enabled(false);
/// # memory.write_byte(0x00, 0x80);
/// # cpu.set_a(0x3a);
/// # cpu.set_b(0xc6);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_a(), 0);
/// assert_eq!(cpu.get_f(), 0xb0);
/// ```
pub fn add_a_r(r: Register, cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r(r);
    add_s(value, cpu);

    4
}

/// Adds 8-bit immediate operand n to the contents of register A and stores the results in register A.
/// ``` rust
/// //Example: When A = 0x3C,
/// //  ADD A, 0xFF ; A <- 0x3B, Z <- 0, N <- 0, H <- 1, CY <- 1
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Mmu::new();
/// # memory.set_bios_enabled(false);
/// # memory.write_byte(0x00, 0xc6);
/// # memory.write_byte(0x01, 0xff);
/// # cpu.set_a(0x3c);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_a(), 0x3b);
/// assert_eq!(cpu.get_f(), 0x30);
/// ```
pub fn add_a_n(cpu: &mut Cpu, memory: &mut Mmu) -> u8 {
    let value = cpu.read_n(memory);
    add_s(value, cpu);

    8
}

/// Loads in register A the contents of memory specified by the contents of register pair HL and simultaneously increments the contents of HL.
/// ``` rust
/// //Example: When HL = 0x1FF and (0x1FF) = 0x56,
/// //LD A, (HLI) ; A <- 0x56, HL <- 0x200
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Mmu::new();
/// # memory.set_bios_enabled(false);
/// # memory.write_byte(0x00, 0x2a);
/// # cpu.set_h(0x01);
/// # cpu.set_l(0xff);
/// # memory.write_byte(0x01ff, 0x56);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_a(), 0x56);
/// assert_eq!(cpu.get_h(), 0x02);
/// assert_eq!(cpu.get_l(), 0x00);
/// ```
pub fn ld_a_hli(cpu: &mut Cpu, memory: &mut Mmu) -> u8 {
    let value = memory.read_byte(cpu.get_rr(HL));
    cpu.a = value;
    cpu.set_rr(HL, cpu.get_rr(HL).wrapping_add(1));

    8
}

fn sbc_s(value: u8, cpu: &mut Cpu) {
    let carry = cpu.get_flag(Carry) as u8;
    let result = cpu.a.wrapping_sub(value).wrapping_sub(carry);

    cpu.set_flag(Zero, result == 0);
    cpu.set_flag(Subtract, true);
    cpu.set_flag(HalfCarry, (cpu.a & 0xf) < (value & 0xf) + carry);
    cpu.set_flag(Carry, (cpu.a as u16) < (value as u16) + (carry as u16));

    cpu.a = result;
}

/// Subtracts the contents of register r and CY from the contents of register A and stores the results in register A.
/// ``` rust
/// //Examples: When A = 0x3B, H = 0x2A, and CY = 1
/// //SBC A, H ; A <- 1 0h, Z <- 0, N <- 1, H <- 0,CY <— 0
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Mmu::new();
/// # memory.set_bios_enabled(false);
/// # memory.write_byte(0x00, 0x9c);
/// # cpu.set_f(0x10);
/// # cpu.set_a(0x3b);
/// # cpu.set_h(0x2a);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_a(), 0x10);
/// assert_eq!(cpu.get_f(), 0x40);
/// ```
pub fn sbc_r(r: Register, cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r(r);
    sbc_s(value, cpu);

    4
}

/// Subtracts the contents of (HL) and CY from the contents of register A and stores the results in register A.
/// ```rust
/// //Examples: When A = 0x3B, (HL) = 0x4F, and CY = 1
/// //SBC A, (HL) ; A <- 0xEB, Z <- 0, N <- 1, H <- 1 , CY <- 1
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Mmu::new();
/// # memory.set_bios_enabled(false);
/// # memory.write_byte(0x00, 0x9e);
/// # cpu.set_f(0x10);
/// # cpu.set_a(0x3b);
/// # cpu.set_h(0x10);
/// # cpu.set_l(0x00);
/// # memory.write_byte(0x1000, 0x4f);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_a(), 0xeb);
/// assert_eq!(cpu.get_f(), 0x70);
///
pub fn sbc_hl(cpu: &mut Cpu, memory: &mut Mmu) -> u8 {
    let value = memory.read_byte(cpu.get_rr(HL));
    sbc_s(value, cpu);

    8
}

/// Subtracts the contents of n and CY from the contents of register A and stores the results in register A.
/// ``` rust
/// //Examples: When A = 0x3B, and CY = 1
/// //SBC A, 0x3A ; A <- 0x00, Z <- 1 , N <- 1, H <- 0, CY <- 0
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Mmu::new();
/// # memory.set_bios_enabled(false);
/// # memory.write_byte(0x00, 0xde);
/// # memory.write_byte(0x01, 0x3a);
/// # cpu.set_f(0x10);
/// # cpu.set_a(0x3b);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_a(), 0x00);
/// assert_eq!(cpu.get_f(), 0xC0);
/// ```
pub fn sbc_n(cpu: &mut Cpu, memory: &mut Mmu) -> u8 {
    let value = cpu.read_n(memory);
    sbc_s(value, cpu);

    8
}

// Takes the logical-AND for each bit of the contents of operand s and register A, and stores the results in register A.
// r, n, and (HL) are used for operand s.
fn and_s(value: u8, cpu: &mut Cpu) {
    cpu.a &= value;

    cpu.set_flag(Zero, cpu.a == 0);
    cpu.set_flag(Subtract, false);
    cpu.set_flag(HalfCarry, true);
    cpu.set_flag(Carry, false);
}

/// Takes the logical-AND for each bit of the contents of regiter r and register A, and stores the results in register A.
/// ```rust
/// //Examples: When A = 0x5A, L = 0x3F
/// //AND L ; A <- 0x1A, Z <- 0, N <- 0, H <- 1, CY <— 0
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Mmu::new();
/// # memory.set_bios_enabled(false);
/// # memory.write_byte(0x00, 0xa5);
/// # cpu.set_a(0x5a);
/// # cpu.set_l(0x3f);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_a(), 0x1a);
/// assert_eq!(cpu.get_f(), 0x20);
/// ```
pub fn and_r(r: Register, cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r(r);
    and_s(value, cpu);

    4
}

/// Takes the logical-AND for each bit of the contents of n and register A, and stores the results in register A.
/// ```rust
/// //Examples: When A = 0x5A
/// //AND 0x38 ; A <- 0x18, Z <- 0, N <- 0, H <- 1, CY <- 0
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Mmu::new();
/// # memory.set_bios_enabled(false);
/// # memory.write_byte(0x00, 0xe6);
/// # memory.write_byte(0x01, 0x38);
/// # cpu.set_a(0x5a);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_a(), 0x18);
/// assert_eq!(cpu.get_f(), 0x20);
/// ```
pub fn and_n(cpu: &mut Cpu, memory: &mut Mmu) -> u8 {
    let value = cpu.read_n(memory);
    and_s(value, cpu);

    8
}

/// Takes the logical-AND for each bit of the contents of n and register A, and stores the results in register A.
/// ```rust
/// //Examples: When A = 0x5A, (HL) = 0x00
/// //AND (HL) ; A <- 0x00, Z <- 1, N <- 0, H <- 1, CY <- 0
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Mmu::new();
/// # memory.set_bios_enabled(false);
/// # memory.write_byte(0x00, 0xa6);
/// # cpu.set_a(0x5a);
/// # cpu.set_h(0x10);
/// # cpu.set_l(0x00);
/// # memory.write_byte(0x1000, 0x00);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_a(), 0x00);
/// assert_eq!(cpu.get_f(), 0xa0);
/// ```
pub fn and_hl(cpu: &mut Cpu, memory: &mut Mmu) -> u8 {
    let value = memory.read_byte(cpu.get_rr(HL));
    and_s(value, cpu);

    8
}

/// Takes the logical-OR for each bit of the contents of operand s and register A and stores the results in register A.
///  r, n, and (HL) are used for operand s.
fn or_s(value: u8, cpu: &mut Cpu) {
    cpu.a |= value;

    cpu.set_flag(Zero, cpu.a == 0);
    cpu.set_flag(Subtract, false);
    cpu.set_flag(HalfCarry, false);
    cpu.set_flag(Carry, false);
}

/// Takes the logical-OR for each bit of the contents of register r and register A and stores the results in register A.
/// ```rust
/// //Examples: When A = 0x5A
/// //OR A ; A <— 0x5A, Z <- 0
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Mmu::new();
/// # memory.set_bios_enabled(false);
/// # memory.write_byte(0x00, 0xb7);
/// # cpu.set_a(0x5a);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_a(), 0x5a);
/// assert_eq!(cpu.get_f(), 0x00);
/// ```
pub fn or_r(r: Register, cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r(r);
    or_s(value, cpu);

    4
}

/// Takes the logical-OR for each bit of the contents of n and register A and stores the results in register A.
/// ```rust
/// //Examples: When A = 0x5A
/// //OR 0x03 ; A <- 0x5B, Z <- 0
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Mmu::new();
/// # memory.set_bios_enabled(false);
/// # memory.write_byte(0x00, 0xf6);
/// # memory.write_byte(0x01, 0x03);
/// # cpu.set_a(0x5a);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_a(), 0x5b);
/// assert_eq!(cpu.get_f(), 0x00);
/// ```
pub fn or_n(cpu: &mut Cpu, memory: &mut Mmu) -> u8 {
    let value = cpu.read_n(memory);
    or_s(value, cpu);

    8
}

/// Takes the logical-OR for each bit of the contents of n and register A and stores the results in register A.
/// ```rust
/// //Examples: When A = 0x5A, (HL) = 0x0f
/// //OR (HL) ; A <- 0x5F, Z <- 0
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Mmu::new();
/// # memory.set_bios_enabled(false);
/// # memory.write_byte(0x00, 0xb6);
/// # cpu.set_a(0x5a);
/// # cpu.set_h(0x10);
/// # cpu.set_l(0x00);
/// # memory.write_byte(0x1000, 0x0f);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_a(), 0x5f);
/// assert_eq!(cpu.get_f(), 0x00);
/// ```
pub fn or_hl(cpu: &mut Cpu, memory: &mut Mmu) -> u8 {
    let value = memory.read_byte(cpu.get_rr(HL));
    or_s(value, cpu);

    8
}

/// Subtract 1 from the contents of register r by 1.
/// ```rust
/// //Example: When B = 0x01,
/// //DEC B ; B <- 0, Z <- 1, N <— 1 H <- 0,
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Mmu::new();
/// # memory.set_bios_enabled(false);
/// # memory.write_byte(0x00, 0x05);
/// # cpu.set_b(0x01);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_b(), 0x00);
/// assert_eq!(cpu.get_f(), 0xc0);
/// ```
pub fn dec_r(r: Register, cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r(r.clone());
    let result = value.wrapping_sub(1);

    cpu.set_flag(Zero, result == 0);
    cpu.set_flag(Subtract, true);
    cpu.set_flag(HalfCarry, (value & 0x0f) == 0);

    cpu.set_r(r, result);
    4
}
