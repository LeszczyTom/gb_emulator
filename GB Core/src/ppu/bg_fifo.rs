use std::mem;

use crate::{log, mmu::MMU, ppu::FetcherState};

pub struct BGFifo {
    data: Vec<u8>,

    data_low: u8,
    data_high: u8,
    tile_y: u8,
    tile_index: u8,
    fetcher_state: FetcherState,
    fetcher_state_ended: bool,
    fetcher_x: u8,
    in_window: bool,
    window_x: u8,
    pub window_y: u8,
}

impl Default for BGFifo {
    fn default() -> Self {
        Self {
            data: vec![],
            data_low: 0,
            data_high: 0,
            tile_y: 0,
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
    pub fn tick(&mut self, mmu: &MMU, fetch_paused: bool, in_window: bool) -> Option<u8> {
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

        self.tick_fetcher(mmu, fetch_paused);
        return self.data.pop();
    }

    pub fn reset(&mut self) {
        self.data.clear();
        self.fetcher_x = 0;
        self.in_window = false;
        self.fetcher_state = FetcherState::GetTile;
        self.fetcher_state_ended = false;
    }

    fn tick_fetcher(&mut self, mmu: &MMU, fetch_paused: bool) {
        match self.fetcher_state {
            FetcherState::GetTile => {
                if fetch_paused {
                    return;
                }

                if self.fetcher_state_ended {
                    self.fetcher_state = FetcherState::GetTileDataLow;
                    self.fetcher_state_ended = false;
                } else {
                    self.fetcher_get_tile(mmu);
                    self.fetcher_state_ended = true;
                }
            }
            FetcherState::GetTileDataLow => {
                if fetch_paused {
                    return;
                }

                if self.fetcher_state_ended {
                    self.fetcher_state = FetcherState::GetTileDataHigh;
                    self.fetcher_state_ended = false;
                } else {
                    self.fetcher_get_tile_data_low(mmu);
                    self.fetcher_state_ended = true;
                }
            }
            FetcherState::GetTileDataHigh => {
                if fetch_paused {
                    return;
                }

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

    fn tile_y(&self, mmu: &MMU) -> u8 {
        if self.in_window {
            mmu.ly()
        } else {
            ((mmu.ly() as u16 + mmu.scy() as u16) & 0xFF) as u8
        }
    }

    fn tile_map(&self, mmu: &MMU) -> u16 {
        if self.in_window {
            if mmu.window_tile_map() {
                0x9C00
            } else {
                0x9800
            }
        } else {
            if mmu.bg_tile_map_area() {
                0x9C00
            } else {
                0x9800
            }
        }
    }

    fn fetcher_get_tile(&mut self, mmu: &MMU) {
        if self.in_window {
            let tile_map = if mmu.window_tile_map() {
                0x9C00
            } else {
                0x9800
            };
            self.tile_y = self.window_y - 1;
            let x = self.window_x as u16;
            let y = self.tile_y as u16 / 8;
            let address = tile_map + y * 32 + x;

            self.tile_index = mmu.mem[address as usize];
        } else {
            let tile_map = if mmu.bg_tile_map_area() {
                0x9C00
            } else {
                0x9800
            };
            self.tile_y = ((mmu.ly() as u16 + mmu.scy() as u16) & 0xFF) as u8;
            let x = self.fetcher_x as u16;
            let y = self.tile_y as u16 / 8;
            let address = tile_map + y * 32 + x;

            self.tile_index = mmu.mem[address as usize];
        }
    }

    fn fetcher_get_tile_data_low(&mut self, mmu: &MMU) {
        let block_address = if mmu.bg_window_tile() { 0x8000 } else { 0x8800 };
        let offset = block_address + self.tile_index as u16 * 16;
        let address = offset + (self.tile_y as u16 % 8) * 2;
        self.data_low = mmu.mem[address as usize];
    }

    fn fetcher_get_tile_data_high(&mut self, mmu: &MMU) {
        let block_address = if mmu.bg_window_tile() { 0x8000 } else { 0x8800 };
        let offset = block_address + self.tile_index as u16 * 16;
        let address = offset + (self.tile_y as u16 % 8) * 2;
        self.data_high = mmu.mem[address as usize + 1];
    }

    fn fetcher_push(&mut self, mmu: &MMU) {
        if self.data.is_empty() {
            self.fetcher_state = FetcherState::GetTile;
            self.fetcher_state_ended = false;

            if self.in_window {
                self.window_x += 1;
            } else {
                self.fetcher_x += 1;
            }

            for i in 0..8 {
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
