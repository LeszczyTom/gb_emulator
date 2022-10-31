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

/// Resets to 0 the specified bit in the specified register r.
/// ``` rust
/// //Example: When A = 0x80 and L = 0x3B,
/// //RES 7, A ; A <- 0
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Memory::new();
/// # memory.set_bios_enabled(false);
/// # memory.write_byte(0x00, 0xcb);
/// # memory.write_byte(0x01, 0xbf);
/// # cpu.set_a(0x80);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_a(), 0x00);
/// 
/// //RES 1, L ; L <- 0x39
/// # memory.write_byte(0x02, 0xcb);
/// # memory.write_byte(0x03, 0x8d);
/// # cpu.set_l(0x3b);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_l(), 0x39);
/// ``` 
pub fn res_r(r: Register, bit: u8, cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r(r.clone());
    cpu.set_r(r, value & !(1 << bit));

    8
}

/// Resets to 0 the specified bit in the memory contents specified by registers H and L.
/// ``` rust
/// //Example: When 0xFF is the memory contents specified by H and L,
/// //RES 3, (HL) ; (HL) <- 0xF7
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Memory::new();
/// # memory.set_bios_enabled(false);
/// # memory.write_byte(0x00, 0xcb);
/// # memory.write_byte(0x01, 0x9e);
/// # cpu.set_h(0x10);
/// # cpu.set_l(0x00);
/// # memory.write_byte(0x1000, 0xff);
/// cpu.cycle(&mut memory);
/// assert_eq!(memory.read_byte(0x1000), 0xf7);
/// ```
pub fn res_hl(bit: u8, cpu: &mut Cpu, memory: &mut Memory) -> u8 {
    let value = memory.read_byte(cpu.get_hl());
    memory.write_byte(cpu.get_hl(),  value & !(1 << bit));

    16
}

/// Sets to 1 the specified bit in specified register r.
/// ``` rust	
/// //Example: When A = 0x80 and L = 0x3B,
/// //SET 3, A ; A <- 0x84
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Memory::new();
/// # memory.set_bios_enabled(false);
/// # memory.write_byte(0x00, 0xcb);
/// # memory.write_byte(0x01, 0xdf);
/// # cpu.set_a(0x80);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_a(), 0x88);
/// 
/// //SET 7, L ; L <- 0xBB
/// # memory.write_byte(0x02, 0xcb);
/// # memory.write_byte(0x03, 0xfd);
/// # cpu.set_l(0x3b);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_l(), 0xbb);
/// ``` 
pub fn set_r(r: Register, bit: u8, cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r(r.clone());
    cpu.set_r(r, value | (1 << bit));

    8
}

/// Sets to 1 the specified bit in the memory contents specified by registers H and L.
/// ``` rust
/// //Example: When 0x00 is the memory contents specified by H and L,
/// //SET 3, (HL) ; (HL) <- 0x08
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Memory::new();
/// # memory.set_bios_enabled(false);
/// # memory.write_byte(0x00, 0xcb);
/// # memory.write_byte(0x01, 0xde);
/// # cpu.set_h(0x10);
/// # cpu.set_l(0x00);
/// # memory.write_byte(0x1000, 0x00);
/// cpu.cycle(&mut memory);
/// assert_eq!(memory.read_byte(0x1000), 0x08);
/// ```
pub fn set_hl(bit: u8, cpu: &mut Cpu, memory: &mut Memory) -> u8 {
    let value = memory.read_byte(cpu.get_hl());
    memory.write_byte(cpu.get_hl(),  value | (1 << bit));

    16
}