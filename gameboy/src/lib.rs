pub mod memory;
pub mod cpu;
mod gpu;
pub mod io;

const CLOCK_SPEED: u32 = 4_194_304;

pub struct GameBoy {
    pub cpu: cpu::cpu::Cpu,
    ppu: gpu::ppu::Ppu,
    pub mmu: memory::mmu::Mmu,
    timer: io::timer::Timer,
    cycles: u8,
    pub debug_paused: bool,
    pub speed: u32,
}

impl GameBoy {
    pub fn new() -> Self {
        Self {
            cpu: cpu::cpu::Cpu::new(),
            ppu: gpu::ppu::Ppu::new(),
            mmu: memory::mmu::Mmu::new(),
            timer: io::timer::Timer::new(),
            cycles: 0,
            debug_paused: false,
            speed: 1,
        }
    }

    pub fn cycle(&mut self, frame: &mut [u8], fps: u32) {
        if self.debug_paused {
            return
        }

        for _ in 0..(CLOCK_SPEED / fps) * self.speed {  // Perform 4_194_304 / fps cycles (1 frame)
            // If the CPU is halted, it will only wake up when an interrupt occurs.
            if self.cpu.get_halt(&mut self.mmu) {
                continue;
            }

            if self.cycles == 0 {
                // Execute the next instruction when the current instruction is finished.
                self.cycles = self.cpu.cycle(&mut self.mmu);
            }
            self.cycles -= 1;

            self.timer.tick(&mut self.mmu);
            self.ppu.cycle(frame, &mut self.mmu);
        }
    }

    pub fn reset(&mut self) {
        self.cpu = cpu::cpu::Cpu::new();
        self.ppu = gpu::ppu::Ppu::new();
        self.mmu = memory::mmu::Mmu::new();
        self.timer = io::timer::Timer::new();
        self.cycles = 0;
    }
}