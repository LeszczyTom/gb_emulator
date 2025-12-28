#[derive(Default)]
pub struct Register {
    pub(crate) hi: u8,
    pub(crate) lo: u8,
}

impl Register {
    pub fn get(&self) -> u16 {
        ((self.hi as u16) << 8) | (self.lo as u16)
    }

    pub fn get_low(&self) -> u8 {
        self.lo
    }

    pub fn get_high(&self) -> u8 {
        self.hi
    }

    pub fn set(&mut self, value: u16) {
        let high = (value >> 8) as u8;
        let low = value as u8;

        self.set_high(high);
        self.set_low(low);
    }

    pub fn set_high(&mut self, value: u8) {
        self.hi = value;
    }

    pub fn set_low(&mut self, value: u8) {
        self.lo = value;
    }
}
