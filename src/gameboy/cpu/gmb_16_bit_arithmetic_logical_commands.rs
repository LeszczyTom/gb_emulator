use crate::gameboy::cpu::Cpu;
use crate::gameboy::cpu::{ Register, RegisterPair };
use crate::gameboy::cpu::Flag::*;

/// Increments the contents of register pair rr by 1.
/// ```rust
/// //Example: When DE = 0x235f
/// //INC DE ; DE <- 0x2360
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Memory::new();
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

/// Subtract 1 from the contents of register r by 1.
/// ```rust
/// //Example: When B = 0x01,
/// //DEC B ; B <- 0, Z <- 1, N <— 1 H <- 0,
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Memory::new();
/// # memory.write_byte(0x00, 0x05);
/// # cpu.set_b(0x01);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_b(), 0x00);
/// assert_eq!(cpu.get_f(), 0xc0);
/// ```
pub fn dec_r( r: Register, cpu: &mut Cpu) -> u8 {
    let value = cpu.get_r(r.clone());
    let result = value.wrapping_sub(1);

    cpu.set_flag(Zero, result == 0);
    cpu.set_flag(Subtract, true);
    cpu.set_flag(HalfCarry, (value & 0x0f) == 0);

    cpu.set_r(r, result);
    4
}