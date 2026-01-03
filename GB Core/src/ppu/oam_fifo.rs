use std::collections::VecDeque;

use crate::{
    log,
    mmu::MMU,
    ppu::{Palette, object::OamObject, pixel::Pixel},
};

pub struct OamFifo {
    dots: u8,
    data: VecDeque<Pixel>,
    pub object: Option<OamObject>,
    data_low: u8,
    data_high: u8,
}

impl Default for OamFifo {
    fn default() -> Self {
        Self {
            data: VecDeque::new(),
            object: None,
            data_low: 0,
            data_high: 0,
            dots: 0,
        }
    }
}

impl OamFifo {
    pub fn tick(&mut self, mmu: &MMU) {
        if self.object.is_some() {
            self.tick_fetcher(mmu);
            self.dots += 1;
        } else {
            self.dots = 0;
        }
    }

    pub fn get_pixel(&mut self) -> Option<Pixel> {
        self.data.pop_front()
    }

    pub fn reset(&mut self) {
        self.data.clear();
        self.object = None;
    }

    pub fn fetching(&self) -> bool {
        return self.dots != 0;
    }

    fn tick_fetcher(&mut self, mmu: &MMU) {
        match self.dots {
            1 => self.get_tile_data_low(mmu),
            3 => self.get_tile_data_high(mmu),
            5 => self.push(mmu),
            _ => (),
        }
    }

    fn get_tile_data_low(&mut self, mmu: &MMU) {
        if let Some(object) = self.object {
            let offset = 0x8000 + object.tile_index() as u16 * 16;
            let y = if object.flip_y() {
                7 - mmu.ly() as u16 % 8
            } else {
                mmu.ly() as u16 % 8
            };
            let address = offset + y * 2;
            self.data_low = mmu.mem[address as usize];
        }
    }

    fn get_tile_data_high(&mut self, mmu: &MMU) {
        if let Some(object) = self.object {
            let offset = 0x8000 + object.tile_index() as u16 * 16;
            let y = if object.flip_y() {
                7 - mmu.ly() as u16 % 8
            } else {
                mmu.ly() as u16 % 8
            };
            let address = offset + y * 2;
            self.data_high = mmu.mem[address as usize + 1];
        }
    }

    fn push(&mut self, mmu: &MMU) {
        if mmu.obj_enable()
            && let Some(obj) = self.object
        {
            for i in 0..8 {
                let shift = if obj.flip_x() { i } else { 7 - i };
                let high = (self.data_high >> shift) & 1;
                let low = (self.data_low >> shift) & 1;
                let data = (high << 1) | low;
                let priority = !obj.priority();
                let palette = if obj.palette() {
                    Palette::OBP1
                } else {
                    Palette::OBP0
                };

                self.data.push_back(Pixel::new(data, priority, palette));
            }
        }

        self.object = None;
        self.dots = 0;
    }
}
