#[cfg(test)]
mod blarggs_tests;

#[cfg(test)]
pub mod tests {
    use crate::core::Core;
    use std::{sync::mpsc, thread, time::Duration};

    pub fn panic_after<T, F>(d: Duration, f: F) -> T
    where
        T: Send + 'static,
        F: FnOnce() -> T,
        F: Send + 'static,
    {
        let (done_tx, done_rx) = mpsc::channel();
        let handle = thread::spawn(move || {
            let val = f();
            done_tx.send(()).expect("Unable to send completion signal");
            val
        });

        match done_rx.recv_timeout(d) {
            Ok(_) => handle.join().expect("Thread panicked"),
            Err(_) => panic!("Thread took too long"),
        }
    }

    pub fn run_until_passed_or_timeout(timeout: Duration, rom_path: String) -> bool {
        return panic_after(timeout, move || {
            let mut core = Core::default();
            core.cpu.af.set(0x01B0);
            core.cpu.bc.set(0x0013);
            core.cpu.de.set(0x00D8);
            core.cpu.hl.set(0x014D);
            core.cpu.sp.set(0xFFFE);
            core.cpu.pc.set(0x0100);
            // core.mmu.mem[0xFF00] = 0xFF;
            // core.mmu.mem[0xFF01] = 0x00;
            // core.mmu.mem[0xFF02] = 0x7E;
            // core.mmu.mem[0xFF04] = 0xAB;
            // core.mmu.mem[0xFF05] = 0x00;
            // core.mmu.mem[0xFF06] = 0x00;
            // core.mmu.mem[0xFF07] = 0xF8;
            // core.mmu.mem[0xFF0F] = 0xE1;
            // core.mmu.mem[0xFF40] = 0x91;
            // core.mmu.mem[0xFF41] = 0x85;
            // core.mmu.mem[0xFF42] = 0x00;
            // core.mmu.mem[0xFF43] = 0x00;
            core.mmu.mem[0xFF44] = 0x90;
            // core.mmu.mem[0xFF45] = 0x00;
            // core.mmu.mem[0xFF46] = 0xFF;
            // core.mmu.mem[0xFF47] = 0xFC;
            // core.mmu.mem[0xFF4A] = 0x00;
            // core.mmu.mem[0xFF4B] = 0x00;
            // core.mmu.mem[0xFF50] = 1;
            // core.mmu.mem[0xFFFF] = 0x00;

            core.mmu.mem[0xA000] = 0x80;
            core.mmu.bios_mapped = false;

            let rom = std::fs::read(rom_path).unwrap();
            core.load_rom(rom.as_slice(), rom.len());

            return core.run();
        });
    }
}
