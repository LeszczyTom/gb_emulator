use crate::cpu::cpu::{
    Cpu,
    Flag::*,
    RegisterPair::{self, *},
};

use crate::memory::mmu::Mmu;

/// Increments the contents of register pair rr by 1.
/// ```rust
/// //Example: When DE = 0x235f
/// //INC DE ; DE <- 0x2360
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Mmu::new();
/// # memory.set_bios_enabled(false);
/// # memory.write_byte(0x00, 0x13);
/// # cpu.set_d(0x23);
/// # cpu.set_e(0x5f);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_de(), 0x2360);
/// ```
pub fn inc_rr(rr: RegisterPair, cpu: &mut Cpu) -> u8 {
    let value = cpu.get_rr(rr.clone());
    cpu.set_rr(rr, value.wrapping_add(1));

    8
}

/// Decrements the contents of register pair ss by 1.
/// ```rust
/// //Example: When DE = 0x235F,
/// //DEC DE ; DE <- 0x235E
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Mmu::new();
/// # memory.set_bios_enabled(false);
/// # memory.write_byte(0x00, 0x1B);
/// # cpu.set_d(0x23);
/// # cpu.set_e(0x5F);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_de(), 0x235E);
/// ```
pub fn dec_rr(rr: RegisterPair, cpu: &mut Cpu) -> u8 {
    let value = cpu.get_rr(rr.clone());
    cpu.set_rr(rr, value.wrapping_sub(1));

    8
}

/// Adds the contents of register pair ss to the contents of register pair HL and stores the results in HL.
/// ```rust
/// //Example: When HL = 0x8A23, BC = 0x0605,
/// //ADD HL, BC ; HL <- 0x9028, N <- 0, H <- 1 , CY <- 0
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Mmu::new();
/// # memory.set_bios_enabled(false);
/// # memory.write_byte(0x00, 0x09);
/// # cpu.set_h(0x8a);
/// # cpu.set_l(0x23);
/// # cpu.set_b(0x06);
/// # cpu.set_c(0x05);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_hl(), 0x9028);
/// assert_eq!(cpu.get_f(), 0x20);
///
/// //ADD HL, HL ; HL <- 0x1446, N <- 0, H <- 1, CY <- 1
/// # cpu.set_pc(0x00);
/// # memory.write_byte(0x00, 0x29);
/// # cpu.set_h(0x8a);
/// # cpu.set_l(0x23);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_hl(), 0x1446);
/// assert_eq!(cpu.get_f(), 0x30);
/// ```
pub fn add_hl_rr(rr: RegisterPair, cpu: &mut Cpu) -> u8 {
    let value = cpu.get_rr(rr.clone());
    let result = cpu.get_rr(HL).overflowing_add(value);

    cpu.set_flag(HalfCarry, value & 0x0fff > result.0 & 0x0fff);
    cpu.set_flag(Carry, result.1);
    cpu.set_flag(Subtract, false);

    cpu.set_rr(HL, result.0);
    8
}

/// Increments by 1 the contents of memory specified by register pair HL.
/// ```rust
/// //Example: When (HL) = 0x50,
/// //INC (HL) ; (HL) <- 0x51 , Z <- 0, N <- 0, H <- 0
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Mmu::new();
/// # memory.set_bios_enabled(false);
/// # memory.write_byte(0x00, 0x34);
/// # cpu.set_h(0x10);
/// # cpu.set_l(0x00);
/// # memory.write_byte(0x1000, 0x50);
/// cpu.cycle(&mut memory);
/// assert_eq!(memory.read_byte(0x1000), 0x51);
/// assert_eq!(cpu.get_f(), 0x00);
/// ```
pub fn inc_hl(cpu: &mut Cpu, memory: &mut Mmu) -> u8 {
    let value = memory.read_byte(cpu.get_rr(HL));
    memory.write_byte(cpu.get_rr(HL), value.wrapping_add(1));

    cpu.set_flag(Zero, value.wrapping_add(1) == 0);
    cpu.set_flag(Subtract, false);
    cpu.set_flag(HalfCarry, value & 0xf == 0xf);

    12
}

/// Decrements by 1 the contents of memory specified by register pair HL.
/// ```rust
/// //Example: When (HL) = 0x00,
/// //DEC (HL) ; (HL) <- 0xFF, Z <- 0, N <- 1, H <- 1
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Mmu::new();
/// # memory.set_bios_enabled(false);
/// # memory.write_byte(0x00, 0x35);
/// # cpu.set_h(0x10);
/// # cpu.set_l(0x00);
/// # memory.write_byte(0x1000, 0x00);
/// cpu.cycle(&mut memory);
/// assert_eq!(memory.read_byte(0x1000), 0xFF);
/// assert_eq!(cpu.get_f(), 0x60);
/// ```
pub fn dec_hl(cpu: &mut Cpu, memory: &mut Mmu) -> u8 {
    let value = memory.read_byte(cpu.get_rr(HL));
    memory.write_byte(cpu.get_rr(HL), value.wrapping_sub(1));

    cpu.set_flag(Zero, value.wrapping_sub(1) == 0);
    cpu.set_flag(Subtract, true);
    cpu.set_flag(HalfCarry, value & 0xf == 0);

    12
}

/// Adds the contents of the 8-bit immediate operand e and SP and stores the results in SP.
/// ```rust
/// //Example: When SP = 0xFFF8,
/// //ADD SP, 2 ; SP <- 0xFFFA, Z <- 0,  N <- 0, H <- 0, CY <- 0
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Mmu::new();
/// # memory.set_bios_enabled(false);
/// # memory.write_byte(0x00, 0xe8);
/// # memory.write_byte(0x01, 0x02);
/// # cpu.set_sp(0xfff8);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_sp(), 0xfffa);
/// assert_eq!(cpu.get_f(), 0);
///
/// //Example: When SP = 0xFFF8,
/// //ADD SP, -2 ; SP <- 0xFFF6, Z <- 0,  N <- 0, H <- 1, CY <- 1
/// # cpu.set_sp(0xfff8);
/// # memory.write_byte(0x02, 0xe8);
/// # memory.write_byte(0x03, 0xfe);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_sp(), 0xfff6);
/// assert_eq!(cpu.get_f(), 0x30);
/// ```
pub fn add_sp_n(cpu: &mut Cpu, memory: &mut Mmu) -> u8 {
    let n = cpu.read_n(memory) as i8 as u16;
    let sp = cpu.get_rr(SP);
    let result = cpu.sp.wrapping_add(n);
    cpu.sp = result;

    cpu.set_flag(Zero, false);
    cpu.set_flag(Subtract, false);
    cpu.set_flag(HalfCarry, (sp ^ n ^ result) & 0x10 == 0x10);
    cpu.set_flag(Carry, (sp ^ n ^ result) & 0x100 == 0x100);

    16
}
