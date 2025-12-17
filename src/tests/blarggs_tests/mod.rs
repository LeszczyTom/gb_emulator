use std::path::PathBuf;

mod cpu_tests;
mod instr_timing;
mod interrupt_time;
mod mem_timing;

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
