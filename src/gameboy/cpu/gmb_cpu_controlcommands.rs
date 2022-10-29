use crate::gameboy::cpu::Cpu;
use crate::gameboy::cpu::Flag::*;

/// Sets the interrupt master enable flag and enables maskable interrupts.
/// This instruction can be used in an interrupt routine to enable higher-order interrupts.
/// ``` rust
///     //EI ; IME <- 1
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Memory::new();
/// # memory.set_bios_enabled(false);
/// # memory.write_byte(0x00, 0xfb);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_ime(), true);
/// ```
pub fn ei(cpu: &mut Cpu) -> u8 {
    cpu._ime = true;
    4
}

/// Execution of a STOP instruction stops both the system clock and oscillator circuit. STOP mode is entered, and the LCD controller also stops.
pub fn stop() ->u8{
    panic!("Stop");
    // TODO: 
    // The following conditions should be met before a STOP instruction is executed and STOP mode is entered
    // All interrupt-enable (IE) flags are reset.
    // Input to PI 0 — PI 3 is LOW for all.
}

/// When performing addition and subtraction, binary coded decimal representation is used to set the contents of register A to a binary coded decimal number (BCD).
/// ``` rust
/// //Examples: When A = 0x45 and B = 0x38,
/// //ADD A, B ; A <- 0x7D, N <- 0
/// //DAA ; A <- 0x7D + 0x06 (0x83), CY <- 0
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Memory::new();
/// # use gameboy::gameboy::cpu::Flag::Subtract;
/// # memory.set_bios_enabled(false);
/// # cpu.set_a(0x45);
/// # cpu.set_b(0x38);
/// # memory.write_byte(0x00, 0x80);
/// # memory.write_byte(0x01, 0x27);
/// cpu.cycle(&mut memory);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_a(), 0x83);
/// assert_eq!(cpu.get_f() >> 4, 0);
/// 
/// //SUB A, B ; A <- 0x83 - 0x38 (0x4B), N <- 1
/// //DAA ; A <- 0x4B + 0xFA (0x45)
/// # cpu.set_a(0x83);
/// # cpu.set_b(0x38);
/// # memory.write_byte(0x02, 0x90);
/// # memory.write_byte(0x03, 0x27);
/// cpu.cycle(&mut memory);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_a(), 0x45);
/// assert_eq!(cpu.get_flag(Subtract), true);
/// ```
pub fn daa(cpu: &mut Cpu) -> u8 {
    let mut c = false;
    if !cpu.get_flag(Subtract) {
        if cpu.get_flag(Carry) || cpu.a > 0x99 {
            cpu.a = cpu.a.wrapping_add(0x60);
            c = true;
        }
        if cpu.get_flag(HalfCarry) || cpu.a & 0xf > 0x9 {
            cpu.a = cpu.a.wrapping_add(0x6);
        }
    } else if cpu.get_flag(Carry) {
        c = true;
        cpu.a = cpu.a.wrapping_add(
            if cpu.get_flag(HalfCarry) { 0x9a } else { 0xa0 }
        );
    } else if cpu.get_flag(HalfCarry) {
        cpu.a = cpu.a.wrapping_add(0xfa);
    }

    cpu.set_flag(Zero, cpu.a == 0);
    cpu.set_flag(HalfCarry, false);
    cpu.set_flag(Carry, c);
    4
}