use crate::gameboy::cpu::Cpu;

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