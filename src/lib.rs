pub mod gameboy {
    pub mod cpu;
    pub mod ppu;
    pub mod memory;
    pub mod timer;

    const WIDTH: u16 = 160;
    const _HEIGHT: u16 = 144;
    const CLOCK_SPEED: u32 = 4_194_304;
    const SPEED: u32 = 1;

    pub struct GameBoy {
        cpu: cpu::Cpu,
        ppu: ppu::Ppu,
        memory: memory::Memory,
        timer: timer::Timer,
        cycles: u8,
    }

    impl GameBoy {
        pub fn new() -> Self {
            Self {
                cpu: cpu::Cpu::new(),
                ppu: ppu::Ppu::new(),
                memory: memory::Memory::new(),
                timer: timer::Timer::new(),
                cycles: 0
            }
        }

        pub fn cycle(&mut self, frame: &mut [u8], fps: u32) {
            for _ in 0..(CLOCK_SPEED / fps) * SPEED {  // Perform 4_194_304 / fps cycles (1 frame)
                // If the CPU is halted, it will only wake up when an interrupt occurs.
                if !self.cpu.get_halt(&mut self.memory) && self.cycles == 0 {
                    // Execute the next instruction when the current instruction is finished.
                    self.cycles = self.cpu.cycle(&mut self.memory);
                }
                self.cycles -= 1;

                self.timer.tick(&mut self.memory);
                self.ppu.cycle(frame, &mut self.memory);
            }
        }
    }
}