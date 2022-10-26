pub mod gameboy {
    pub mod cpu;
    pub mod ppu;
    pub mod memory;

    const WIDTH: u16 = 160;
    const _HEIGHT: u16 = 144;
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

        pub fn cycle(&mut self, frame: &mut [u8]) {
            self.ppu.cycle(frame, &mut self.memory);
            self.cpu.cycle(&mut self.memory);
        }
    }
}