use crate::{log, mmu::MMU, ppu::FetcherState};

pub struct BGFifo {
    data: Vec<u8>,

    data_low: u8,
    data_high: u8,
    tile_index: u8,
    fetcher_state: FetcherState,
    fetcher_state_ended: bool,
    fetcher_x: u8,
    in_window: bool,
    window_x: usize,
    pub window_y: usize,
}

impl Default for BGFifo {
    fn default() -> Self {
        Self {
            data: vec![],
            data_low: 0,
            data_high: 0,
            tile_index: 0,
            fetcher_state: FetcherState::GetTile,
            fetcher_state_ended: false,
            fetcher_x: 0,
            in_window: false,
            window_x: 0,
            window_y: 0,
        }
    }
}

impl BGFifo {
    pub fn tick(&mut self, mmu: &MMU, in_window: bool) {
        if in_window {
            if !self.in_window {
                self.window_x = 0;
                self.window_y += 1;
                self.data.clear();
                self.fetcher_state = FetcherState::GetTile;
                self.fetcher_state_ended = false;
            }
            self.in_window = in_window;
        }

        self.tick_fetcher(mmu);
    }

    pub fn get_pixel(&mut self) -> Option<u8> {
        self.data.pop()
    }

    pub fn reset(&mut self) {
        self.data.clear();
        self.fetcher_x = 0;
        self.in_window = false;
        self.fetcher_state = FetcherState::GetTile;
        self.fetcher_state_ended = false;
    }

    fn tick_fetcher(&mut self, mmu: &MMU) {
        match self.fetcher_state {
            FetcherState::GetTile => {
                if self.fetcher_state_ended {
                    self.fetcher_state = FetcherState::GetTileDataLow;
                    self.fetcher_state_ended = false;
                } else {
                    self.fetcher_get_tile(mmu);
                    self.fetcher_state_ended = true;
                }
            }
            FetcherState::GetTileDataLow => {
                if self.fetcher_state_ended {
                    self.fetcher_state = FetcherState::GetTileDataHigh;
                    self.fetcher_state_ended = false;
                } else {
                    self.fetcher_get_tile_data_low(mmu);
                    self.fetcher_state_ended = true;
                }
            }
            FetcherState::GetTileDataHigh => {
                if self.fetcher_state_ended {
                    self.fetcher_state = FetcherState::Sleep;
                    self.fetcher_state_ended = false;
                    return;
                } else {
                    self.fetcher_get_tile_data_high(mmu);
                    self.fetcher_state_ended = true;
                }
            }
            FetcherState::Sleep => {
                if self.fetcher_state_ended {
                    self.fetcher_state = FetcherState::Push;
                    self.fetcher_state_ended = false;
                } else {
                    self.fetcher_state_ended = true;
                }
            }
            FetcherState::Push => self.fetcher_push(mmu),
        }
    }

    fn get_tile_index(&self, tile_x: usize, tile_y: usize, tilemap: bool, mmu: &MMU) -> u8 {
        let tilemap = (tilemap as usize) << 10;
        let y = (tile_y & 0x1F) << 5;
        let x = tile_x & 0x1F;
        let adress = 0x9800 | tilemap | y | x;

        return mmu.mem[adress];
    }

    fn fetcher_get_tile(&mut self, mmu: &MMU) {
        if self.in_window {
            let tile_y = (self.window_y - 1) / 8;
            let tile_x = self.window_x;
            self.tile_index = self.get_tile_index(tile_x, tile_y, mmu.window_tile_map(), mmu);
        } else {
            let tile_y = mmu.ly().wrapping_add(mmu.scy()) as usize >> 3;
            let tile_x = self.fetcher_x.wrapping_add(mmu.scx() / 8) as usize;
            self.tile_index = self.get_tile_index(tile_x, tile_y, mmu.bg_tile_map_area(), mmu);
        }
    }

    fn get_tile_adress(&self, mmu: &MMU) -> usize {
        let offset = if mmu.bg_window_tile() {
            0x8000 + self.tile_index as usize * 16
        } else {
            0x9000 + self.tile_index as i8 as usize * 16
        };
        let py = mmu.ly().wrapping_add(mmu.scy()) as usize & 7;

        return offset + py * 2;
    }

    fn fetcher_get_tile_data_low(&mut self, mmu: &MMU) {
        self.data_low = mmu.mem[self.get_tile_adress(mmu)];
    }

    fn fetcher_get_tile_data_high(&mut self, mmu: &MMU) {
        self.data_high = mmu.mem[self.get_tile_adress(mmu) + 1];
    }

    fn fetcher_push(&mut self, mmu: &MMU) {
        if self.data.is_empty() {
            self.fetcher_state = FetcherState::GetTile;
            self.fetcher_state_ended = false;

            let start_index = if self.fetcher_x == 0 {
                mmu.scx() % 8
            } else {
                0
            };

            if self.in_window {
                self.window_x += 1;
            } else {
                self.fetcher_x += 1;
            }

            for i in start_index..8 {
                if mmu.bg_enabled() {
                    let high = (self.data_high >> i) & 1;
                    let low = (self.data_low >> i) & 1;
                    self.data.push((high << 1) | low);
                } else {
                    self.data.push(0);
                }
            }
        }
    }
}
