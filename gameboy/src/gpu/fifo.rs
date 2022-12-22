use crate::gpu::fetcher::Fetcher;
use crate::memory::mmu::Mmu;
use std::collections::VecDeque;

pub struct Fifo {
    fetcher: Fetcher,
    data: VecDeque<([u8; 4])>,
    clock: bool,
}

impl Fifo {
    pub fn new() -> Fifo {
        Fifo {
            fetcher: Fetcher::new(),
            data: VecDeque::new(),
            clock: false,
        }
    }

    pub fn clear(&mut self) {
        self.data.clear();
        self.fetcher = Fetcher::new();
        self.clock = false;
    }

    pub fn cycle(&mut self, memory: &mut Mmu) -> Option<([u8; 4])> {
        if self.clock {
            // 1Mhz
            self.clock = false;
            return self.push();
        }
        self.clock = true;

        self.fetcher.cycle(memory, &mut self.data);
        self.push()
    }

    fn push(&mut self) -> Option<([u8; 4])> {
        if self.data.len() <= 7 {
            return None;
        }

        self.data.pop_front()
    }
}
