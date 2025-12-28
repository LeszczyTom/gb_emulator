use crate::{mmu::MMU, ppu::fetcher::Fetcher};

#[derive(Default)]
pub struct FIFO {
    data: Vec<u8>,
    fetcher: Fetcher,
    out: u8,
}

impl FIFO {
    pub fn tick(&mut self, mmu: &mut MMU) -> Option<u8> {
        self.fetcher.tick(mmu, &mut self.data);

        if self.data.is_empty() {
            return None;
        }

        self.out += 1;
        return Some(self.data.pop().unwrap());
    }

    pub fn line_done(&self) -> bool {
        return self.out == 160;
    }

    pub fn reset(&mut self) {
        self.data = vec![];
        self.fetcher.fetcher_x = 0;
        self.out = 0;
    }
}
