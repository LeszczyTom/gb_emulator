use crate::{
    log,
    mmu::MMU,
    ppu::{FetcherState, object::OamObject},
};

pub struct OamFifo {
    data: Vec<(u8, bool)>,
    pub object: Option<OamObject>,
    tile_index: Option<u8>,
    pub fetcher_x: u8,
    data_low: u8,
    data_high: u8,
    fetcher_state: FetcherState,
    fetcher_state_ended: bool,
}

impl Default for OamFifo {
    fn default() -> Self {
        Self {
            data: vec![],
            object: None,
            tile_index: None,
            fetcher_x: 0,
            data_low: 0,
            data_high: 0,
            fetcher_state: FetcherState::GetTile,
            fetcher_state_ended: false,
        }
    }
}

impl OamFifo {
    pub fn tick(&mut self, mmu: &MMU) {
        self.tick_fetcher(mmu);
    }

    pub fn get_pixel(&mut self) -> Option<(u8, bool)> {
        self.data.pop()
    }

    pub fn reset(&mut self) {
        self.data.clear();
        self.fetcher_x = 0;
        self.object = None;
        self.fetcher_state = FetcherState::GetTile;
        self.fetcher_state_ended = false;
    }

    pub fn fetching(&self) -> bool {
        return self.object.is_some()
            && self.fetcher_state_ended
            && (self.fetcher_state == FetcherState::GetTile
                || self.fetcher_state == FetcherState::GetTileDataLow
                || self.fetcher_state == FetcherState::GetTileDataHigh);
    }

    fn tick_fetcher(&mut self, mmu: &MMU) {
        match self.fetcher_state {
            FetcherState::GetTile => {
                if self.fetcher_state_ended {
                    self.fetcher_state = FetcherState::GetTileDataLow;
                    self.fetcher_state_ended = false;
                } else {
                    self.tile_index = if let Some(obj) = self.object {
                        Some(obj.tile_index())
                    } else {
                        None
                    };

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

    fn fetcher_get_tile_data_low(&mut self, mmu: &MMU) {
        if let Some(index) = self.tile_index {
            let offset = 0x8000 + index as u16 * 16;
            let address = offset + (mmu.ly() as u16 % 8) * 2;
            self.data_low = mmu.mem[address as usize];
        }
    }

    fn fetcher_get_tile_data_high(&mut self, mmu: &MMU) {
        if let Some(index) = self.tile_index {
            let offset = 0x8000 + index as u16 * 16;
            let address = offset + (mmu.ly() as u16 % 8) * 2;
            self.data_low = mmu.mem[address as usize + 1];
        }
    }

    fn fetcher_push(&mut self, mmu: &MMU) {
        if self.data.is_empty() {
            let priority = if let Some(obj) = self.object {
                !obj.priority()
            } else {
                false
            };

            if mmu.obj_enable()
                && let Some(obj) = self.object
            {
                for i in 0..8 {
                    let high = (self.data_high >> i) & 1;
                    let low = (self.data_low >> i) & 1;
                    let data = mmu.convert_obj_color((high << 1) | low, obj.palette());
                    // self.data.push((data, priority));
                    self.data.push((data, true));
                }

                self.object = None;
            } else {
                for _ in 0..8 {
                    self.data.push((3, false));
                }
            }

            self.fetcher_x += 1;
            self.fetcher_state = FetcherState::GetTile;
            self.fetcher_state_ended = false;
        }
    }
}
