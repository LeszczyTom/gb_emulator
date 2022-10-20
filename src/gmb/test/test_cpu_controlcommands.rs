#[test]
fn test_ccf() {
    let mut gmb = super::get_test_gmb(0x3f);
    super::set_flags([false; 4], &mut gmb);
    assert_eq!(gmb.cycle(), 4);
    assert_eq!(super::check_flags([false, false, false, true], &gmb), true);
    assert_eq!(gmb.cpu.get_pc(), 0x0101);

    let mut gmb = super::get_test_gmb(0x3f);
    super::set_flags([true, true, true, true], &mut gmb);
    assert_eq!(gmb.cycle(), 4);
    assert_eq!(super::check_flags([true, false, false, false], &gmb), true);
    assert_eq!(gmb.cpu.get_pc(), 0x0101);
}

#[test]
fn test_scf() {
    let mut gmb = super::get_test_gmb(0x37);
    super::set_flags([false; 4], &mut gmb);
    assert_eq!(gmb.cycle(), 4);
    assert_eq!(super::check_flags([false, false, false, true], &gmb), true);
    assert_eq!(gmb.cpu.get_pc(), 0x0101);

    let mut gmb = super::get_test_gmb(0x37);
    super::set_flags([true, true, true, true], &mut gmb);
    assert_eq!(gmb.cycle(), 4);
    assert_eq!(super::check_flags([true, false, false, true], &gmb), true);
}

#[test]
fn test_nop() {
    let mut gmb = super::get_test_gmb(0x00);
    super::set_flags([true, false, false, true], &mut gmb);
    assert_eq!(gmb.cycle(), 4);
    assert_eq!(super::check_flags([true, false, false, true], &gmb), true);
    assert_eq!(gmb.cpu.get_pc(), 0x0101);
}

#[test]
fn test_halt() {
    let mut gmb = super::get_test_gmb(0x76);
    assert_eq!(gmb.cycle(), 4);
    assert_eq!(gmb.cpu.get_pc(), 0x0101);
    assert_eq!(gmb.cpu.get_halt(), true);

    assert_eq!(gmb.cycle(), 4);
    assert_eq!(gmb.cpu.get_pc(), 0x0101);
    assert_eq!(gmb.cpu.get_halt(), true);

    gmb.cpu.clear_halt();

    assert_eq!(gmb.cycle(), 4);
    assert_eq!(gmb.cpu.get_pc(), 0x0102);

    // TODO: complete halt test
}

#[test]
#[should_panic(expected = "STOP")]
fn test_stop() {
    let mut gmb = super::get_test_gmb(0x10);
    gmb.cycle();
}

#[test]
fn test_di() {
    let mut gmb = super::get_test_gmb(0xf3);
    gmb.cpu.set_ime();
    assert_eq!(gmb.cycle(), 4);
    assert_eq!(gmb.cpu.get_ime(), false);
    assert_eq!(gmb.cpu.get_pc(), 0x0101);
}

#[test]
fn test_ei() {
    let mut gmb = super::get_test_gmb(0xfb);
    gmb.cpu.clear_ime();
    assert_eq!(gmb.cycle(), 4);
    assert_eq!(gmb.cpu.get_ime(), true);
    assert_eq!(gmb.cpu.get_pc(), 0x0101);
}