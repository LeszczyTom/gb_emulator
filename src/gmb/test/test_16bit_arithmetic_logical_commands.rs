// add HL,rr - opcode[0x09, 0x19, 0x29, 0x39]
#[test]
fn test_add_hl_rr() {
    let tuples: [(u8, &str); 4] = [(0x09, "bc"), (0x19, "de"), (0x29, "hl"), (0x39, "sp")];
    for tuple in tuples {
        test_add_hl_rr_flag_h(tuple);
        test_add_hl_rr_flag_c(tuple);
    }
}

#[cfg(test)]
fn test_add_hl_rr_flag_h(tuple: (u8, &str)) { // Set if there is a carry from bit 11 ; otherwise reset
    let mut gmb = super::get_test_gmb(tuple.0);
    gmb.cpu.set_rr("hl", 0x0800);
    gmb.cpu.set_rr(tuple.1, 0x0800);
    let expected_flags = [gmb.cpu.get_flag("z"), false, true, false];
    assert_eq!(gmb.cycle(), 8);                 // 8 cycles
    assert_eq!(gmb.cpu.get_rr("hl"), 0x1000);   // hl = 0x1000
    assert_eq!(super::check_flags(expected_flags, &gmb), true);
}

#[cfg(test)]
fn test_add_hl_rr_flag_c(tuple: (u8, &str)) { // Set if there is a carry from bit 15 ; otherwise reset
    let mut gmb = super::get_test_gmb(tuple.0);
    gmb.cpu.set_rr("hl", 0x8000);
    gmb.cpu.set_rr(tuple.1, 0x8000);
    let expected_flags = [gmb.cpu.get_flag("z"), false, false, true];
    assert_eq!(gmb.cycle(), 8);                 // 8 cycles
    assert_eq!(gmb.cpu.get_rr("hl"), 0x0000);   // hl = 0x0000
    assert_eq!(super::check_flags(expected_flags, &gmb), true);
}

#[test]
fn test_add_hl_rr_from_book() {
    let mut gmb = super::get_test_gmb(0x09);
    gmb.cpu.set_rr("hl", 0x8a23);
    gmb.cpu.set_rr("bc", 0x0605);
    let expected_flags = [gmb.cpu.get_flag("z"), false, true, false];
    assert_eq!(gmb.cycle(), 8);                 // 8 cycles
    assert_eq!(gmb.cpu.get_rr("hl"), 0x9028);   // hl = 0x9028
    assert_eq!(super::check_flags(expected_flags, &gmb), true);

    let mut gmb = super::get_test_gmb(0x29);
    gmb.cpu.set_rr("hl", 0x8a23);
    let expected_flags = [gmb.cpu.get_flag("z"), false, true, true];
    assert_eq!(gmb.cycle(), 8);                 // 8 cycles
    assert_eq!(gmb.cpu.get_rr("hl"), 0x1446);   // hl = 0x1446
    assert_eq!(super::check_flags(expected_flags, &gmb), true);
}

// inc rr - opcode[0x03, 0x13, 0x23, 0x33]
#[test]
fn test_inc_rr() {
    let tuples: [(u8, &str); 4] = [(0x03, "bc"), (0x13, "de"), (0x23, "hl"), (0x33, "sp")];
    for tuple in tuples {
        let mut gmb = super::get_test_gmb(tuple.0);
        gmb.cpu.set_rr(tuple.1, 0x0800);
        let expected_flags = [gmb.cpu.get_flag("z"), gmb.cpu.get_flag("n"), gmb.cpu.get_flag("h"), gmb.cpu.get_flag("c")];
        assert_eq!(gmb.cycle(), 8);                 // 8 cycles
        assert_eq!(gmb.cpu.get_rr(tuple.1), 0x801);   // hl = 0x1000
        assert_eq!(super::check_flags(expected_flags, &gmb), true);
    }
}

// dec rr - opcode[0x0b, 0x1b, 0x2b, 0x3b]
#[test]
fn test_dec_rr() {
    let tuples: [(u8, &str); 4] = [(0x0b, "bc"), (0x1b, "de"), (0x2b, "hl"), (0x3b, "sp")];
    for tuple in tuples {
        let mut gmb = super::get_test_gmb(tuple.0);
        gmb.cpu.set_rr(tuple.1, 0x0800);
        let expected_flags = [gmb.cpu.get_flag("z"), gmb.cpu.get_flag("n"), gmb.cpu.get_flag("h"), gmb.cpu.get_flag("c")];
        assert_eq!(gmb.cycle(), 8);                 // 8 cycles
        assert_eq!(gmb.cpu.get_rr(tuple.1), 0x7ff); // hl = 0x1000
        assert_eq!(super::check_flags(expected_flags, &gmb), true);
    }
}

// add sp, n - opcode[0xe8]
#[test]
fn test_add_sp_n() {
    test_add_sp_n_flag_h();
    test_add_sp_n_flag_c();
    test_add_sp_n_from_book();
}

#[cfg(test)]
fn test_add_sp_n_flag_h() {
    let mut gmb = super::get_test_gmb(0xe8);
    gmb.cpu.set_rr("sp", 0x0fff);
    gmb.memory.write_byte(0x101, 0x01); // n = 0x01
    let expected_flags = [false, false, true, false];
    assert_eq!(gmb.cycle(), 16);                  // 16 cycles
    assert_eq!(gmb.cpu.get_rr("sp"), 0x1000);     // sp = 0x8001
    assert_eq!(super::check_flags(expected_flags, &gmb), true);
}

#[cfg(test)]
fn test_add_sp_n_flag_c() {
    let mut gmb = super::get_test_gmb(0xe8);
    gmb.cpu.set_rr("sp", 0xffff);
    gmb.memory.write_byte(0x101, 0x01);   // n = 0x01
    let expected_flags = [false, false, true, true];
    assert_eq!(gmb.cycle(), 16);                    // 16 cycles
    assert_eq!(gmb.cpu.get_rr("sp"), 0x0000);       // sp = 0x0000
    assert_eq!(super::check_flags(expected_flags, &gmb), true);
}

#[cfg(test)]
fn test_add_sp_n_from_book() {
    let mut gmb = super::get_test_gmb(0xe8);
    gmb.cpu.set_rr("sp", 0xfff8);
    gmb.memory.write_byte(0x101, 0x02);   // n = 0x02
    let expected_flags = [false, false, false, false];
    assert_eq!(gmb.cycle(), 16);                    // 16 cycles
    assert_eq!(gmb.cpu.get_rr("sp"), 0xfffa);       // sp = 0xfffa
    assert_eq!(super::check_flags(expected_flags, &gmb), true);
}

// ld hl, sp+n - opcode[0xf8]
#[test]
fn test_ld_hl_sp_n() {
    test_ld_hl_sp_n_from_book();
    test_ld_hl_sp_n_flag_h();
    test_ld_hl_sp_n_flag_c();
}

#[cfg(test)]
fn test_ld_hl_sp_n_flag_h() {
    let mut gmb = super::get_test_gmb(0xf8);
    gmb.cpu.set_rr("sp", 0x0fff);
    gmb.memory.write_byte(0x101, 0x01); // n = 0x01
    let expected_flags = [false, false, true, false];
    assert_eq!(gmb.cycle(), 12);                  // 16 cycles
    assert_eq!(gmb.cpu.get_rr("hl"), 0x1000);     // sp = 0x8001
    assert_eq!(super::check_flags(expected_flags, &gmb), true);
}

#[cfg(test)]
fn test_ld_hl_sp_n_flag_c() {
    let mut gmb = super::get_test_gmb(0xf8);
    gmb.cpu.set_rr("sp", 0xffff);
    gmb.memory.write_byte(0x101, 0x01);   // n = 0x01
    let expected_flags = [false, false, true, true];
    assert_eq!(gmb.cycle(), 12);                    // 16 cycles
    assert_eq!(gmb.cpu.get_rr("hl"), 0x0000);       // sp = 0x0000
    assert_eq!(super::check_flags(expected_flags, &gmb), true);
}

#[cfg(test)]
fn test_ld_hl_sp_n_from_book() { 
    let mut gmb = super::get_test_gmb(0xf8);
    gmb.cpu.set_rr("sp", 0xfff8);
    gmb.memory.write_byte(0x101, 0x02); 
    let expected_flags = [false, false, false, false];
    assert_eq!(gmb.cycle(), 12);                  
    assert_eq!(gmb.cpu.get_rr("hl"), 0xfffa);     
    assert_eq!(super::check_flags(expected_flags, &gmb), true);     
}