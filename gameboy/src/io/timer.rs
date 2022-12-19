use crate::memory::mmu::Mmu;

pub struct Timer {
    pub counter: u16,
}

impl Timer {
    pub fn new () -> Self {
        Self {
            counter: 0,
        }
    }

    pub fn tick(&mut self, memory: &mut Mmu) {
        let old_div = u16::from_be_bytes([memory.read_byte(0xFF04), memory.read_byte(0xFF03)]);

        memory.increment_divider();
        
        if memory.read_byte(0xFF07) & 0x4 == 0 {
            return;
        }

        let tac_mask = match memory.read_byte(0xFF07) & 0x3 {
            0 => 0x200,
            1 => 0x8,
            2 => 0x20,
            3 => 0x80,
            _ => unreachable!()
        };

        let new_div = u16::from_be_bytes([memory.read_byte(0xFF04), memory.read_byte(0xFF03)]);
        // when bit 9 go low increment TIMA
        if (old_div & tac_mask) != 0 && (new_div & tac_mask) == 0 {
            let tima = memory.read_byte(0xFF05);
            if tima == 0xFF {
                memory.write_byte(0xFF05, memory.read_byte(0xFF06));
                memory.set_interrupt_flag(0x4);
            } else {
                memory.write_byte(0xFF05, tima + 1);
            }
        }
    }
}

#[test]
fn test_tick() {
    let mut memory = Mmu::new();
    let mut timer = Timer::new();

    memory.write_byte(0xFF07, 0x4); // Enable timer, set rate to 1024
    timer.tick(&mut memory);                  // Total ticks: 1
    assert_eq!(memory.read_byte(0xFF04), 0);  
    assert_eq!(memory.read_byte(0xFF05), 0);

    for _ in 0..255 {                         // Total ticks: 256
        timer.tick(&mut memory);
    }
    assert_eq!(memory.read_byte(0xFF04), (256 / 256) as u8); // 0
    assert_eq!(memory.read_byte(0xFF05), (256 / 1024) as u8); // 0

    for _ in 0..256 {                          // Total ticks: 512
        timer.tick(&mut memory);
    }
    assert_eq!(memory.read_byte(0xFF04), (512 / 256) as u8); // 2
    assert_eq!(memory.read_byte(0xFF05), (512 / 1024) as u8); // 0

    for _ in 0..512 {                         // Total ticks: 1024
        timer.tick(&mut memory);
    }
    assert_eq!(memory.read_byte(0xFF04), (1024 / 256) as u8); // 4
    assert_eq!(memory.read_byte(0xFF05), (1024 / 1024) as u8); // 1

    for _ in 0..260_096 {                         // Total ticks: 261_120
        timer.tick(&mut memory);
    }
    assert_eq!(memory.read_byte(0xFF04), (261_120 / 256) as u8); // 0xFC
    assert_eq!(memory.read_byte(0xFF05), (261_120 / 1024) as u8); // 0xFF

    let mut total_cycle = 261_120;
    for _ in 0..1_048_576 {                         // Total ticks: 1_309_696
        total_cycle += 1;
        timer.tick(&mut memory);
        assert_eq!(memory.read_byte(0xFF04), (total_cycle / 256) as u8);
        assert_eq!(memory.read_byte(0xFF05), (total_cycle / 1024) as u8);
    }
    assert_eq!(memory.read_byte(0xFF04), (1_309_696 / 256) as u8);
    assert_eq!(memory.read_byte(0xFF05), (1_309_696 / 1024) as u8);

    memory.write_byte(0xFF06, 0x15);
    for _ in 0..1024 {
        timer.tick(&mut memory);
    }
    assert_eq!(memory.read_byte(0xFF04), 0);
    assert_eq!(memory.read_byte(0xFF05), 0x15);
}