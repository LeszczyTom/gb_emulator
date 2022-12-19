use crate::cpu::cpu::{
    Cpu,
    Register,
    RegisterPair::{ HL, DE, BC }
};

use crate::memory::mmu::Mmu;

/// Stores the contents of register A in the memory specified by register pair HL and simultaneously decrements the contents of HL.
/// ``` rust
/// //Example: HL = 0x4000 and A = 0x05,
/// //LD (HLD), A ; (0x4000) <- 0x05, HL = 0x3FFF
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Mmu::new();
/// # memory.set_bios_enabled(false);
/// # memory.write_byte(0x00, 0x32);
/// # cpu.set_a(0x05);
/// # cpu.set_h(0x40);
/// # cpu.set_l(0x00);
/// cpu.cycle(&mut memory);
/// assert_eq!(memory.read_byte(0x4000), 0x05);
/// assert_eq!(cpu.get_hl(), 0x3fff);
/// ```
pub fn ld_hld_a(cpu: &mut Cpu, memory: &mut Mmu) -> u8 {
    let hl = cpu.get_rr(HL);
    memory.write_byte(hl, cpu.a);
    cpu.set_rr(HL, hl.wrapping_sub(1));

    8
}

/// Loads 8-bit immediate data n into register r.
/// ``` rust
/// //Example: LD B, 0x24 ; B <- 0x24
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Mmu::new();
/// # memory.set_bios_enabled(false);
/// # memory.write_byte(0x00, 0x06);
/// # memory.write_byte(0x01, 0x24);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_b(), 0x24);
/// ```
pub fn ld_r_n(r: Register, cpu: &mut Cpu, memory: &mut Mmu) -> u8 {
    let n = cpu.read_n(memory);
    cpu.set_r(r, n);
    8
}

/// Loads the contents of register A in the internal RAM, port register, or mode register at the address in the range FFOOh-FFFFh specified by register C.
/// ``` rust
/// //Example: When C = 0x9F, A = 0x24
/// //LD (C), A ; (0xFF9F) <- A
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Mmu::new();
/// # memory.set_bios_enabled(false);
/// # memory.write_byte(0x00, 0xe2);
/// # cpu.set_c(0x9f);
/// # cpu.set_a(0x24);
/// cpu.cycle(&mut memory);
/// assert_eq!(memory.read_byte(0xff9f), 0x24);
/// ```
pub fn ld_c_a(cpu: &mut Cpu, memory: &mut Mmu) -> u8 {
    memory.write_byte(0xff00 + cpu.c as u16, cpu.a);
    8
}

/// Stores the contents of register A in the memory specified by register pair DE.
/// ``` rust
/// // Example: When DE = 0x205 and A = 0x00,
/// // LD (DE) , A ; (0x205) <- 0xOO
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Mmu::new();
/// # memory.set_bios_enabled(false);
/// # memory.write_byte(0x00, 0x12);
/// # cpu.set_d(0x02);
/// # cpu.set_e(0x05);
/// # cpu.set_a(0x00);
/// cpu.cycle(&mut memory);
/// assert_eq!(memory.read_byte(0x205), 0x00);
/// ```
pub fn ld_de_a(cpu: &mut Cpu, memory: &mut Mmu) -> u8 {
    memory.write_byte(cpu.get_rr(DE), cpu.a);
    8
}

/// Stores the contents of register A in the memory specified by register pair BC.
/// ``` rust
/// //Example: When BC = 0x205F and A = 0x3F,
/// //LD (BC) , A ; (0x205F) <- 0x3F
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Mmu::new();
/// # memory.set_bios_enabled(false);
/// # memory.write_byte(0x00, 0x02);
/// # cpu.set_b(0x20);
/// # cpu.set_c(0x5f);
/// # cpu.set_a(0x3f);
/// cpu.cycle(&mut memory);
/// assert_eq!(memory.read_byte(0x205f), 0x3f);
/// ```
pub fn ld_bc_a(cpu: &mut Cpu, memory: &mut Mmu) -> u8 {
    memory.write_byte(cpu.get_rr(BC), cpu.a);
    8
}

/// Loads into 0xffnn the contents of the register A.
/// ```rust
/// //Example: When n = 0x12, A = 0x34
/// //LDH (d8), A ; (0xFF12) <- A
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Mmu::new();
/// # memory.set_bios_enabled(false);
/// # memory.write_byte(0x00, 0xe0);
/// # memory.write_byte(0x01, 0x12);
/// # cpu.set_a(0x34);
/// cpu.cycle(&mut memory);
/// assert_eq!(memory.read_byte(0xff12), 0x34);
/// ```
pub fn ldh_n_a(cpu: &mut Cpu, memory: &mut Mmu) -> u8 {
    let addr = u16::from_be_bytes([0xff, cpu.read_n(memory)]);
    memory.write_byte(addr, cpu.a);

    12
}

/// Loads the contents specified by the contents of register pair DE into register A.
/// ```rust	
/// //Example: When (DE) = 0x5F,
/// //LD A, (DE) ; A <- 0x5F
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Mmu::new();
/// # memory.set_bios_enabled(false);
/// # memory.write_byte(0x00, 0x1a);
/// # cpu.set_d(0x01);
/// # cpu.set_e(0x00);
/// # memory.write_byte(0x100, 0x5f);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_a(), 0x5f);
/// ```
pub fn ld_a_de(cpu: &mut Cpu, memory: &mut Mmu) -> u8 {
    cpu.a = memory.read_byte(cpu.get_rr(DE));
    8
}

/// Loads the contents specified by the contents of register pair BC into register A.
/// ``` rust
/// //Example: When (BC) = 0x2F,
/// //LD A, (BC) ; A <- 0x2F
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Mmu::new();
/// # memory.set_bios_enabled(false);
/// # memory.write_byte(0x00, 0x0a);
/// # cpu.set_b(0x10);
/// # cpu.set_c(0x00);
/// # memory.write_byte(0x1000, 0x2f);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_a(), 0x2f);
/// ```
pub fn ld_a_bc(cpu: &mut Cpu, memory: &mut Mmu) -> u8 {
    cpu.a = memory.read_byte(cpu.get_rr(BC));
    8
}

/// Loads the contents of register r2 into register r1.
/// ```rust
/// //Examples:
/// //LD A, B ; A <- B
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Mmu::new();
/// # memory.set_bios_enabled(false);
/// # memory.write_byte(0x00, 0x78);
/// # cpu.set_b(0x5f);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_a(), 0x5f);
///
/// // LD B, D ; B <- D
/// memory.write_byte(0x01, 0x42);
/// cpu.set_d(0x12);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_b(), 0x12);
/// ```
pub fn ld_r_r(r1: Register, r2: Register, cpu: &mut Cpu) -> u8 {
    cpu.set_r(r1, cpu.get_r(r2));
    4
}

/// Loads the contents of memory (8 bits) specified by register pair HL into register r.
/// ```rust
/// //Example: When (HL) = 0x5C,
/// //LD H, (HL) ; H <- 0x5C
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Mmu::new();
/// # memory.set_bios_enabled(false);
/// # memory.write_byte(0x00, 0x66);
/// # cpu.set_h(0x10);
/// # cpu.set_l(0x00);
/// # memory.write_byte(0x1000, 0x5c);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_h(), 0x5c);
/// ```
pub fn ld_r_hl(r: Register, cpu: &mut Cpu, memory: &mut Mmu) -> u8 {
    cpu.set_r(r, memory.read_byte(cpu.get_rr(HL)));
    8
}

/// Stores the contents of register A in the memory specified by register pair HL and simultaneously increments the contents of HL.
/// ```rust
/// //Example: When HL = 0xFFFF and A = 0x56,
/// //LD (HLI), A ; (0xFFFF) <- 0x56, HL = 0x0000
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Mmu::new();
/// # memory.set_bios_enabled(false);
/// # memory.write_byte(0x00, 0x22);
/// # cpu.set_h(0xff);
/// # cpu.set_l(0xff);
/// # cpu.set_a(0x56);
/// cpu.cycle(&mut memory);
/// assert_eq!(memory.read_byte(0xffff), 0x56);
/// assert_eq!(cpu.get_hl(), 0x0000);
/// ```
pub fn ld_hli_a(cpu: &mut Cpu, memory: &mut Mmu) -> u8 {
    let hl = cpu.get_rr(HL);
    memory.write_byte(hl, cpu.a);
    cpu.set_rr(HL, hl.wrapping_add(1));

    8
}

/// Loads the contents of register A to the internal RAM or register specified by 16-bit immediate operand nn.
/// ```rust
/// //Example: LD (0xFF44), A ; (LY) <- A
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Mmu::new();
/// # memory.set_bios_enabled(false);
/// # memory.write_byte(0x00, 0xea);
/// # memory.write_byte(0x01, 0x44);
/// # memory.write_byte(0x02, 0xff);
/// # cpu.set_a(0x56);
/// cpu.cycle(&mut memory);
/// assert_eq!(memory.read_byte(0xff44), 0x56);
/// // LD (0x8000), A ; (0x8000) <- A
/// # memory.write_byte(0x03, 0xea);
/// # memory.write_byte(0x04, 0x00);    
/// # memory.write_byte(0x05, 0x80);
/// cpu.cycle(&mut memory);
/// assert_eq!(memory.read_byte(0x8000), 0x56);
/// ```
pub fn ld_nn_a(cpu: &mut Cpu, memory: &mut Mmu) -> u8 {
    let addr = cpu.read_nn(memory);
    memory.write_byte(addr, cpu.a);

    16
}

pub fn ldh_a_n(cpu: &mut Cpu, memory: &mut Mmu) -> u8 {
    let addr = 0xff00 + u16::from(cpu.read_n(memory));
    cpu.a = memory.read_byte(addr);

    12
}

/// Loads 8-bit immediate data n into memory specified by register pair HL.
/// ```rust
/// //Example: When HL = 0x8AC5,
/// //LD (HL), 0 ; 0x8AC5 <- 0
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Mmu::new();
/// # memory.set_bios_enabled(false);
/// # memory.write_byte(0x00, 0x36);
/// # memory.write_byte(0x01, 0x00);
/// # cpu.set_h(0x8a);
/// # cpu.set_l(0xc5);
/// cpu.cycle(&mut memory);
/// assert_eq!(memory.read_byte(0x8ac5), 0x00);
/// ```
pub fn ld_hl_n(cpu: &mut Cpu, memory: &mut Mmu) -> u8 {
    let addr = cpu.get_rr(HL);
    let value = cpu.read_n(memory);
    memory.write_byte(addr, value);

    12
}

/// Loads in register A the contents of memory specified by the contents of register pair HL and simultaneously decrements the contents of HL.
/// ```rust
/// //Example: When HL = 0x8A5C and (0x8A5C) = 0x3C,
/// //LD A, (HLD) ; A <- 0x3C, HL <- 0x8A5B
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Mmu::new();
/// # memory.set_bios_enabled(false);
/// # memory.write_byte(0x00, 0x3a);
/// # cpu.set_h(0x8a);
/// # cpu.set_l(0x5c);
/// # memory.write_byte(0x8a5c, 0x3c);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_a(), 0x3c);
/// assert_eq!(cpu.get_hl(), 0x8a5b);
/// ```
pub fn ld_a_hld(cpu: &mut Cpu, memory: &mut Mmu) -> u8 {
    let hl = cpu.get_rr(HL);
    cpu.a = memory.read_byte(hl);
    cpu.set_rr(HL, hl.wrapping_sub(1));

    8
}

/// Stores the contents of register r in memory specified by register pair HL.
/// ```rust
/// //Example: When A = 0x3C, HL = 0x8AC5 
/// //LD (HL), A ; (0x8AC5h) <- 0x3C
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Mmu::new();
/// # memory.set_bios_enabled(false);
/// # memory.write_byte(0x00, 0x77);
/// # cpu.set_a(0x3c);
/// # cpu.set_h(0x8a);
/// # cpu.set_l(0xc5);
/// cpu.cycle(&mut memory);
/// assert_eq!(memory.read_byte(0x8ac5), 0x3c);
/// ```
pub fn ld_hl_r(r: Register, cpu: &mut Cpu, memory: &mut Mmu) -> u8 {
    let addr = cpu.get_rr(HL);
    let value = cpu.get_r(r);
    memory.write_byte(addr, value);

    8
}

/// Loads into register A the contents of the internal RAM, port register, or mode register at the address in the range FFOOh-FFFFh specified by register C.
/// ``` rust
/// //Example: When C = 0x95,
/// //LD A, (C) ; A <- contents of (0xFF95)
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Mmu::new();
/// # memory.set_bios_enabled(false);
/// # memory.write_byte(0x00, 0xf2);
/// # cpu.set_c(0x95);
/// # memory.write_byte(0xff95, 0x3c);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_a(), 0x3c);
/// ```
pub fn ld_a_c(cpu: &mut Cpu, memory: &mut Mmu) -> u8 {
    let addr = 0xff00 + u16::from(cpu.c);
    cpu.a = memory.read_byte(addr);

    8
}

/// Loads into register A the contents of the internal RAM or register specified by 16-bit immediate operand nn.
/// ```rust
/// //Example: LD A, (0xFF44) ; (0xFF44) = 0x1a
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Mmu::new();
/// # memory.set_bios_enabled(false);
/// # memory.write_byte(0x00, 0xfa);
/// # memory.write_byte(0x01, 0x44);
/// # memory.write_byte(0x02, 0xff);
/// # memory.write_byte(0xff44, 0x1a);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_a(), 0x1a);
/// ```
pub fn ld_a_nn(cpu: &mut Cpu, memory: &mut Mmu) -> u8 {
    let addr = cpu.read_nn(memory);
    cpu.a = memory.read_byte(addr);

    16
}