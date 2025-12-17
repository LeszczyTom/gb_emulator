use std::time::Duration;

use crate::tests::{blarggs_tests::get_cpu_instr_path, tests::run_until_passed_or_timeout};

#[test]
fn cpu_01() {
    let mut dir_path = get_cpu_instr_path();
    dir_path.push("cpu_instr");
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
    dir_path.push("cpu_instr");
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
    dir_path.push("cpu_instr");
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
    dir_path.push("cpu_instr");
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
    dir_path.push("cpu_instr");
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
    dir_path.push("cpu_instr");
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
    dir_path.push("cpu_instr");
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
    dir_path.push("cpu_instr");
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
    dir_path.push("cpu_instr");
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
    dir_path.push("cpu_instr");
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
    dir_path.push("cpu_instr");
    dir_path.push("11-op a,(hl).gb");
    println!("{}", dir_path.display());

    assert!(run_until_passed_or_timeout(
        Duration::from_secs(5),
        dir_path.to_str().unwrap().into(),
    ));
}
