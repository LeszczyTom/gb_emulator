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
            // core.mmu.mem[0xFF44] = 0x90;
            core.mmu.mem[0xFF02] = 0xFF;
            core.mmu.mem[0xA000] = 0x67;

            core.mmu
                .load_file(rom_path.into())
                .expect("Couldn't load file");

            return core.run();
        });
    }
}
