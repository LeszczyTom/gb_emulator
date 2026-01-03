use crate::{
    log,
    mmu::MMU,
    ppu::{FetcherState, object::OamObject},
};

pub struct OamFifo {
    data: Vec<(u8, bool)>,
    pub object: Option<OamObject>,
    fetcher_object: Option<OamObject>,
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
            fetcher_object: None,
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
        self.object = None;
        self.fetcher_object = None;
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
                    self.fetcher_object = self.object;
                    self.object = None;
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
        if let Some(object) = self.fetcher_object {
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

    fn fetcher_get_tile_data_high(&mut self, mmu: &MMU) {
        if let Some(object) = self.fetcher_object {
            let offset = 0x8000 + object.tile_index() as u16 * 16;
            let y = if object.flip_y() {
                7 - mmu.ly() as u16 % 8
            } else {
                mmu.ly() as u16 % 8
            };
            let address = offset + y * 2;
            self.data_low = mmu.mem[address as usize + 1];
        }
    }

    fn fetcher_push(&mut self, mmu: &MMU) {
        if self.data.is_empty() {
            if mmu.obj_enable()
                && let Some(obj) = self.fetcher_object
            {
                for i in 0..8 {
                    let shift = if obj.flip_x() { 8 - i } else { i };
                    let high = (self.data_high >> shift) & 1;
                    let low = (self.data_low >> shift) & 1;
                    let data = mmu.convert_obj_color((high << 1) | low, obj.palette());
                    self.data.push((data, !obj.priority()));
                }

                self.fetcher_object = None;
            } else {
                for _ in 0..8 {
                    self.data.push((0, false));
                }
            }

            self.fetcher_state = FetcherState::GetTile;
            self.fetcher_state_ended = false;
        }
    }
}
