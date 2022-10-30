use crate::gameboy::{ cpu::Cpu, memory::Memory };
use crate::gameboy::cpu::RegisterPair;
use crate::gameboy::cpu::Flag::*;

/// Loads 2 bytes of immediate data to register pair rr.
/// 
/// ``` rust
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Memory::new();
/// # memory.set_bios_enabled(false);
/// # memory.write_byte(0x00, 0x21);
/// # memory.write_byte(0x01, 0x5b);
/// # memory.write_byte(0x02, 0x3a);
/// //Example: LD HL, 0x3A5B ; H <- 0x3A, L <- 0x5B
/// 
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_h(), 0x3a);
/// assert_eq!(cpu.get_l(), 0x5b);
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
/// # memory.set_bios_enabled(false);
/// # memory.write_byte(0x00, 0xc5);
/// # cpu.set_sp(0xfffe);
/// # cpu.set_b(0x12);
/// # cpu.set_c(0x34);
/// cpu.cycle(&mut memory);
/// assert_eq!(memory.read_byte(0xfffd), 0x12);
/// assert_eq!(memory.read_byte(0xfffc), 0x34);
/// assert_eq!(cpu.get_sp(), 0xfffc);
/// ```
pub fn push_rr(rr: RegisterPair, cpu: &mut Cpu, memory: &mut Memory) -> u8 {
    let value = cpu.get_rr(rr).to_be_bytes();
    cpu.sp = cpu.sp.wrapping_sub(1);
    memory.write_byte(cpu.sp, value[0]);
    cpu.sp = cpu.sp.wrapping_sub(1);
    memory.write_byte(cpu.sp, value[1]);
    16
}

/// Pops contents from the memory stack and into register pair rr.
/// ```rust
/// //Example: When SP = 0xFFFC, (0xFFFC) = 0x5F, and (0xFFFD) = 0x3C,
/// //POP BC ; B <- 0x3C, C <- 0x5F, SP <- 0xFFFE
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Memory::new();
/// # memory.set_bios_enabled(false);
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
    cpu.sp = cpu.sp.wrapping_add(1);
    let high = memory.read_byte(cpu.sp);
    cpu.sp = cpu.sp.wrapping_add(1);

    cpu.set_rr(rr, u16::from_be_bytes([high, low]));
    
    cpu.set_f(cpu.get_f() & 0xf0);
    12
}

/// Stores the lower byte of SP at address nn specified by the 16-bit immediate operand nn and the upper byte of SP at address nn + 1.
/// ```rust	
/// //Example: When SP = 0xFFF8,
/// //LD (0xc100) , SP ; 0xc100 <- 0xF8, 0xc101 <- 0xFF
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Memory::new();
/// # memory.set_bios_enabled(false);
/// # memory.write_byte(0x00, 0x08);
/// # memory.write_byte(0x01, 0x00);
/// # memory.write_byte(0x02, 0x01);
/// # cpu.set_sp(0xfff8);
/// cpu.cycle(&mut memory);
/// assert_eq!(memory.read_byte(0x100), 0xf8);
/// assert_eq!(memory.read_byte(0x101), 0xff);
/// ```
pub fn ld_nn_sp(cpu: &mut Cpu, memory: &mut Memory) -> u8 {
    let nn = cpu.read_nn(memory);
    memory.write_byte(nn, cpu.sp as u8);
    memory.write_byte(nn.wrapping_add(1), (cpu.sp >> 8) as u8);
    20
}

/// The 8-bit operand e is added to SP and the result is stored in HL
/// ```rust
/// //Example: When SP = 0xFFF8,
/// //LDHL SP, 2 ; HL <- 0xFFFA, Z <- 0,  N <- 0, H <- 0, CY <- 0
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Memory::new();
/// # memory.set_bios_enabled(false);
/// # memory.write_byte(0x00, 0xf8);
/// # memory.write_byte(0x01, 0x02);
/// # cpu.set_sp(0xfff8);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_hl(), 0xfffa);
/// assert_eq!(cpu.get_f(), 0);
/// 
/// //Example: When SP = 0xFFFF,
/// //LDHL SP, -2 ; HL <- 0xFFFE, Z <- 0,  N <- 0, H <- 0, CY <- 0
/// # cpu.set_sp(0xfff8);
/// # memory.write_byte(0x02, 0xf8);
/// # memory.write_byte(0x03, 0xfe);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_hl(), 0xfff6);
/// //assert_eq!(cpu.get_f(), 0x0);
/// ```
pub fn ldhl_sp_n(cpu: &mut Cpu, memory: &mut Memory) -> u8 { //TODO fix flags
    let n = cpu.read_n(memory) as i8 as u16;
    let sp = cpu.sp;
    let result = cpu.sp.wrapping_add(n);
    cpu.set_rr(RegisterPair::HL, result);

    cpu.set_flag(Zero, false);
    cpu.set_flag(Subtract, false);
    cpu.set_flag(HalfCarry, (sp & 0xfff).wrapping_add(n & 0xfff) > 0xfff);
    cpu.set_flag(Carry, (sp).overflowing_add(n).1 == true);
    
    12
}

/// Load the contents of register pair HL in stack pointer SP.
pub fn ld_sp_hl(cpu: &mut Cpu) -> u8 {
    cpu.sp = cpu.get_hl();
    8
}