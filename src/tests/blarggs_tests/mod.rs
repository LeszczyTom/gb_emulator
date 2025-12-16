mod cpu_tests;
mod instr_timing;
mod interrupt_time;
mod mem_timing;

#[cfg(test)]
mod blargg {
    use std::path::PathBuf;

    pub fn get_cpu_instr_path() -> PathBuf {
        let mut dir_path = std::env::current_exe().expect("Can't find path to executable");
        dir_path.pop();
        dir_path.pop();
        dir_path.pop();
        dir_path.pop();
        dir_path.push("resources");
        dir_path.push("blargg");
        return dir_path;
    }
}
