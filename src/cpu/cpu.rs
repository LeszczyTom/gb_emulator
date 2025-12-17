use crate::cpu::register::Register;

pub enum Flag {
    Z = 7,
    N = 6,
    H = 5,
    C = 4,
}

#[derive(Default)]
pub struct CPU {
    pub(crate) af: Register,
    pub(crate) bc: Register,
    pub(crate) de: Register,
    pub(crate) hl: Register,
    pub(crate) sp: Register,
    pub(crate) pc: Register,

    pub(crate) halted: bool,
    pub(crate) ime: bool,
    pub(crate) cb: bool,
    pub(crate) set_ime: Option<u8>,
}

impl CPU {
    pub fn increment_pc(&mut self, value: u16) {
        self.pc.set(self.pc.get().wrapping_add(value));
    }

    pub fn set_flag(&mut self, flag: Flag, set: bool) {
        if set {
            self.af.set_low(self.af.get_low() | (1 << flag as u8));
        } else {
            self.af.set_low(self.af.get_low() & !(1 << flag as u8));
        }
    }

    pub fn get_flag(&self, flag: Flag) -> bool {
        return ((self.af.get_low() >> flag as u8) & 1) == 1;
    }

    pub fn set_ime(&mut self, cycles: u8) {
        if let Some(value) = self.set_ime {
            for _ in 0..cycles {
                if value == 0 {
                    self.ime = true;
                    self.set_ime = None;
                    return;
                } else {
                    self.set_ime = Some(value - 1);
                }
            }
        }
    }
}
