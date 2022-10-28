use crate::gameboy::{ cpu::Cpu, memory::Memory };
use crate::gameboy::cpu::gmb_16_bit_loadcommands::{ push_rr, pop_rr };
use crate::gameboy::cpu::RegisterPair::PC;
use crate::gameboy::cpu::Flag;

/// Jumps -127 to +129 steps from the current address.
/// ``` rust
/// //JR NZ, 0x05 ; PC <- PC + 0x05
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Memory::new();
/// # memory.write_byte(0x00, 0x20);
/// # memory.write_byte(0x01, 0x05);
/// # cpu.set_f(0);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_pc(), 0x07);
/// 
/// //JR C, 0x80 ; PC <- PC - 127
/// # cpu.set_pc(0x100);
/// # memory.write_byte(0x100, 0x20);
/// # memory.write_byte(0x101, 0x80);
/// # cpu.set_f(0x10);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_pc(), 0x082);
/// ```
pub fn jr_cc_n(condition: Flag, cpu: &mut Cpu, memory: &mut Memory) -> u8 {
    let condition = cpu.get_flag(condition);
    let n = cpu.read_n(memory) as i8;
    let result = cpu.pc.wrapping_add(n as u16);
    
    if condition {
        cpu.pc = result;
        12
    } else {
        8
    }
    
}

pub fn jr_n(cpu: &mut Cpu, memory: &mut Memory) -> u8 {
    let n = cpu.read_n(memory) as i8;
    let pc = cpu.pc;
    cpu.pc = pc.wrapping_add(n as u16);
    12
}

/// In memory, pushes the PC value corresponding to the instruction at the address following that of the
/// CALL instruction to the 2 bytes following the byte specified by the current SP.
/// Operand nn is then loaded in the PC.
/// ```rust
/// //Examples: When PC = 0x8000 and SP = 0xFFFE,
/// //CALL 0x1234; (0xFFDH) <- 0x80, (0xFFCH) <- 0x03, SP <- 0xFFCH, PC <- 0x1234
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Memory::new();
/// # memory.write_byte(0x8000, 0xcd);
/// # cpu.set_pc(0x8000);
/// # cpu.set_sp(0xfffe);
/// # memory.write_byte(0x8001, 0x34);
/// # memory.write_byte(0x8002, 0x12);
/// cpu.cycle(&mut memory);
/// assert_eq!(memory.read_byte(0xfffd), 0x80);
/// assert_eq!(memory.read_byte(0xfffc), 0x03);
/// assert_eq!(cpu.get_sp(), 0xfffc);
/// assert_eq!(cpu.get_pc(), 0x1234);
/// ```
pub fn call_nn(cpu: &mut Cpu, memory: &mut Memory) -> u8 {
    let addr = cpu.read_nn(memory);
    push_rr(PC, cpu, memory);
    cpu.pc = addr;
    24
}

/// Pops from the memory stack the PC value pushed when the subroutine was called, returning control to the source program.
/// ```rust
/// // Examples: When PC = 0x8000; (0x9000) = 0xc9;
/// // CALL 0x9000; PC = 0x9000
/// //RET ; Returns to address 0x8003
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Memory::new();
/// # memory.write_byte(0x8000, 0xcd);
/// # memory.write_byte(0x8001, 0x00);
/// # memory.write_byte(0x8002, 0x90);
/// # memory.write_byte(0x9000, 0xc9);
/// # cpu.set_pc(0x8000);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_pc(), 0x9000);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_pc(), 0x8003);
/// ```
pub fn ret(cpu: &mut Cpu, memory: &mut Memory) -> u8 {
    pop_rr(PC, cpu, memory);

    16
}