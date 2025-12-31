use crate::cpu::cpu::{CPU, Flag};
use crate::instructions::instruction::{
    CB_INSTRUCTIONS, CB_INSTRUCTIONS_PARAMETERS, Cond, INSTRUCTIONS, INSTRUCTIONS_PARAMETERS, R8,
    R16, R16mem, R16stk,
};
use crate::log;
use crate::mmu::MMU;
use crate::ppu::ppu::PPU;
use crate::timer::Timer;

#[cfg(test)]
const PASSED_BLARGG: [u8; 6] = [80, 97, 115, 115, 101, 100];
#[cfg(test)]
const FAILED_BLARGG: [u8; 6] = [70, 97, 105, 108, 101, 100];

const WIDTH: usize = 160;
const HEIGHT: usize = 144;
const BUFFER_SIZE: usize = WIDTH * HEIGHT * 4;

const COLOR_1: [u8; 4] = [0xe0, 0xf0, 0xe7, 0xff];
const COLOR_2: [u8; 4] = [0x8b, 0xa3, 0x94, 0xff];
const COLOR_3: [u8; 4] = [0x55, 0x64, 0x5a, 0xff];
const COLOR_4: [u8; 4] = [0x34, 0x3d, 0x37, 0xff];

pub struct Core {
    pub cpu: CPU,
    pub mmu: MMU,
    pub(crate) timer: Timer,
    pub(crate) ppu: PPU,
    screen_index: usize,
    screen_buffer: [u8; BUFFER_SIZE],
}

impl Default for Core {
    fn default() -> Self {
        Self {
            cpu: CPU::default(),
            mmu: MMU::default(),
            timer: Timer::default(),
            ppu: PPU::default(),
            screen_index: 0,
            screen_buffer: [0; BUFFER_SIZE],
        }
    }
}

impl Core {
    pub fn run(&mut self) -> bool {
        loop {
            if self.cpu.halted {
                self.cpu.halted = self.mmu.get(0xFFFF) & self.mmu.get(0xFF0F) == 0;
            }

            if !self.cpu.halted {
                let opcode = self.fetch_opcode();
                self.execute_instruction(opcode);
            } else {
                self.tick_timer(1);
            }

            #[cfg(test)]
            if let Some(value) = self.test_ended() {
                return value;
            }
        }
    }

    pub fn load_bios(&mut self, data: &[u8], len: usize) -> bool {
        //TODO err
        self.mmu.bios[0..len].copy_from_slice(data);
        return true;
    }

    pub fn load_rom(&mut self, data: &[u8], len: usize) -> bool {
        //TODO err
        self.mmu.mem[0..len].copy_from_slice(data);
        return true;
    }

    pub fn get_screen_buffer(&self) -> [u8; BUFFER_SIZE] {
        return self.screen_buffer;
    }

    pub fn tick(&mut self) {
        if !self.cpu.halted {
            let opcode = self.fetch_opcode();
            self.execute_instruction(opcode);
        } else {
            self.tick_timer(1);
        }
    }

    pub fn tick_timer(&mut self, cycles: u8) {
        if self.cpu.halted {
            self.cpu.halted = self.mmu.get(0xFFFF) & self.mmu.get(0xFF0F) == 0;
        }

        for _ in 0..cycles {
            self.tick_ppu();
        }

        self.cpu.set_ime(cycles);
        self.timer.cycle(&mut self.mmu, cycles);
        self.handle_interrupts();
    }

    fn tick_ppu(&mut self) {
        for _ in 0..4 {
            if let Some(data) = self.ppu.cycle(&mut self.mmu) {
                let pixels = match data {
                    0 => COLOR_1,
                    1 => COLOR_2,
                    2 => COLOR_3,
                    3 => COLOR_4,
                    _ => unreachable!("pixel: {}", data),
                };

                for pixel in pixels {
                    self.screen_buffer[self.screen_index] = pixel;
                    self.screen_index += 1;

                    if self.screen_index == BUFFER_SIZE {
                        self.screen_index = 0;
                    }
                }
            }
        }
    }

    #[cfg(test)]
    fn test_ended(&self) -> Option<bool> {
        if self.mmu.mem[0xA001] == 0xDE
            && self.mmu.mem[0xA002] == 0xB0
            && self.mmu.mem[0xA003] == 0x61
        {
            let status = self.mmu.mem[0xA000];
            if status != 0x80 {
                println!("Status: {:02X}", status);

                let mut index = 0xA004;
                loop {
                    let value = self.mmu.mem[index];

                    if value == 0x00 {
                        break;
                    }

                    index += 1;
                    print!("{}", value as char);
                }

                if status == 0 {
                    return Some(true);
                } else {
                    return Some(false);
                }
            }
        }

        if self.mmu.serial_output == PASSED_BLARGG {
            return Some(true);
        }

        if self.mmu.serial_output == FAILED_BLARGG {
            return Some(false);
        }

        if self.cpu.bc.get() == 0x0305 && self.cpu.de.get() == 0x080D && self.cpu.hl.get() == 0x1522
        {
            return Some(true);
        }

        if self.cpu.bc.get() == 0x4242 && self.cpu.de.get() == 0x4242 && self.cpu.hl.get() == 0x4242
        {
            return Some(false);
        }

        return None;
    }

    fn execute_instruction(&mut self, opcode: usize) {
        if self.cpu.cb {
            self.cpu.cb = false;
            CB_INSTRUCTIONS[opcode](self, &CB_INSTRUCTIONS_PARAMETERS[opcode]);
        } else {
            INSTRUCTIONS[opcode](self, &INSTRUCTIONS_PARAMETERS[opcode]);
        }
    }

    fn fetch_opcode(&mut self) -> usize {
        let pc = self.cpu.pc.get();
        let opcode = self.mmu.get(pc);

        // Halt && CB
        if !self.cpu.cb && (opcode == 0x76 || opcode == 0xDB || opcode == 0x10) {
        } else {
            self.tick_timer(1);
        }

        self.cpu.increment_pc(1);

        return opcode as usize;
    }

    pub fn fetch_imm8(&mut self) -> u8 {
        let pc = self.cpu.pc.get();
        let imm8 = self.mem_get(pc);
        self.cpu.increment_pc(1);

        imm8
    }

    pub fn fetch_imm16(&mut self) -> u16 {
        let lower = self.fetch_imm8() as u16;
        let upper = (self.fetch_imm8() as u16) << 8;

        return upper | lower;
    }

    pub fn get_r8(&mut self, r8: &R8) -> u8 {
        match r8 {
            R8::B => self.cpu.bc.get_high(),
            R8::C => self.cpu.bc.get_low(),
            R8::D => self.cpu.de.get_high(),
            R8::E => self.cpu.de.get_low(),
            R8::H => self.cpu.hl.get_high(),
            R8::L => self.cpu.hl.get_low(),
            R8::HL => self.mem_get(self.cpu.hl.get()),
            R8::A => self.cpu.af.get_high(),
        }
    }

    pub fn mem_set(&mut self, value: u8, address: u16) {
        self.mmu.set(value, address);
        self.tick_timer(1);
    }

    pub fn mem_get(&mut self, address: u16) -> u8 {
        let value = self.mmu.get(address);
        self.tick_timer(1);

        return value;
    }

    pub fn set_r8(&mut self, r8: &R8, value: u8) {
        match r8 {
            R8::B => self.cpu.bc.set_high(value),
            R8::C => self.cpu.bc.set_low(value),
            R8::D => self.cpu.de.set_high(value),
            R8::E => self.cpu.de.set_low(value),
            R8::H => self.cpu.hl.set_high(value),
            R8::L => self.cpu.hl.set_low(value),
            R8::HL => self.mem_set(value, self.cpu.hl.get()),
            R8::A => self.cpu.af.set_high(value),
        };
    }

    pub fn get_r16mem(&mut self, r16mem: &R16mem) -> u8 {
        match r16mem {
            R16mem::BC => self.mem_get(self.cpu.bc.get()),
            R16mem::DE => self.mem_get(self.cpu.de.get()),
            R16mem::HLi => {
                let hl = self.cpu.hl.get();
                self.cpu.hl.set(hl.wrapping_add(1));
                self.mem_get(hl)
            }
            R16mem::HLd => {
                let hl = self.cpu.hl.get();
                self.cpu.hl.set(hl.wrapping_sub(1));
                self.mem_get(hl)
            }
        }
    }

    pub fn set_r16mem(&mut self, r16mem: &R16mem, value: u8) {
        match r16mem {
            R16mem::BC => self.mem_set(value, self.cpu.bc.get()),
            R16mem::DE => self.mem_set(value, self.cpu.de.get()),
            R16mem::HLi => {
                let hl = self.cpu.hl.get();
                self.cpu.hl.set(hl.wrapping_add(1));
                self.mem_set(value, hl)
            }
            R16mem::HLd => {
                let hl = self.cpu.hl.get();
                self.cpu.hl.set(hl.wrapping_sub(1));
                self.mem_set(value, hl)
            }
        };
    }

    pub fn is_condition_met(&self, cond: &Cond) -> bool {
        match cond {
            Cond::NZ => !self.cpu.get_flag(Flag::Z),
            Cond::Z => self.cpu.get_flag(Flag::Z),
            Cond::NC => !self.cpu.get_flag(Flag::C),
            Cond::C => self.cpu.get_flag(Flag::C),
        }
    }

    pub fn push(&mut self, value: u16) {
        let values = value.to_be_bytes();
        self.cpu.sp.set(self.cpu.sp.get().wrapping_sub(1));
        self.mem_set(values[0], self.cpu.sp.get());
        self.cpu.sp.set(self.cpu.sp.get().wrapping_sub(1));
        self.mem_set(values[1], self.cpu.sp.get());
    }

    pub fn pop(&mut self) -> u16 {
        let low = self.mem_get(self.cpu.sp.get());
        self.cpu.sp.set(self.cpu.sp.get().wrapping_add(1));
        let high = self.mem_get(self.cpu.sp.get());
        self.cpu.sp.set(self.cpu.sp.get().wrapping_add(1));

        return u16::from_be_bytes([high, low]);
    }

    pub fn get_r16stk(&self, r16stk: &R16stk) -> u16 {
        match r16stk {
            R16stk::BC => self.cpu.bc.get(),
            R16stk::DE => self.cpu.de.get(),
            R16stk::HL => self.cpu.hl.get(),
            R16stk::AF => self.cpu.af.get(),
        }
    }

    pub fn set_r16stk(&mut self, r16stk: &R16stk, value: u16) {
        match r16stk {
            R16stk::BC => self.cpu.bc.set(value),
            R16stk::DE => self.cpu.de.set(value),
            R16stk::HL => self.cpu.hl.set(value),
            R16stk::AF => self.cpu.af.set(value),
        }
    }

    pub fn get_r16(&self, r16: &R16) -> u16 {
        match r16 {
            R16::BC => return self.cpu.bc.get(),
            R16::DE => return self.cpu.de.get(),
            R16::HL => return self.cpu.hl.get(),
            R16::SP => return self.cpu.sp.get(),
        }
    }

    pub fn set_r16(&mut self, r16: &R16, value: u16) {
        match r16 {
            R16::BC => return self.cpu.bc.set(value),
            R16::DE => return self.cpu.de.set(value),
            R16::HL => return self.cpu.hl.set(value),
            R16::SP => return self.cpu.sp.set(value),
        }
    }

    pub fn handle_interrupts(&mut self) {
        if !self.cpu.ime {
            return;
        }

        let interrupt_flag = self.mmu.get(0xFF0F);
        let interrupt_enable = self.mmu.get(0xFFFF);

        for flag in 0..5 {
            if interrupt_flag & (1 << flag) == 0 {
                continue;
            }

            if interrupt_enable & (1 << flag) == 0 {
                continue;
            }

            self.cpu.ime = false;
            self.cpu.halted = false;

            self.mem_set(interrupt_flag & !(1 << flag), 0xFF0F);
            self.push(self.cpu.pc.get());

            self.cpu.pc.set(match flag {
                0 => 0x40,
                1 => 0x48,
                2 => 0x50,
                3 => 0x58,
                4 => 0x60,
                _ => panic!("Invalid interrupt flag"),
            });

            self.tick_timer(2);
        }
    }
}
