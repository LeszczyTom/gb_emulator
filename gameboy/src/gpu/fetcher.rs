use crate::memory::mmu::Mmu;
use std::collections::VecDeque;

const COLORS: [[u8; 4]; 4] = [
    [0xe0, 0xf8, 0xd0, 0xff],
    [0x88, 0xc0, 0x70, 0xff],
    [0x34, 0x68, 0x56, 0xff],
    [0x08, 0x18, 0x20, 0xff],
    ];

enum Step {
    ReadTileId,
    ReadData0,
    ReadData1,
    Idle,
}

pub struct Fetcher {
    step: Step,
    tile_id: u8,
    data0: u8,
    data1: u8,
    x: usize,
}

impl Fetcher {
    pub fn new() -> Fetcher {
        Fetcher {
            step: Step::ReadTileId,
            tile_id: 0,
            data0: 0,
            data1: 0,
            x: 0,
        }
    }

    pub fn cycle(&mut self, memory: &mut Mmu, fifo: &mut VecDeque<([u8; 4])>) {
        let scx = memory.get_scx();
        let scy = memory.get_scy();
        let x = ((scx / 8) + self.x as u8) & 0x1F;
        let y = (memory.get_ly() + scy) & 255;
        let row: u16 = y as u16 / 8;
        let col = x as u16 ;

        let offset = 0x8000 + self.tile_id as u16 * 16;
        let addr = offset + (y as u16 % 8) * 2;
        match self.step {
            Step::Idle => {
                if fifo.len() <= 7 {
                    self.step = Step::ReadTileId;
                    for i in 0..8 {
                        let mut data = [0; 2];
                        data[0] = self.data0 >> (7 - i) & 1;
                        data[1] = self.data1 >> (7 - i) & 1;
                        fifo.push_back(COLORS[memory.get_background_palette((data[1] << 1) | data[0])]);
                    }
                }
            }
            Step::ReadTileId => {
                self.tile_id = memory.read_byte(0x8000 + 0x1800 + (row * 32 + col)); // 0x1800 or 0x1C00
                self.step = Step::ReadData0;
            }
            Step::ReadData0 => {
                self.data0 = memory.read_byte(addr);
                self.step = Step::ReadData1;
            }
            Step::ReadData1 => {
                self.data1 = memory.read_byte(addr + 1);
                self.x += 1;
                self.step = Step::Idle;
            }
        }
    }
}