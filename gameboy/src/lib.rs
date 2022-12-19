mod memory;
mod cpu;
mod gpu;
mod io;

const CLOCK_SPEED: u32 = 4_194_304;
const SPEED: u32 = 10;

pub struct GameBoy {
    cpu: cpu::cpu::Cpu,
    ppu: gpu::ppu::Ppu,
    memory: memory::mmu::Mmu,
    timer: io::timer::Timer,
    cycles: u8,
}

impl GameBoy {
    pub fn new() -> Self {
        Self {
            cpu: cpu::cpu::Cpu::new(),
            ppu: gpu::ppu::Ppu::new(),
            memory: memory::mmu::Mmu::new(),
            timer: io::timer::Timer::new(),
            cycles: 0
        }
    }

    pub fn cycle(&mut self, frame: &mut [u8], fps: u32) {
        for _ in 0..(CLOCK_SPEED / fps) * SPEED {  // Perform 4_194_304 / fps cycles (1 frame)
            // If the CPU is halted, it will only wake up when an interrupt occurs.
            if self.cpu.get_halt(&mut self.memory) {
                continue;
            }

            if self.cycles == 0 {
                // Execute the next instruction when the current instruction is finished.
                self.cycles = self.cpu.cycle(&mut self.memory);
            }
            self.cycles -= 1;

            self.timer.tick(&mut self.memory);
            self.ppu.cycle(frame, &mut self.memory);
        }
    }

    pub fn get_memory(&self) -> &memory::mmu::Mmu {
        &self.memory
    }
}