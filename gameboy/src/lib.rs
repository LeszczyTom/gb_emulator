mod memory;
mod cpu;
mod gpu;
pub mod io;

const CLOCK_SPEED: u32 = 4_194_304;
const SPEED: u32 = 1;

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

    pub fn cycle(&mut self, frame: &mut [u8], fps: u32, debug_values: &mut io::debug::Debug) {
        for _ in 0..(CLOCK_SPEED / fps) * SPEED {  // Perform 4_194_304 / fps cycles (1 frame)
            // If the CPU is halted, it will only wake up when an interrupt occurs.
            if self.cpu.get_halt(&mut self.memory) {
                continue;
            }

            self.update_debug_values(debug_values);

            if self.cycles == 0 {
                // Execute the next instruction when the current instruction is finished.
                self.cycles = self.cpu.cycle(&mut self.memory);
            }
            self.cycles -= 1;

            self.timer.tick(&mut self.memory);
            self.ppu.cycle(frame, &mut self.memory);
        }
    }

    fn update_debug_values(&mut self, debug_values: &mut io::debug::Debug) {
        debug_values.cycles = debug_values.cycles.wrapping_add(1);
        debug_values.a = self.cpu.get_r(cpu::cpu::Register::A);
        debug_values.b = self.cpu.get_r(cpu::cpu::Register::B);
        debug_values.c = self.cpu.get_r(cpu::cpu::Register::C);
        debug_values.d = self.cpu.get_r(cpu::cpu::Register::D);
        debug_values.e = self.cpu.get_r(cpu::cpu::Register::E);
        debug_values.h = self.cpu.get_r(cpu::cpu::Register::H);
        debug_values.l = self.cpu.get_r(cpu::cpu::Register::L);
        debug_values.f = self.cpu.get_r(cpu::cpu::Register::F);
        debug_values.pc = self.cpu.get_rr(cpu::cpu::RegisterPair::PC);
        debug_values.sp = self.cpu.get_rr(cpu::cpu::RegisterPair::SP);
    }

    pub fn get_memory(&self) -> &memory::mmu::Mmu {
        &self.memory
    }
}