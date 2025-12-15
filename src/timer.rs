use crate::mmu::MMU;

const DIV_ADDRESS: u16 = 0xFF04;
const TIMA_ADDRESS: u16 = 0xFF05;
const TMA_ADDRESS: u16 = 0xFF06;
const TAC_ADDRESS: u16 = 0xFF07;

#[derive(Default)]
pub struct Timer {
    div_ticks: u16,
    tima_ticks: u16,
}

impl Timer {
    pub fn cycle(&mut self, mmu: &mut MMU, cycles: u8) {
        self.tick_div(mmu, cycles);

        if mmu.get(TAC_ADDRESS) & 4 == 4 {
            self.tick_tima(mmu, cycles);
        }
    }

    fn tick_div(&mut self, mmu: &mut MMU, cycles: u8) {
        for _ in 0..cycles {
            self.div_ticks += 1;

            if self.div_ticks >= 256 {
                self.div_ticks = 0;
                let div = mmu.get(DIV_ADDRESS).wrapping_add(1);
                mmu.mem[DIV_ADDRESS as usize] = div;
            }
        }
    }

    fn tick_tima(&mut self, mmu: &mut MMU, cycles: u8) {
        for _ in 0..cycles {
            self.tima_ticks += 1;

            if self.tima_ticks >= self.get_tima_frequency(mmu.get(TAC_ADDRESS)) {
                self.tima_ticks = 0;
                let tima: u8 = mmu.get(TIMA_ADDRESS);

                if tima == 0xFF {
                    mmu.set_interrupt_flag(2);
                    mmu.set(mmu.get(TMA_ADDRESS), TIMA_ADDRESS);
                } else {
                    mmu.set(tima + 1, TIMA_ADDRESS);
                }
            }
        }
    }

    fn get_tima_frequency(&self, tac: u8) -> u16 {
        return match tac & 0b11 {
            0b00 => 1024,
            0b01 => 16,
            0b10 => 64,
            0b11 => 256,
            _ => unreachable!("Something went worng with TIMA"),
        };
    }
}
