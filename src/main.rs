pub mod rom;
pub mod memory;
pub mod cpu;

use memory::MEMORY;
use rom::ROM;
use cpu::LR35902;

struct GAMEBOY {
    cpu: LR35902,
    memory: MEMORY,
    rom: ROM,
}

impl GAMEBOY {
    pub fn new() -> GAMEBOY {
        GAMEBOY {
            cpu: LR35902::new(),
            memory: MEMORY::new(),
            rom: ROM::new(),
        }
    }

    fn load_rom(&mut self, rom_path: &str) {
        self.rom.load_rom(rom_path);
    }

    fn fetch_opcode(&mut self) -> u16 {
        self.rom.fetch_opcode(self.cpu.get_pc())
    }

    fn execute_opcode(&mut self, opcode: u16) -> u8 {
        match opcode {
            0x00 => return 4, // NOP
            0x01 => { // LD BC, d16
                let d16 = self.read_d16();
                self.cpu.set_bc(d16);
                return 12
            },
            0x02 => { // LD (BC), A
                self.memory.write_byte(self.cpu.get_bc(), self.cpu.get_a());
                return 8
            },
            0x03 => { // INC BC
                self.cpu.increase_register("bc");
                return 8
            },
            0x04 => { // INC B
                self.cpu.increase_register("b");
                return 4
            },
            0x05 => { // DEC B
                self.cpu.decrease_register("b");
                return 4
            },
            0x06 => { // LD B, d8
                let d8 = self.read_d8();
                self.cpu.set_b(d8);
                return 8
            },
            0x07 => { // RLCA
                self.cpu.rotate_a_through_carry(false);
                self.cpu.flag_zero(false);
                self.cpu.flag_substract(false);
                self.cpu.flag_half_carry(false);
                return 4
            },
            0x08 => { // LD (a16), SP
                let d16 = self.read_d16();
                let sp:[u8; 2] = self.cpu.get_sp().to_be_bytes();
                self.memory.write_byte(d16, sp[0]);
                self.memory.write_byte(d16 + 1, sp[1]);
                return 20
            },
            0x09 => { // ADD HL, BC
                return self.cpu.add_to_hl("bc")
            },
            0x0A => { // LD A, (BC)
                self.cpu.set_a(self.memory.read_byte(self.cpu.get_bc()));
                return 8
            },
            0x0B => { // DEC BC
                self.cpu.decrease_register("bc");
                return 8
            },
            0x0C => { // INC C
                self.cpu.increase_register("c");
                return 4
            },
            0x0D => { // DEC C
                self.cpu.decrease_register("c");
                return 4
            },
            0x0E => { // LD C, d8
                let d8 = self.read_d8();
                self.cpu.set_c(d8);
                return 8
            },
            0x0F => { // RRCA
                self.cpu.rotate_a_through_carry(true);
                self.cpu.flag_zero(false);
                self.cpu.flag_substract(false);
                self.cpu.flag_half_carry(false);
                return 4
            },
            _ => return 0,
        }
    }

    fn read_d16(&mut self) -> u16 {
        let mut bytes: [u8; 2] = [0,0];
        for i in 0..2 {
            bytes[i] = self.memory.read_byte(self.cpu.get_pc());
        }

        u16::from_be_bytes(bytes)
    }

    fn read_d8(&mut self) -> u8 {
        self.memory.read_byte(self.cpu.get_pc())
    }
}

fn main() {
    let rom_path = "./resources/rom.gbc";
    let mut gba = GAMEBOY::new();
    gba.load_rom(rom_path);

    let op = gba.fetch_opcode();
    println!("{:02x}, cycles: {}", op, gba.execute_opcode(op));

}

