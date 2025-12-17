use std::time::Duration;

use crate::tests::{blarggs_tests::get_cpu_instr_path, tests::run_until_passed_or_timeout};

#[test]
fn instr_timing() {
    let mut dir_path = get_cpu_instr_path();
    dir_path.push("instr_timing");
    dir_path.push("instr_timing.gb");
    println!("{}", dir_path.display());

    assert!(run_until_passed_or_timeout(
        Duration::from_secs(5),
        dir_path.to_str().unwrap().into(),
    ));
}
