use std::time::Duration;

use crate::tests::{blarggs_tests::get_cpu_instr_path, tests::run_until_passed_or_timeout};

#[test]
fn lcd_sync() {
    let mut dir_path = get_cpu_instr_path();
    dir_path.push("oam_bug");
    dir_path.push("1-lcd_sync.gb");
    println!("{}", dir_path.display());

    assert!(run_until_passed_or_timeout(
        Duration::from_secs(500),
        dir_path.to_str().unwrap().into(),
    ));
}

#[test]
fn causes() {
    let mut dir_path = get_cpu_instr_path();
    dir_path.push("oam_bug");
    dir_path.push("2-causes.gb");
    println!("{}", dir_path.display());

    assert!(run_until_passed_or_timeout(
        Duration::from_secs(5),
        dir_path.to_str().unwrap().into(),
    ));
}

#[test]
fn non_causes() {
    let mut dir_path = get_cpu_instr_path();
    dir_path.push("oam_bug");
    dir_path.push("3-non_causes.gb");
    println!("{}", dir_path.display());

    assert!(run_until_passed_or_timeout(
        Duration::from_secs(5),
        dir_path.to_str().unwrap().into(),
    ));
}

#[test]
fn scanline_timing() {
    let mut dir_path = get_cpu_instr_path();
    dir_path.push("oam_bug");
    dir_path.push("4-scanline_timing.gb");
    println!("{}", dir_path.display());

    assert!(run_until_passed_or_timeout(
        Duration::from_secs(5),
        dir_path.to_str().unwrap().into(),
    ));
}

#[test]
fn timing_bug() {
    let mut dir_path = get_cpu_instr_path();
    dir_path.push("oam_bug");
    dir_path.push("5-timing_bug.gb");
    println!("{}", dir_path.display());

    assert!(run_until_passed_or_timeout(
        Duration::from_secs(5),
        dir_path.to_str().unwrap().into(),
    ));
}

#[test]
fn timing_no_bug() {
    let mut dir_path = get_cpu_instr_path();
    dir_path.push("oam_bug");
    dir_path.push("6-timing_no_bug.gb");
    println!("{}", dir_path.display());

    assert!(run_until_passed_or_timeout(
        Duration::from_secs(5),
        dir_path.to_str().unwrap().into(),
    ));
}

#[test]
fn timing_effect() {
    let mut dir_path = get_cpu_instr_path();
    dir_path.push("oam_bug");
    dir_path.push("7-timing_effect.gb");
    println!("{}", dir_path.display());

    assert!(run_until_passed_or_timeout(
        Duration::from_secs(5),
        dir_path.to_str().unwrap().into(),
    ));
}

#[test]
fn instr_effect() {
    let mut dir_path = get_cpu_instr_path();
    dir_path.push("oam_bug");
    dir_path.push("8-instr_effect.gb");
    println!("{}", dir_path.display());

    assert!(run_until_passed_or_timeout(
        Duration::from_secs(5),
        dir_path.to_str().unwrap().into(),
    ));
}
