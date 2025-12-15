#[cfg(test)]
mod cpu_test {
    use crate::core::Core;
    use std::{path::PathBuf, sync::mpsc, thread, time::Duration};

    fn panic_after<T, F>(d: Duration, f: F) -> T
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

    fn run_until_passed_or_timeout(timeout: Duration, rom_path: String) -> bool {
        return panic_after(timeout, move || {
            let mut core = Core::default();
            core.cpu.af.set(0x01B0);
            core.cpu.bc.set(0x0013);
            core.cpu.de.set(0x00D8);
            core.cpu.hl.set(0x014D);
            core.cpu.sp.set(0xFFFE);
            core.cpu.pc.set(0x0100);
            core.mmu.mem[0xFF44] = 0x90;
            core.mmu
                .load_file(rom_path.into())
                .expect("Couldn't load file");

            return core.run();
        });
    }

    fn get_cpu_instr_path() -> PathBuf {
        let mut dir_path = std::env::current_exe().expect("Can't find path to executable");
        dir_path.pop();
        dir_path.pop();
        dir_path.pop();
        dir_path.pop();
        dir_path.push("resources");
        dir_path.push("cpu_instr");
        return dir_path;
    }

    #[test]
    fn cpu_01() {
        let mut dir_path = get_cpu_instr_path();
        dir_path.push("01-special.gb");
        println!("{}", dir_path.display());

        assert!(run_until_passed_or_timeout(
            Duration::from_secs(5),
            dir_path.to_str().unwrap().into(),
        ));
    }

    #[test]
    fn cpu_02() {
        let mut dir_path = get_cpu_instr_path();
        dir_path.push("02-interrupts.gb");
        println!("{}", dir_path.display());

        assert!(run_until_passed_or_timeout(
            Duration::from_secs(5),
            dir_path.to_str().unwrap().into(),
        ));
    }

    #[test]
    fn cpu_03() {
        let mut dir_path = get_cpu_instr_path();
        dir_path.push("03-op sp,hl.gb");
        println!("{}", dir_path.display());

        assert!(run_until_passed_or_timeout(
            Duration::from_secs(5),
            dir_path.to_str().unwrap().into(),
        ));
    }

    #[test]
    fn cpu_04() {
        let mut dir_path = get_cpu_instr_path();
        dir_path.push("04-op r,imm.gb");
        println!("{}", dir_path.display());

        assert!(run_until_passed_or_timeout(
            Duration::from_secs(5),
            dir_path.to_str().unwrap().into(),
        ));
    }

    #[test]
    fn cpu_05() {
        let mut dir_path = get_cpu_instr_path();
        dir_path.push("05-op rp.gb");
        println!("{}", dir_path.display());

        assert!(run_until_passed_or_timeout(
            Duration::from_secs(5),
            dir_path.to_str().unwrap().into(),
        ));
    }

    #[test]
    fn cpu_06() {
        let mut dir_path = get_cpu_instr_path();
        dir_path.push("06-ld r,r.gb");
        println!("{}", dir_path.display());

        assert!(run_until_passed_or_timeout(
            Duration::from_secs(5),
            dir_path.to_str().unwrap().into(),
        ));
    }
    #[test]
    fn cpu_07() {
        let mut dir_path = get_cpu_instr_path();
        dir_path.push("07-jr,jp,call,ret,rst.gb");
        println!("{}", dir_path.display());

        assert!(run_until_passed_or_timeout(
            Duration::from_secs(5),
            dir_path.to_str().unwrap().into(),
        ));
    }
    #[test]
    fn cpu_08() {
        let mut dir_path = get_cpu_instr_path();
        dir_path.push("08-misc instrs.gb");
        println!("{}", dir_path.display());

        assert!(run_until_passed_or_timeout(
            Duration::from_secs(5),
            dir_path.to_str().unwrap().into(),
        ));
    }
    #[test]
    fn cpu_09() {
        let mut dir_path = get_cpu_instr_path();
        dir_path.push("09-op r,r.gb");
        println!("{}", dir_path.display());

        assert!(run_until_passed_or_timeout(
            Duration::from_secs(20),
            dir_path.to_str().unwrap().into(),
        ));
    }
    #[test]
    fn cpu_10() {
        let mut dir_path = get_cpu_instr_path();
        dir_path.push("10-bit ops.gb");
        println!("{}", dir_path.display());

        assert!(run_until_passed_or_timeout(
            Duration::from_secs(5),
            dir_path.to_str().unwrap().into(),
        ));
    }
    #[test]
    fn cpu_11() {
        let mut dir_path = get_cpu_instr_path();
        dir_path.push("11-op a,(hl).gb");
        println!("{}", dir_path.display());

        assert!(run_until_passed_or_timeout(
            Duration::from_secs(5),
            dir_path.to_str().unwrap().into(),
        ));
    }

    #[test]
    fn instr_timing() {
        let mut dir_path = get_cpu_instr_path();
        dir_path.pop();
        dir_path.push("instr_timing.gb");
        println!("{}", dir_path.display());

        assert!(run_until_passed_or_timeout(
            Duration::from_secs(5),
            dir_path.to_str().unwrap().into(),
        ));
    }
}
