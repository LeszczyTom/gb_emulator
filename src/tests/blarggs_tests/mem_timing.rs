use std::time::Duration;

use crate::tests::{blarggs_tests::get_cpu_instr_path, tests::run_until_passed_or_timeout};

#[test]
fn read_timming() {
    let mut dir_path = get_cpu_instr_path();
    dir_path.push("mem_timing");
    dir_path.push("01-read_timing.gb");
    println!("{}", dir_path.display());

    assert!(run_until_passed_or_timeout(
        Duration::from_secs(5),
        dir_path.to_str().unwrap().into(),
    ));
}

#[test]
fn write_timming() {
    let mut dir_path = get_cpu_instr_path();
    dir_path.push("mem_timing");
    dir_path.push("02-write_timing.gb");
    println!("{}", dir_path.display());

    assert!(run_until_passed_or_timeout(
        Duration::from_secs(5),
        dir_path.to_str().unwrap().into(),
    ));
}

#[test]
fn modify_timming() {
    let mut dir_path = get_cpu_instr_path();
    dir_path.push("mem_timing");
    dir_path.push("03-modify_timing.gb");
    println!("{}", dir_path.display());

    assert!(run_until_passed_or_timeout(
        Duration::from_secs(5),
        dir_path.to_str().unwrap().into(),
    ));
}
