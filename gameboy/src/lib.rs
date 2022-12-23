use std::path::PathBuf;

pub mod cpu;
mod gpu;
pub mod io;
pub mod memory;

const CLOCK_SPEED: u32 = 4_194_304;

pub struct GameBoy {
    pub cpu: cpu::cpu::Cpu,
    ppu: gpu::ppu::Ppu,
    pub mmu: memory::mmu::Mmu,
    cycles: u8,
    pub debug_paused: bool,
    pub speed: u32,
}

impl GameBoy {
    pub fn new() -> Self {
        Self {
            cpu: cpu::cpu::Cpu::new(),
            ppu: gpu::ppu::Ppu::new(),
            mmu: memory::mmu::Mmu::default(),
            cycles: 0,
            debug_paused: false,
            speed: 1,
        }
    }

    pub fn cycle(&mut self, frame: &mut [u8], fps: u32) {
        if self.debug_paused {
            return;
        }

        let mut cycles = 0;
        for _ in 0..(CLOCK_SPEED / fps) * self.speed {
            if !self.cpu.get_halt(&mut self.mmu) && cycles == 0 {
                cycles = self.cpu.cycle(&mut self.mmu);
            }

            cycles += io::interrupts::exectute_interrupts(&mut self.cpu, &mut self.mmu);
            self.ppu.update(frame, &mut self.mmu);
            io::timer::update(&mut self.mmu);
            cycles = cycles.checked_sub(1).unwrap_or(0);
        }
    }

    pub fn load_roam(&mut self, rom_path: PathBuf) {
        self.reset();
        self.mmu = memory::mmu::Mmu::new(rom_path)
    }

    pub fn reset(&mut self) {
        self.cpu = cpu::cpu::Cpu::new();
        self.ppu = gpu::ppu::Ppu::new();
        self.mmu = memory::mmu::Mmu::default();
        self.cycles = 0;
    }
}
