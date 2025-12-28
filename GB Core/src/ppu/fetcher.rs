use crate::{log, mmu::MMU};

const SCX_ADRESS: u16 = 0xFF43;
const SCY_ADRESS: u16 = 0xFF42;
const WX_ADRESS: u16 = 0xFF4B;
const WY_ADRESS: u16 = 0xFF4A;

pub struct Fetcher {
    dots: u8,
    pub fetcher_x: u8,
    tile_index: u8,
    data_high: u8,
    data_low: u8,
}

impl Default for Fetcher {
    fn default() -> Self {
        Self {
            dots: 0,
            fetcher_x: 0,
            tile_index: 0,
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
        return mmu.get(0xFF4A) < 166 && mmu.get(0xFF4B) < 143;
    }

    fn get_tile(&mut self, mmu: &MMU) {
        // log!("{}: | x: {}, y: {}", self.dots, self.tile_x, self.tile_y);

        let tile_map = if mmu.bg_tile_map_area() && !self.tile_in_window(mmu) {
            0x9C00
        } else if mmu.window_tile_map() && self.tile_in_window(mmu) {
            0x9C00
        } else {
            0x9800
        };
        let address = tile_map
            + (mmu.get(SCY_ADRESS).wrapping_add(mmu.ly()) as u16 / 8) * 32
            + self.fetcher_x as u16;
        self.tile_index = mmu.get(address);
        // if self.tile_in_window(mmu) {
        //self.tile_x = self.fetcher_x.wrapping_add(mmu.get(WX_ADRESS));
        //self.tile_y = mmu.ly().wrapping_add(mmu.get(WY_ADRESS));
        // } else {
        //     self.tile_x = ((mmu.get(SCX_ADRESS) / 8) + self.fetcher_x) & 0x1F;
        //     self.tile_y = ((mmu.ly() as u16 + mmu.get(SCY_ADRESS) as u16) & 0xFF) as u8;
        // }
    }

    fn get_tile_data_low(&mut self, mmu: &MMU) {
        let block_address = if mmu.bg_window_tile() { 0x8000 } else { 0x8800 };
        // let tile_coordinate = self.tile_x as u16 + self.tile_y as u16 * 16;
        // let addr = block_address + tile_coordinate as u16;
        let offset = block_address + self.tile_index as u16 * 16;
        let address = offset + (mmu.get(SCY_ADRESS).wrapping_add(mmu.ly()) as u16 % 8) * 2;
        self.data_low = mmu.get(address);
    }

    fn get_tile_data_high(&mut self, mmu: &MMU) {
        let block_address = if mmu.bg_window_tile() { 0x8000 } else { 0x8800 };
        let offset = block_address + self.tile_index as u16 * 16;
        let address = offset + (mmu.get(SCY_ADRESS).wrapping_add(mmu.ly()) as u16 % 8) * 2;
        self.data_high = mmu.get(address + 1);
    }
}
