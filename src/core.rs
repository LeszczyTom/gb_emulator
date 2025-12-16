use crate::cpu::cpu::{CPU, Flag};
use crate::instructions::instruction::{
    CB_INSTRUCTIONS, CB_INSTRUCTIONS_PARAMETERS, CB_INSTRUCTIONS_TIMING, Cond, INSTRUCTIONS,
    INSTRUCTIONS_PARAMETERS, INSTRUCTIONS_TIMING, R8, R16, R16mem, R16stk,
};
use crate::mmu::MMU;
use crate::timer::Timer;

#[cfg(test)]
const PASSED_BLARGG: [u8; 6] = [80, 97, 115, 115, 101, 100];
#[cfg(test)]
const FAILED_BLARGG: [u8; 6] = [70, 97, 105, 108, 101, 100];

#[derive(Default)]
pub struct Core {
    pub(crate) cpu: CPU,
    pub(crate) mmu: MMU,
    pub(crate) timer: Timer,
}

impl Core {
    pub fn run(&mut self) -> bool {
        loop {
            if self.cpu.halted {
                self.cpu.halted = self.mmu.get(0xFFFF) & self.mmu.get(0xFF0F) == 0;
            }

            self.cpu.set_ime();

            if let Some(cycles) = self.handle_interrupts() {
                self.timer.cycle(&mut self.mmu, cycles);
            }

            if !self.cpu.halted {
                let opcode = self.fetch_opcode();
                let cycles = self.execute_instruction(opcode);
                self.timer.cycle(&mut self.mmu, cycles);
            } else {
                self.timer.cycle(&mut self.mmu, 4);
            }

            #[cfg(test)]
            if let Some(value) = self.test_ended() {
                return value;
            }
        }
    }

    #[cfg(test)]
    fn test_ended(&self) -> Option<bool> {
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

    fn execute_instruction(&mut self, opcode: usize) -> u8 {
        if self.cpu.cb {
            self.cpu.cb = false;
            CB_INSTRUCTIONS[opcode](self, &CB_INSTRUCTIONS_PARAMETERS[opcode]);
            return CB_INSTRUCTIONS_TIMING[opcode];
        } else {
            INSTRUCTIONS[opcode](self, &INSTRUCTIONS_PARAMETERS[opcode]);

            let mut cycles = INSTRUCTIONS_TIMING[opcode];

            if self.cpu.condition_met {
                self.cpu.condition_met = false;

                match opcode {
                    0x20 | 0x28 | 0x30 | 0x38 | 0xC2 | 0xCA | 0xD2 | 0xDA => cycles += 1,
                    0xC0 | 0xC8 | 0xD0 | 0xD8 | 0xC4 | 0xCC | 0xD4 | 0xDC => cycles += 3,
                    _ => (),
                }
            }

            return cycles;
        }
    }

    fn fetch_opcode(&mut self) -> usize {
        return self.fetch_imm8() as usize;
    }

    pub fn fetch_imm8(&mut self) -> u8 {
        let pc = self.cpu.pc.get();
        let opcode = self.mmu.get(pc);
        self.cpu.increment_pc(1);

        opcode
    }

    pub fn fetch_imm16(&mut self) -> u16 {
        let lower = self.fetch_imm8() as u16;
        let upper = (self.fetch_imm8() as u16) << 8;

        return upper | lower;
    }

    pub fn get_r8(&self, r8: &R8) -> u8 {
        match r8 {
            R8::B => self.cpu.bc.get_high(),
            R8::C => self.cpu.bc.get_low(),
            R8::D => self.cpu.de.get_high(),
            R8::E => self.cpu.de.get_low(),
            R8::H => self.cpu.hl.get_high(),
            R8::L => self.cpu.hl.get_low(),
            R8::HL => self.mmu.get(self.cpu.hl.get()),
            R8::A => self.cpu.af.get_high(),
        }
    }

    pub fn set_r8(&mut self, r8: &R8, value: u8) {
        match r8 {
            R8::B => self.cpu.bc.set_high(value),
            R8::C => self.cpu.bc.set_low(value),
            R8::D => self.cpu.de.set_high(value),
            R8::E => self.cpu.de.set_low(value),
            R8::H => self.cpu.hl.set_high(value),
            R8::L => self.cpu.hl.set_low(value),
            R8::HL => self.mmu.set(value, self.cpu.hl.get()),
            R8::A => self.cpu.af.set_high(value),
        };
    }

    pub fn get_r16mem(&mut self, r16mem: &R16mem) -> u8 {
        match r16mem {
            R16mem::BC => self.mmu.get(self.cpu.bc.get()),
            R16mem::DE => self.mmu.get(self.cpu.de.get()),
            R16mem::HLi => {
                let hl = self.cpu.hl.get();
                self.cpu.hl.set(hl.wrapping_add(1));
                self.mmu.get(hl)
            }
            R16mem::HLd => {
                let hl = self.cpu.hl.get();
                self.cpu.hl.set(hl.wrapping_sub(1));
                self.mmu.get(hl)
            }
        }
    }

    pub fn set_r16mem(&mut self, r16mem: &R16mem, value: u8) {
        match r16mem {
            R16mem::BC => self.mmu.set(value, self.cpu.bc.get()),
            R16mem::DE => self.mmu.set(value, self.cpu.de.get()),
            R16mem::HLi => {
                let hl = self.cpu.hl.get();
                self.cpu.hl.set(hl.wrapping_add(1));
                self.mmu.set(value, hl)
            }
            R16mem::HLd => {
                let hl = self.cpu.hl.get();
                self.cpu.hl.set(hl.wrapping_sub(1));
                self.mmu.set(value, hl)
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
        self.mmu.set(values[0], self.cpu.sp.get());
        self.cpu.sp.set(self.cpu.sp.get().wrapping_sub(1));
        self.mmu.set(values[1], self.cpu.sp.get());
    }

    pub fn pop(&mut self) -> u16 {
        let low = self.mmu.get(self.cpu.sp.get());
        self.cpu.sp.set(self.cpu.sp.get().wrapping_add(1));
        let high = self.mmu.get(self.cpu.sp.get());
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

    pub fn handle_interrupts(&mut self) -> Option<u8> {
        if !self.cpu.ime {
            return None;
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

            self.mmu.set(interrupt_flag & !(1 << flag), 0xFF0F);
            self.push(self.cpu.pc.get());

            self.cpu.pc.set(match flag {
                0 => 0x40,
                1 => 0x48,
                2 => 0x50,
                3 => 0x58,
                4 => 0x60,
                _ => panic!("Invalid interrupt flag"),
            });

            return Some(20);
        }

        return None;
    }
}
