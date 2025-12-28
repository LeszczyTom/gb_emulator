use std::path::PathBuf;

mod cpu_tests;
mod instr_timing;
mod interrupt_time;
mod mem_timing;
mod mem_timing2;
mod oam_bug;

#[test]
fn halt_bug() {
    let mut dir_path = get_cpu_instr_path();
    dir_path.push("halt_bug.gb");
    println!("{}", dir_path.display());

    assert!(crate::tests::tests::run_until_passed_or_timeout(
        std::time::Duration::from_secs(5),
        dir_path.to_str().unwrap().into(),
    ));
}

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
