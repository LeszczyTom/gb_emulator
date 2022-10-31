use crate::gameboy::{ cpu::Cpu, memory::Memory };
use crate::gameboy::cpu::Register;
use crate::gameboy::cpu::RegisterPair::HL;
use crate::gameboy::cpu::Flag::*;

/// Rotates the contents of operand m to the left, r and (HL) are used for operand m.
/// ```rust
/// //Examples: When C = 0x80, (HL) = 0x11, and CY = 0,
/// //RL C ; C <- 0x00, Z <- 1, N <- 0, H <- 0, CY <- 1
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Memory::new();
/// # memory.set_bios_enabled(false);
/// # memory.write_byte(0x00, 0xcb);
/// # memory.write_byte(0x01, 0x11);
/// # cpu.set_c(0x80);
/// # cpu.set_f(0);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_c(), 0x00);
/// assert_eq!(cpu.get_f(), 0x90);
/// //RL (HL) ; (HL) <- 0x22,  Z <- 0, N <- 0, H <- 0, CY <- 0
/// # memory.write_byte(0x02, 0xcb);
/// # memory.write_byte(0x03, 0x16);
/// # cpu.set_h(0x10);
/// # cpu.set_l(0x00);
/// # cpu.set_f(0);
/// # memory.write_byte(0x1000, 0x11);
/// cpu.cycle(&mut memory);
/// assert_eq!(memory.read_byte(0x1000), 0x22);
/// assert_eq!(cpu.get_f(), 0x00);
/// ```
fn rl_m(value: u8, cpu: &mut Cpu) -> u8 {
    let carry = cpu.get_flag(Carry);
    let result = (value << 1) | carry as u8;

    cpu.set_flag(Zero, result == 0);
    cpu.set_flag(Subtract, false);
    cpu.set_flag(HalfCarry, false);
    cpu.set_flag(Carry, value >> 7 == 1);

    result
}

pub fn rl_r(r: Register, cpu: &mut Cpu) -> u8 {
    let result = rl_m(cpu.get_r(r.clone()), cpu);
    cpu.set_r(r, result);

    8
}

pub fn rl_hl(cpu: &mut Cpu, memory: &mut Memory) -> u8 {
    let result = rl_m(memory.read_byte(cpu.get_rr(HL)), cpu);
    memory.write_byte(cpu.get_rr(HL), result);

    16
}

/// Rotates the contents of register A to the left.
/// ```rust	
/// //Example: When A = 0x95 and CY = 1,
/// //RLA ; A <- 0x2B, Z <- 0, N <- 0, H <- 0, C <- 1,
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Memory::new();
/// # memory.set_bios_enabled(false);
/// # memory.write_byte(0x00, 0x17);
/// # cpu.set_a(0x95);
/// # cpu.set_f(0x10);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_a(), 0x2b);
/// assert_eq!(cpu.get_f(), 0x10);
/// ```
pub fn rla(cpu: &mut Cpu) -> u8 {
    let value = cpu.a;
    let carry = cpu.get_flag(Carry);
    let result = (value << 1) | carry as u8;

    cpu.set_flag(Zero, false);
    cpu.set_flag(Subtract, false);
    cpu.set_flag(HalfCarry, false);
    cpu.set_flag(Carry, value >> 7 == 1);

    cpu.a = result;
    4
}

/// Rotates the contents of register A to the right.
/// ```rust
/// //Example: When A = 0x81 and CY = 0,
/// //RRA ; A <- 0x40,  Z <- 0, N <- 0, H <- 0, CY <- 1 ,
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Memory::new();
/// # memory.set_bios_enabled(false);
/// # memory.write_byte(0x00, 0x1f);
/// # cpu.set_a(0x81);
/// # cpu.set_f(0);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_a(), 0x40);
/// assert_eq!(cpu.get_f(), 0x10);
/// ```
pub fn rra(cpu: &mut Cpu) -> u8 {
    let value = cpu.a;
    let carry = cpu.get_flag(Carry);
    let result = (value >> 1) | (carry as u8) << 7;

    cpu.set_flag(Zero, false);
    cpu.set_flag(Subtract, false);
    cpu.set_flag(HalfCarry, false);
    cpu.set_flag(Carry, value & 0x01 == 1);

    cpu.a = result;
    4
}

/// Rotates the contents of register A to the left.
/// ```rust
/// //Example: When A = 0x85 and CY = 0,
/// //RLCA ; A <- 0x0b,  Z <- 0, N <- 0, H <- 0, CY <- 1
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Memory::new();
/// # memory.set_bios_enabled(false);
/// # memory.write_byte(0x00, 0x07);
/// # cpu.set_a(0x85);
/// # cpu.set_f(0);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_a(), 0x0b);
/// assert_eq!(cpu.get_f(), 0x10);
/// ```
pub fn rlca(cpu: &mut Cpu) -> u8 {
    let value = cpu.a;
    cpu.a = (value << 1) | (value >> 7);

    cpu.set_flag(Zero, false);
    cpu.set_flag(Subtract, false);
    cpu.set_flag(HalfCarry, false);
    cpu.set_flag(Carry, value > 0x7f);

    4
}

/// Rotates the contents of register A to the right.
/// ```rust
/// //Example: When A = 0x3B and CY = 0,
/// //RRCA ; A <- 0x9D, Z <- 0, N <- 0, H <- 0, CY <- 1
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Memory::new();
/// # memory.set_bios_enabled(false);
/// # memory.write_byte(0x00, 0x0f);
/// # cpu.set_a(0x3b);
/// # cpu.set_f(0);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_a(), 0x9d);
/// assert_eq!(cpu.get_f(), 0x10);
/// ```
pub fn rrca(cpu: &mut Cpu) -> u8 {
    let value = cpu.a;
    let result = value.rotate_right(1);

    cpu.set_flag(Zero, false);
    cpu.set_flag(Subtract, false);
    cpu.set_flag(HalfCarry, false);
    cpu.set_flag(Carry, value & 0x01 == 1);

    cpu.a = result;
    4
}

/// Rotates the contents of operand m to the left, r and (HL) are used for operand m.
fn rlc_m(value: u8, cpu: &mut Cpu) -> u8 {
    let result = value.rotate_left(1);

    cpu.set_flag(Zero, result == 0);
    cpu.set_flag(Subtract, false);
    cpu.set_flag(HalfCarry, false);
    cpu.set_flag(Carry, value >> 7 == 1);

    result
}

/// Rotates the contents of register r to the left.
/// ```rust
/// //Examples: When B = 0x85, and CY = 0,
/// //RLC B ; B <- 0x0b, Z <- 0, N <- 0, H <- 0, CY <- 1
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Memory::new();
/// # memory.set_bios_enabled(false);
/// # memory.write_byte(0x00, 0xcb);
/// # memory.write_byte(0x01, 0x00);
/// # cpu.set_b(0x85);
/// # cpu.set_f(0);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_b(), 0x0b);
/// assert_eq!(cpu.get_f(), 0x10);
/// ```
pub fn rlc_r(r: Register, cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r(r.clone());
    let result = rlc_m(value, cpu);
    cpu.set_r(r, result);

    8
}

/// Rotates the contents of (HL) to the left.
/// ```rust
/// //Examples: When (HL) = 0, and CY = 0,
/// //RLC (HL) ; (HL) <- 0x00, Z <- 1 , N <- 0, H <- 0, CY <- 0
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Memory::new();
/// # memory.set_bios_enabled(false);
/// # memory.write_byte(0x00, 0xcb);
/// # memory.write_byte(0x01, 0x06);
/// # memory.write_byte(0xff00, 0x00);
/// # cpu.set_h(0xff);
/// # cpu.set_l(0x00);
/// # cpu.set_f(0);
/// cpu.cycle(&mut memory);
/// assert_eq!(memory.read_byte(0xff00), 0x00);
/// assert_eq!(cpu.get_f(), 0x80);
/// ```
pub fn rlc_hl(cpu: &mut Cpu, memory: &mut Memory) -> u8 {
    let value = memory.read_byte(cpu.get_hl());
    let result = rlc_m(value, cpu);
    memory.write_byte(cpu.get_hl(), result);

    16
}

/// Rotates the contents of operand m to the right, r and (HL) are used for operand m.
fn rrc_m(value: u8, cpu: &mut Cpu) -> u8 {
    let result = value.rotate_right(1);

    cpu.set_flag(Zero, result == 0);
    cpu.set_flag(Subtract, false);
    cpu.set_flag(HalfCarry, false);
    cpu.set_flag(Carry, value & 0x01 == 1);

    result
}

/// Rotates the contents of register r to the right.
/// ```rust
/// //Examples: When C = 1, CY = 0,
/// //RRC C ; C <- 0x80, Z <- 0, N <- 0, H <- 0, CY <- 1
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Memory::new();
/// # memory.set_bios_enabled(false);
/// # memory.write_byte(0x00, 0xcb);
/// # memory.write_byte(0x01, 0x09);
/// # cpu.set_c(1);
/// # cpu.set_f(0);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_c(), 0x80);
/// assert_eq!(cpu.get_f(), 0x10);
/// ```
pub fn rrc_r(r: Register, cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r(r.clone());
    let result = rrc_m(value, cpu);
    cpu.set_r(r, result);

    8
}

/// Rotates the contents of (HL) to the right.
/// ```rust
/// //Examples: When (HL) = 0, CY = 0,
/// //RRC (HL) ; (HL) <- 0, Z <- 1, N <- 0, H <- 0, CY <- 0
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Memory::new();
/// # memory.set_bios_enabled(false);
/// # memory.write_byte(0x00, 0xcb);
/// # memory.write_byte(0x01, 0x0e);
/// # memory.write_byte(0xff00, 0x00);
/// # cpu.set_h(0xff);
/// # cpu.set_l(0x00);
/// # cpu.set_f(0);
/// cpu.cycle(&mut memory);
/// assert_eq!(memory.read_byte(0xff00), 0x00);
/// assert_eq!(cpu.get_f(), 0x80);
/// ```
pub fn rrc_hl(cpu: &mut Cpu, memory: &mut Memory) -> u8 {
    let value = memory.read_byte(cpu.get_hl());
    let result = rrc_m(value, cpu);
    memory.write_byte(cpu.get_hl(), result);

    16
}

/// Rotates the contents of operand m to the right, r and (HL) are used for operand m.
fn rr_m(value: u8, cpu: &mut Cpu) -> u8 {
    let carry = cpu.get_flag(Carry) as u8;
    let result = (value >> 1) | (carry << 7);

    cpu.set_flag(Zero, result == 0);
    cpu.set_flag(Subtract, false);
    cpu.set_flag(HalfCarry, false);
    cpu.set_flag(Carry, value & 0x01 == 1);

    result
}

/// Rotates the contents of register r to the right.
/// ```rust
/// //Examples: When A = 1, CY = 0,
/// //RR A ; A <- 0, Z <- 1, N <- 0, H 0, CY <- 1
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Memory::new();
/// # memory.set_bios_enabled(false);
/// # memory.write_byte(0x00, 0xcb);
/// # memory.write_byte(0x01, 0x1f);
/// # cpu.set_a(1);
/// # cpu.set_f(0);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_a(), 0x00);
/// assert_eq!(cpu.get_f(), 0x90);  
/// ```
pub fn rr_r(r: Register, cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r(r.clone());
    let result = rr_m(value, cpu);
    cpu.set_r(r, result);

    8
}

/// Rotates the contents of (HL) to the right.
/// ```rust
/// //Examples: When (hl) = 0x8a, CY = 0,
/// //RR (HL) ; (HL) <- 45h, Z <- 0, N <- 0, H w- 0, CY <- 0
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Memory::new();
/// # memory.set_bios_enabled(false);
/// # memory.write_byte(0x00, 0xcb);
/// # memory.write_byte(0x01, 0x1e);
/// # memory.write_byte(0xff00, 0x8a);
/// # cpu.set_h(0xff);
/// # cpu.set_l(0x00);
/// # cpu.set_f(0);
/// cpu.cycle(&mut memory);
/// assert_eq!(memory.read_byte(0xff00), 0x45);
/// assert_eq!(cpu.get_f(), 0x00);
/// ```
pub fn rr_hl(cpu: &mut Cpu, memory: &mut Memory) -> u8 {
    let value = memory.read_byte(cpu.get_hl());
    let result = rr_m(value, cpu);
    memory.write_byte(cpu.get_hl(), result);

    16
}

fn sla_m(value: u8, cpu: &mut Cpu) -> u8 {
    let result = value << 1;

    cpu.set_flag(Zero, result == 0);
    cpu.set_flag(Subtract, false);
    cpu.set_flag(HalfCarry, false);
    cpu.set_flag(Carry, value & 0x80 == 0x80);

    result
}

/// ```rust
/// //Examples: When D = 80h, (HL) = FFh, and CY = 0,
/// //SLA D ; D <- 0, Z <- 1, N <- 0, H <- 0,  CY <- 1 
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Memory::new();
/// # memory.set_bios_enabled(false);
/// # memory.write_byte(0x00, 0xcb);
/// # memory.write_byte(0x01, 0x22);
/// # cpu.set_d(0x80);
/// # cpu.set_f(0);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_d(), 0x00);
/// assert_eq!(cpu.get_f(), 0x90);
/// ```
pub fn sla_r(r: Register, cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r(r.clone());
    let result = sla_m(value, cpu);
    cpu.set_r(r, result);

    8
}

/// ```rust
/// //Examples: When D = 80h, (HL) = FFh, and CY = 0,
/// //SLA (HL) ; (HL) <- FEh, Z <- 0, N <- 0, H <- 0, CY <- 1
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Memory::new();
/// # memory.set_bios_enabled(false);
/// # memory.write_byte(0x00, 0xcb);
/// # memory.write_byte(0x01, 0x26);
/// # memory.write_byte(0xff00, 0xff);
/// # cpu.set_h(0xff);
/// # cpu.set_l(0x00);
/// # cpu.set_f(0);
/// cpu.cycle(&mut memory);
/// assert_eq!(memory.read_byte(0xff00), 0xfe);
/// assert_eq!(cpu.get_f(), 0x10);
/// ```
pub fn sla_hl(cpu: &mut Cpu, memory: &mut Memory) -> u8 {
    let value = memory.read_byte(cpu.get_hl());
    let result = sla_m(value, cpu);
    memory.write_byte(cpu.get_hl(), result);

    16
}

fn sra_m(value: u8, cpu: &mut Cpu) -> u8 {
    let result = (value >> 1) | (value & 0x80);

    cpu.set_flag(Zero, result == 0);
    cpu.set_flag(Subtract, false);
    cpu.set_flag(HalfCarry, false);
    cpu.set_flag(Carry, value & 0x01 == 0x01);

    result
}

/// ```rust
/// //Example: When A = 0x8A, (HL) = 0x01, and CY = 0,
/// //SRA A ; A <- 0xC5, Z <- 0, N <- 0, H <- 0, CY <- 0,
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Memory::new();
/// # memory.set_bios_enabled(false);
/// # memory.write_byte(0x00, 0xcb);
/// # memory.write_byte(0x01, 0x2f);
/// # cpu.set_a(0x8a);
/// # cpu.set_f(0);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_a(), 0xc5);
/// assert_eq!(cpu.get_f(), 0x00);
/// ```
pub fn sra_r(r: Register, cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r(r.clone());
    let result = sra_m(value, cpu);
    cpu.set_r(r, result);
    8
}

/// ```rust
/// //Example: When A = 0x8A, (HL) = 0x01, and CY = 0,
/// //SRA (HL) ; (HL) <- 0,  Z <- 1, N <- 0, H <- 0, CY <- 1
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Memory::new();
/// # memory.set_bios_enabled(false);
/// # memory.write_byte(0x00, 0xcb);
/// # memory.write_byte(0x01, 0x2e);
/// # memory.write_byte(0xff00, 0x01);
/// # cpu.set_h(0xff);
/// # cpu.set_l(0x00);
/// # cpu.set_f(0);
/// cpu.cycle(&mut memory);
/// assert_eq!(memory.read_byte(0xff00), 0x00);
/// assert_eq!(cpu.get_f(), 0x90);
/// ```
pub fn sra_hl(cpu: &mut Cpu, memory: &mut Memory) -> u8 {
    let value = memory.read_byte(cpu.get_hl());
    let result = sra_m(value, cpu);
    memory.write_byte(cpu.get_hl(), result);

    16
}

fn swap_m(value: u8, cpu: &mut Cpu) -> u8 {
    let result = value << 4 & 240 | value >> 4;

    cpu.set_flag(Zero, result == 0);
    cpu.set_flag(Subtract, false);
    cpu.set_flag(HalfCarry, false);
    cpu.set_flag(Carry, false);

    result
}

/// ```rust
/// //xamples: When A = 0 and (HL) = 0xFO,
/// //SWAP A ; A <- 0, Z <- 1 , N <- 0, H <- 0, CY <- 0 
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Memory::new();
/// # memory.set_bios_enabled(false);
/// # memory.write_byte(0x00, 0xcb);
/// # memory.write_byte(0x01, 0x37);
/// # cpu.set_a(0);
/// # cpu.set_f(0);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_a(), 0);
/// assert_eq!(cpu.get_f(), 0x80);
/// ```
pub fn swap_r(r: Register, cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r(r.clone());
    let result = swap_m(value, cpu);
    cpu.set_r(r, result);

    8
}

/// ```rust
/// //Examples: When A = 0 and (HL) = 0xFO,
/// //SWAP (HL) ; (HL) <- 0x0f, Z <- 0, N <- 0, H <- 0, CY <- 0
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Memory::new();
/// # memory.set_bios_enabled(false);
/// # memory.write_byte(0x00, 0xcb);
/// # memory.write_byte(0x01, 0x36);
/// # memory.write_byte(0xff00, 0x0f);
/// # cpu.set_h(0xff);
/// # cpu.set_l(0x00);
/// # cpu.set_f(0);
/// cpu.cycle(&mut memory);
/// assert_eq!(memory.read_byte(0xff00), 0x0f);
/// assert_eq!(cpu.get_f(), 0x00);
/// ```
pub fn swap_hl(cpu: &mut Cpu, memory: &mut Memory) -> u8 {
    let value = memory.read_byte(cpu.get_hl());
    let result = swap_m(value, cpu);
    memory.write_byte(cpu.get_hl(), result);

    16
}

fn srl_m(value: u8, cpu: &mut Cpu) -> u8 {
    let result = value >> 1;
    cpu.set_flag(Zero, result == 0);
    cpu.set_flag(Subtract, false);
    cpu.set_flag(HalfCarry, false);
    cpu.set_flag(Carry, value & 0x01 == 0x01);

    result
}

/// ```rust
/// //Examples: When A = 1, (HL) = 0xFF, CY = 0,
/// //SRL A ; A <- 0, Z <- 1 , N <- 0 H <- 0, CY <- 1 
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Memory::new();
/// # memory.set_bios_enabled(false);
/// # memory.write_byte(0x00, 0xcb);
/// # memory.write_byte(0x01, 0x3f);
/// # cpu.set_a(1);
/// # cpu.set_f(0);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_a(), 0);
/// assert_eq!(cpu.get_f(), 0x90);
/// ```
pub fn srl_r(r: Register, cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r(r.clone());
    let result = srl_m(value, cpu);
    cpu.set_r(r, result);

    8
}

/// ```rust
/// //Examples: When A = 1, (HL) = 0xFF, CY = 0,
/// //SRL (HL) ; (HL) <- 0x7F, Z <- 0, N <- 0, H <- 0, CY <- 1
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Memory::new();
/// # memory.set_bios_enabled(false);
/// # memory.write_byte(0x00, 0xcb);
/// # memory.write_byte(0x01, 0x3e);
/// # memory.write_byte(0xff00, 0xff);
/// # cpu.set_h(0xff);
/// # cpu.set_l(0x00);
/// # cpu.set_f(0);
/// cpu.cycle(&mut memory);
/// assert_eq!(memory.read_byte(0xff00), 0x7f);
/// assert_eq!(cpu.get_f(), 0x10);
/// ```
pub fn srl_hl(cpu: &mut Cpu, memory: &mut Memory) -> u8 {
    let value = memory.read_byte(cpu.get_hl());
    let result = srl_m(value, cpu);
    memory.write_byte(cpu.get_hl(), result);

    16
}