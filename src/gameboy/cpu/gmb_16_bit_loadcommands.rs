use crate::gameboy::{ cpu::Cpu, memory::Memory };
use crate::gameboy::cpu::RegisterPair;

/// Loads 2 bytes of immediate data to register pair rr.
/// 
/// ``` rust
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Memory::new();
/// # memory.write_byte(0x00, 0x21);
/// # memory.write_byte(0x01, 0x5b);
/// # memory.write_byte(0x02, 0x3a);
/// //Example: LD HL, 0x3A5B ; H <- 0x3A, L <- 0x5B
/// 
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_r("h"), 0x3a);
/// assert_eq!(cpu.get_r("l"), 0x5b);
/// ```
pub fn ld_rr_nn(rr: RegisterPair, cpu: &mut Cpu, memory: &mut Memory) -> u8 {
    let nn = cpu.read_nn(memory);
    cpu.set_rr(rr, nn);
    12
}

/// Pushes the contents of register pair rr onto the memory stack.
/// ```rust
/// //Example: When SP = 0xFFFE,
/// //PUSH BC ; (0xFFFC) <- C, (0xFFFD) <- B, SP <- 0xFFFC
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Memory::new();
/// # memory.write_byte(0x00, 0xc5);
/// # cpu.set_rr("sp", 0xfffe);
/// # cpu.set_rr("bc", 0x1234);
/// cpu.cycle(&mut memory);
/// assert_eq!(memory.read_byte(0xfffd), 0x12);
/// assert_eq!(memory.read_byte(0xfffc), 0x34);
/// assert_eq!(cpu.get_rr("sp"), 0xfffc);
/// ```
pub fn push_rr(rr: RegisterPair, cpu: &mut Cpu, memory: &mut Memory) -> u8 {
    let value = cpu.get_rr(rr).to_be_bytes();
    memory.write_byte(cpu.sp.wrapping_sub(1), value[0]);
    memory.write_byte(cpu.sp.wrapping_sub(2), value[1]);
    cpu.sp = cpu.sp.wrapping_sub(2);
    16
}

/// Pops contents from the memory stack and into register pair rr.
/// ```rust
/// //Example: When SP = 0xFFFC, (0xFFFC) = 0x5F, and (0xFFFD) = 0x3C,
/// //POP BC ; B <- 0x3C, C <- 0x5F, SP <- 0xFFFE
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Memory::new();
/// # memory.write_byte(0x00, 0xc1);
/// # cpu.set_sp(0xfffc);
/// # memory.write_byte(0xfffc, 0x5f);
/// # memory.write_byte(0xfffd, 0x3c);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_bc(), 0x3c5f);
/// assert_eq!(cpu.get_sp(), 0xfffe);
/// ```
pub fn pop_rr(rr: RegisterPair, cpu: &mut Cpu, memory: &mut Memory) -> u8 {
    let low = memory.read_byte(cpu.sp);
    let high = memory.read_byte(cpu.sp.wrapping_add(1));
    cpu.set_rr(rr, u16::from_be_bytes([high, low]));
    cpu.sp = cpu.sp.wrapping_add(2);
    
    12
}