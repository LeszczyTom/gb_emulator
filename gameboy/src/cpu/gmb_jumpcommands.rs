use crate::cpu::cpu::{
    Cpu,
    RegisterPair::{ PC, HL },
    Flag
};
use crate::cpu::gmb_16_bit_loadcommands::{ push_rr, pop_rr};
use crate::memory::mmu::Mmu;


/// Jumps -127 to +129 steps from the current address.
/// ``` rust
/// //JR NZ, 0x05 ; PC <- PC + 0x05
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Mmu::new();
/// # memory.set_bios_enabled(false);
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
pub fn jr_cc_n(condition: Flag, cpu: &mut Cpu, memory: &mut Mmu) -> u8 {
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

pub fn jr_n(cpu: &mut Cpu, memory: &mut Mmu) -> u8 {
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
/// # let mut memory = gameboy::gameboy::memory::Mmu::new();
/// # memory.set_bios_enabled(false);
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
pub fn call_nn(cpu: &mut Cpu, memory: &mut Mmu) -> u8 {
    let addr = cpu.read_nn(memory);
    push_rr(PC, cpu, memory);
    cpu.pc = addr;
    24
}

/// Pops from the memory stack the PC value pushed when the subroutine was called, returning control to the source program.
/// ```rust
/// //Examples: When PC = 0x8000; (0x9000) = 0xc9;
/// //CALL 0x9000; PC = 0x9000
/// //RET ; Returns to address 0x8003
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Mmu::new();
/// # memory.set_bios_enabled(false);
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
pub fn ret(cpu: &mut Cpu, memory: &mut Mmu) -> u8 {
    pop_rr(PC, cpu, memory);

    16
}

/// If condition cc and the flag match, control is returned to the source program by popping from the memory stack 
/// the PC value pushed to the stack when the subroutine was called.
/// ```rust
/// //Examples: When PC = 0x8000; (0x9000) = 0xfe, (0x9001) = 0, (0x9002) = 0xc8;
/// //CALL 0x9000; PC = 0x9000
/// //CP 0x00
/// //RET Z ; Returns to address 0x8003 if Z = 1, Moves to next instruction after 2 cycles if Z = 0.
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Mmu::new();
/// # memory.set_bios_enabled(false);
/// # cpu.set_pc(0x8000);
/// # memory.write_byte(0x8000, 0xcd);
/// # memory.write_byte(0x8001, 0x00);
/// # memory.write_byte(0x8002, 0x90);
/// # memory.write_byte(0x9000, 0xfe);
/// # memory.write_byte(0x9001, 0x00);
/// # memory.write_byte(0x9002, 0xc8);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_pc(), 0x9000);
/// cpu.cycle(&mut memory);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_pc(), 0x8003);
/// 
/// //When (0x9001) = 0x01
/// # cpu.set_pc(0x8000);
/// # memory.write_byte(0x9001, 0x01);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_pc(), 0x9000);
/// cpu.cycle(&mut memory);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_pc(), 0x9003);
/// ```
pub fn ret_cc(condition: Flag, cpu: &mut Cpu, memory: &mut Mmu) -> u8 {
    if cpu.get_flag(condition) {
        pop_rr(PC, cpu, memory);
        20
    } else {
        8
    }
}

/// Loads the operand nn to the program counter (PC).
/// ```rust
/// //Example: JP 0x8000 ; Jump to 0x8000.
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Mmu::new();
/// # memory.set_bios_enabled(false);
/// # memory.write_byte(0x0, 0xc3);
/// # memory.write_byte(0x1, 0x00);
/// # memory.write_byte(0x2, 0x80);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_pc(), 0x8000);
/// ```
pub fn jp_nn(cpu: &mut Cpu, memory: &mut Mmu) -> u8 {
    let addr = cpu.read_nn(memory);
    cpu.pc = addr;

    16
}

/// Loads operand nn in the PC if condition cc and the flag status match.
/// ```rust
/// //Example: When Z = 1 and C = 0, 
/// //JP NZ, 0x8000; Moves to next instruction after 3 cycles
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Mmu::new();
/// # memory.set_bios_enabled(false);
/// # cpu.set_f(0x80);
/// # memory.write_byte(0x0, 0xc2);
/// # memory.write_byte(0x1, 0x00);
/// # memory.write_byte(0x2, 0x80);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_pc(), 0x0003);
/// 
/// //JP Z, 0x8000; Jump to 0x8000
/// # cpu.set_pc(0x0);
/// # memory.write_byte(0x0, 0xca);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_pc(), 0x8000);
/// 
/// //JP C, 0x8000; Moves to next instruction after 3 cycles
/// # cpu.set_pc(0x0);
/// # memory.write_byte(0x0, 0xda);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_pc(), 0x0003);
/// 
/// //JP NC, 0x8000; Jump to 0x8000
/// # cpu.set_pc(0x0);
/// # memory.write_byte(0x0, 0xd2);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_pc(), 0x8000);
/// ```
pub fn jp_cc_nn(condition: Flag, cpu: &mut Cpu, memory: &mut Mmu) -> u8 {
    if cpu.get_flag(condition) {
        jp_nn(cpu, memory)
    } else {
        cpu.pc += 2;
        12
    }
}

/// If condition cc matches the flag, the PC value corresponding to the instruction following the CALL 
/// instruction in memory is pushed to the 2 bytes following the memory byte specified by the SP.
/// ```rust
/// //Examples: WhenZ = 1, PC = 0x7ffc
/// //CALL NZ, 0x1234; Moves to next instruction after 3 cycles
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Mmu::new();
/// # memory.set_bios_enabled(false);
/// # cpu.set_pc(0x7ffc);
/// # cpu.set_f(0x80);
/// # memory.write_byte(0x7ffc, 0xc4);
/// # memory.write_byte(0x7ffd, 0x34);
/// # memory.write_byte(0x7ffe, 0x12);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_pc(), 0x7fff);
/// 
/// //When PC = 0x8000
/// //CALL Z, 0x1234; Pushes 0x8003 to the stack and jumps to 0x1234
/// # cpu.set_pc(0x8000);
/// # memory.write_byte(0x8000, 0xcc);
/// # memory.write_byte(0x8001, 0x34);
/// # memory.write_byte(0x8002, 0x12);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_pc(), 0x1234);
/// assert_eq!(memory.read_byte(cpu.get_sp()), 0x03);
/// assert_eq!(memory.read_byte(cpu.get_sp() + 1), 0x80);
/// ``` 
pub fn call_cc_nn(condition: Flag, cpu: &mut Cpu, memory: &mut Mmu) -> u8 {
    if cpu.get_flag(condition) {
        call_nn(cpu, memory)
    } else {
        cpu.pc += 2;
        12
    }
}

pub fn reti(cpu: &mut Cpu, memory: &mut Mmu) -> u8 {
    ret(cpu, memory);
    cpu.ime = true;
    16
}

/// Pushes the current value of the PC to the memory stack and loads to the PC the page 0 memory addresses provided by operand t.
/// ```rust
/// //Example: when PC = 0x8000, RST 0x0008 ; Pushes 0x8001 to the stack and jumps to 0x0008
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Mmu::new();
/// # memory.set_bios_enabled(false);
/// # cpu.set_pc(0x8000);
/// # memory.write_byte(0x8000, 0xcf);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_pc(), 0x0008);
/// assert_eq!(memory.read_byte(cpu.get_sp()), 0x01);
/// assert_eq!(memory.read_byte(cpu.get_sp() + 1), 0x80);
/// ```
pub fn rst(addr: u16, cpu: &mut Cpu, memory: &mut Mmu) -> u8 {
    push_rr(PC, cpu, memory);
    cpu.pc = addr;

    16
}

/// Loads the contents of register pair HL in program counter PC.
/// ```rust
/// //Example: When HL = 0x8000,
/// //JP (HL) ; Jumps to 0x8000.
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Mmu::new();
/// # memory.set_bios_enabled(false);
/// # cpu.set_h(0x80);
/// # cpu.set_l(0x00);
/// # memory.write_byte(0x0, 0xe9);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_pc(), 0x8000);
/// ```
pub fn jp_hl(cpu: &mut Cpu) -> u8 {
    cpu.pc = cpu.get_rr(HL);

    4
}