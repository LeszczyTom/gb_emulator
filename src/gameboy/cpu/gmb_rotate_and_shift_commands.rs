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
/// # memory.write_byte(0x100, 0x11);
/// cpu.cycle(&mut memory);
/// assert_eq!(memory.read_byte(0x100), 0x22);
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

pub fn _rl_hl(cpu: &mut Cpu, memory: &mut Memory) -> u8 {
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
/// # memory.write_byte(0x00, 0x17);
/// # cpu.set_r("a", 0x95);
/// # cpu.set_flag("c", true);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_r("a"), 0x2b);
/// assert_eq!(cpu.get_r("f"), 0x10);
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