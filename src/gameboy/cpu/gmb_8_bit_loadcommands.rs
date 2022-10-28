use crate::gameboy::{ cpu::Cpu, memory::Memory };
use crate::gameboy::cpu::Register;
use crate::gameboy::cpu::RegisterPair::{ HL, DE };

/// Stores the contents of register A in the memory specified by register pair HL and simultaneously decrements the contents of HL.
/// ``` rust
/// //Example: HL = 0x4000 and A = 0x05,
/// //LD (HLD), A ; (0x4000) <- 0x05, HL = 0x3FFF
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Memory::new();
/// # memory.write_byte(0x00, 0x32);
/// # cpu.set_a(0x05);
/// # cpu.set_h(0x40);
/// # cpu.set_l(0x00);
/// cpu.cycle(&mut memory);
/// assert_eq!(memory.read_byte(0x4000), 0x05);
/// assert_eq!(cpu.get_hl(), 0x3fff);
/// ```
pub fn ld_hld_a(cpu: &mut Cpu, memory: &mut Memory) -> u8 {
    let hl = cpu.get_rr(HL);
    memory.write_byte(hl, cpu.a);
    cpu.set_rr(HL, hl.wrapping_sub(1));

    8
}

/// Loads 8-bit immediate data n into register r.
/// ``` rust
/// //Example: LD B, 0x24 ; B <- 0x24
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Memory::new();
/// # memory.write_byte(0x00, 0x06);
/// # memory.write_byte(0x01, 0x24);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_b(), 0x24);
/// ```
pub fn ld_r_n(r: Register, cpu: &mut Cpu, memory: &mut Memory) -> u8 {
    let n = cpu.read_n(memory);
    cpu.set_r(r, n);
    8
}

/// Loads the contents of register A in the internal RAM, port register, or mode register at the address in the range FFOOh-FFFFh specified by register C.
/// ``` rust
/// //Example: When C = 0x9F, A = 0x24
/// //LD (C), A ; (0xFF9F) <- A
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Memory::new();
/// # memory.write_byte(0x00, 0xe2);
/// # cpu.set_c(0x9f);
/// # cpu.set_a(0x24);
/// cpu.cycle(&mut memory);
/// assert_eq!(memory.read_byte(0xff9f), 0x24);
/// ```
pub fn ld_c_a(cpu: &mut Cpu, memory: &mut Memory) -> u8 {
    memory.write_byte(0xff00 + cpu.c as u16, cpu.a);
    8
}

/// Stores the contents of register A in the memory specified by register pair DE.
/// ``` rust
/// // Example: When DE = 0x205 and A = 0x00,
/// // LD (DE) , A ; (0x205) <- 0xOO
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Memory::new();
/// # memory.write_byte(0x00, 0x12);
/// # cpu.set_d(0x02);
/// # cpu.set_e(0x05);
/// # cpu.set_a(0x00);
/// cpu.cycle(&mut memory);
/// assert_eq!(memory.read_byte(0x205), 0x00);
/// ```
pub fn _ld_de_a(cpu: &mut Cpu, memory: &mut Memory) -> u8 {
    memory.write_byte(cpu.get_rr(DE), cpu.a);
    8
}

/// Loads into 0xffnn the contents of the register A.
/// ```rust
/// //Example: When n = 0x12, A = 0x34
/// //LDH (d8), A ; (0xFF12) <- A
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Memory::new();
/// # memory.write_byte(0x00, 0xe0);
/// # memory.write_byte(0x01, 0x12);
/// # cpu.set_a(0x34);
/// cpu.cycle(&mut memory);
/// assert_eq!(memory.read_byte(0xff12), 0x34);
/// ```
pub fn ldh_n_a(cpu: &mut Cpu, memory: &mut Memory) -> u8 {
    let addr = u16::from_be_bytes([0xff, cpu.read_n(memory)]);
    memory.write_byte(addr, cpu.a);

    12
}

/// Loads the contents specified by the contents of register pair DE into register A.
/// ```rust	
/// //Example: When (DE) = 0x5F,
/// //LD A, (DE) ; A <- 0x5F
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Memory::new();
/// # memory.write_byte(0x00, 0x1a);
/// # cpu.set_d(0x01);
/// # cpu.set_e(0x00);
/// # memory.write_byte(0x100, 0x5f);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_a(), 0x5f);
/// ```
pub fn ld_a_de(cpu: &mut Cpu, memory: &mut Memory) -> u8 {
    cpu.a = memory.read_byte(cpu.get_rr(DE));
    8
}

/// Loads the contents of register r2 into register r1.
/// ```rust
/// //Examples:
/// //LD A, B ; A <- B
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Memory::new();
/// # memory.write_byte(0x00, 0x78);
/// # cpu.set_b(0x5f);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_a(), 0x5f);
///
/// // LD B, D ; B <- D
/// memory.write_byte(0x01, 0x58);
/// cpu.set_d(0x12);
/// cpu.cycle(&mut memory);
/// assert_eq!(cpu.get_b(), 0x12);
/// ```
pub fn ld_r_r(r1: Register, r2: Register, cpu: &mut Cpu) -> u8 {
    cpu.set_r(r1, cpu.get_r(r2));
    4
}

/// Stores the contents of register A in the memory specified by register pair HL and simultaneously increments the contents of HL.
/// ```rust
/// //Example: When HL = 0xFFFF and A = 0x56,
/// //LD (HLI), A ; (0xFFFF) <- 0x56, HL = 0x0000
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Memory::new();
/// # memory.write_byte(0x00, 0x22);
/// # cpu.set_h(0xff);
/// # cpu.set_l(0xff);
/// # cpu.set_a(0x56);
/// cpu.cycle(&mut memory);
/// assert_eq!(memory.read_byte(0xffff), 0x56);
/// assert_eq!(cpu.get_hl(), 0x0000);
/// ```
pub fn ld_hli_a(cpu: &mut Cpu, memory: &mut Memory) -> u8 {
    let hl = cpu.get_rr(HL);
    memory.write_byte(hl, cpu.a);
    cpu.set_rr(HL, hl.wrapping_add(1));

    8
}

/// Loads the contents of register A to the internal RAM or register specified by 16-bit immediate operand nn.
/// ```rust
/// //Example: LD (0xFF44), A ; (LY) <- A
/// # let mut cpu = gameboy::gameboy::cpu::Cpu::new();
/// # let mut memory = gameboy::gameboy::memory::Memory::new();
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
pub fn ld_nn_a(cpu: &mut Cpu, memory: &mut Memory) -> u8 {
    let addr = cpu.read_nn(memory);
    memory.write_byte(addr, cpu.a);

    16
}

pub fn ldh_a_n(cpu: &mut Cpu, memory: &mut Memory) -> u8 {
    let addr = 0xff00 + u16::from(cpu.read_n(memory));
    cpu.a = memory.read_byte(addr);

    12
}
