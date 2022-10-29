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
    }

    impl GameBoy {
        pub fn new() -> Self {
            Self {
                cpu: cpu::Cpu::new(),
                ppu: ppu::Ppu::new(),
                memory: memory::Memory::new(),
            }
        }

        pub fn cycle(&mut self, frame: &mut [u8], fps: u32) {
            let mut cycles = 0;
            for _ in 0..(CLOCK_SPEED / fps) * SPEED {
                if self.cpu.get_halt() {
                    continue;
                }
                
                cycles -= 1;

                if cycles == 0 {
                    cycles = self.cpu.cycle(&mut self.memory);
                }

                self.ppu.cycle(frame, &mut self.memory);
            }
        }
    }
}