pub mod gameboy {
    pub mod cpu;
    pub mod ppu;
    pub mod memory;

    const WIDTH: u16 = 160;
    const _HEIGHT: u16 = 144;
    const CLOCK_SPEED: u32 = 4_194_304;
    const SPEED: u32 = 1;

    pub struct GameBoy {
        cpu: cpu::Cpu,
        ppu: ppu::Ppu,
        memory: memory::Memory,
        cycles: u32,
    }

    impl GameBoy {
        pub fn new() -> Self {
            Self {
                cpu: cpu::Cpu::new(),
                ppu: ppu::Ppu::new(),
                memory: memory::Memory::new(),
                cycles: 0,
            }
        }

        pub fn cycle(&mut self, frame: &mut [u8], fps: u32) {
            let mut cycles = 0;
            for _ in 0..(CLOCK_SPEED / fps) * SPEED {             
                self.cycles += 1;
                increment_timer(&mut self.memory, &mut self.cycles);

                // If the CPU is halted, it will only wake up when an interrupt occurs.
                if self.cpu.get_halt(&mut self.memory) {
                    continue;
                }

                cycles -= 1;
                // Execute the next instruction when the current instruction is finished.
                if cycles == 0 {
                    cycles = self.cpu.cycle(&mut self.memory);
                }

                self.ppu.cycle(frame, &mut self.memory);
            }
        }
    }
    
    fn increment_timer(memory: &mut memory::Memory, cycles: &mut u32) {
        // Increment divide register
        if *cycles == 256 {
            let divider = memory.read_byte(0xFF04);
            memory.write_byte(0xFF04, divider.wrapping_add(1));
        }

        // Timer disabled ?
        if memory.read_byte(0xff07) & 4 == 0 {
            *cycles = 0;
            return;
        }

        // Increment timer at rate specified by TAC.
        if *cycles == memory.get_timer_control() {
            *cycles = 0;
            let timer = memory.read_byte(0xFF05).overflowing_add(1);
            // If timer overflowed, set it to the value in TMA and request an interrupt.
            if timer.1 {
                memory.set_interrupt_flag(4);
                memory.write_byte(0xff05, memory.read_byte(0xff06));
                return
            }
            memory.write_byte(0xff05, timer.0);
        }
    }
}