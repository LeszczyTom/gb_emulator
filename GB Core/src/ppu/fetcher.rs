use crate::{log, mmu::MMU};

const SCX_ADRESS: u16 = 0xFF43;
const SCY_ADRESS: u16 = 0xFF42;
const WX_ADRESS: u16 = 0xFF4B;
const WY_ADRESS: u16 = 0xFF4A;

pub struct Fetcher {
    dots: u8,
    pub fetcher_x: u8,
    tile_index: u8,
    tile_y: u8,
    windows_y: u8,
    data_high: u8,
    data_low: u8,
}

impl Default for Fetcher {
    fn default() -> Self {
        Self {
            dots: 0,
            fetcher_x: 0,
            tile_index: 0,
            tile_y: 0,
            windows_y: 0,
            data_high: 0,
            data_low: 0,
        }
    }
}

impl Fetcher {
    pub fn tick(&mut self, mmu: &mut MMU, fifo: &mut Vec<u8>) {
        if self.dots == 0 {
            self.get_tile(mmu);
        } else if self.dots == 2 {
            self.get_tile_data_low(mmu);
        } else if self.dots == 4 {
            self.get_tile_data_high(mmu);
        } else if self.dots >= 6 && fifo.is_empty() {
            self.dots = 0;
            self.fetcher_x += 1;

            for i in 0..8 {
                let high = (self.data_high >> i) & 1;
                let low = (self.data_low >> i) & 1;
                fifo.push((high << 1) | low);
            }
            return;
        }

        self.dots += 1;
    }

    fn tile_in_window(&self, mmu: &MMU) -> bool {
        return mmu.window_enabled()
            && self.fetcher_x >= mmu.get(WX_ADRESS) - 7
            && mmu.ly() >= mmu.get(WY_ADRESS);
    }

    fn get_tile(&mut self, mmu: &MMU) {
        // let tile_map = if self.tile_in_window(mmu) {
        //     if mmu.window_tile_map() {
        //         0x9C00
        //     } else {
        //         0x9800
        //     }
        // } else {
        //     if mmu.bg_tile_map_area() {
        //         0x9800
        //     } else {
        //         0x9800
        //     }
        // };

        let tile_map = if mmu.bg_tile_map_area() && !self.tile_in_window(mmu) {
            0x9C00
        } else if mmu.window_tile_map() && self.tile_in_window(mmu) {
            0x9C00
        } else {
            0x9800
        }; // if self.tile_in_window(mmu) {
        //     self.tile_y = mmu.ly();
        // } else {
        self.tile_y = mmu.get(SCY_ADRESS).wrapping_add(mmu.ly());
        // };

        let x = self.fetcher_x as u16;
        let y = self.tile_y as u16 / 8;
        let address = tile_map + y * 32 + x;
        self.tile_index = mmu.get(address);
    }

    fn get_tile_data_low(&mut self, mmu: &MMU) {
        let block_address = if mmu.bg_window_tile() { 0x8000 } else { 0x8800 };
        let offset = block_address + self.tile_index as u16 * 16;
        let address = offset + (self.tile_y as u16 % 8) * 2;
        self.data_low = mmu.get(address);
    }

    fn get_tile_data_high(&mut self, mmu: &MMU) {
        let block_address = if mmu.bg_window_tile() { 0x8000 } else { 0x8800 };
        let offset = block_address + self.tile_index as u16 * 16;
        let address = offset + (self.tile_y as u16 % 8) * 2;
        self.data_high = mmu.get(address + 1);
    }
}
