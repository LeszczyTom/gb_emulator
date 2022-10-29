use crate::gameboy::cpu::Cpu;
use crate::gameboy::memory::Memory;
use crate::gameboy::cpu::Register;
use crate::gameboy::cpu::RegisterPair::HL;
use crate::gameboy::cpu::Flag::*;

/// Copies the complement of the contents of the specified bit in register r to the Z flag of the program status word (PSW).
/// 
/// ``` rust
/// //Examples: When A = 0x80 and L = 0xEF
/// //BIT 7, A ; Z <- 0, N <- 0, H <- 1
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Memory::new();
/// # memory.set_bios_enabled(false);
/// # memory.write_byte(0x00, 0xcb);
/// # memory.write_byte(0x01, 0x7f);
/// # cpu.set_a(0x80);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_f(), 0x20);
/// 
/// //BIT 4, L ; Z <- 1, N <- 0, H <- 1
/// # cpu.set_pc(0x00);
/// # cpu.set_l(0xef);
/// # memory.write_byte(0x00, 0xcb);
/// # memory.write_byte(0x01, 0x65);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_f(), 0xa0);
/// ```
pub fn bit_r(r: Register ,b: u8, cpu: &mut Cpu) -> u8 {

    cpu.set_flag(Zero, cpu.get_r(r) & (1 << b) == 0);
    cpu.set_flag(Subtract, false);
    cpu.set_flag(HalfCarry, true);

    8
}

/// Copies the complement of the contents of the specified bit in memory specified by the contents of register pair HL to the Z flag of the program status word (PSW).
/// ``` rust
/// //Examples: When (HL) = 0xFE,
/// //BIT 0, (HL) ; Z <- 1, N <- 0, H <- 1,
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Memory::new();
/// # memory.set_bios_enabled(false);
/// # memory.write_byte(0x00, 0xcb);
/// # memory.write_byte(0x01, 0x46);
/// # cpu.set_h(0x10);
/// # cpu.set_l(0x00);
/// # memory.write_byte(0x1000, 0xfe);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_f(), 0xa0);
/// 
/// //BIT 1, (HL) ; Z <- 0, N <- 0, H <- 1
/// # cpu.set_pc(0x00);
/// # memory.write_byte(0x00, 0xcb);
/// # memory.write_byte(0x01, 0x4e);
/// # memory.write_byte(0x1000, 0xfe);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_f(), 0x20);
/// ```
pub fn bit_hl(b: u8, cpu: &mut Cpu, memory: &mut Memory) -> u8 {

    cpu.set_flag(Zero, memory.read_byte(cpu.get_rr(HL)) & (1 << b) == 0);
    cpu.set_flag(Subtract, false);
    cpu.set_flag(HalfCarry, true);

    12
}
