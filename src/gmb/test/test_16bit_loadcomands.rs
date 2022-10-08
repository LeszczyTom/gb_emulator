// ld rr,nn - opcode[0x01, 0x11, 0x21, 0x31]

#[cfg(test)]
fn test_ld_rr_nn(rr: &str) {
    let opcode = match rr {
        "bc" => 0x01,
        "de" => 0x11,
        "hl" => 0x21,
        "sp" => 0x31,
        _ => panic!("Invalid register pair"),
    };

    let mut gmb = super::get_test_gmb(opcode);
    gmb.memory.write_byte(0x101, 0x34);
    gmb.memory.write_byte(0x102, 0x12);
    assert_eq!(gmb.cycle(), 12);                    // 12 cycles
    assert_eq!(gmb.cpu.get_rr(rr), 0x1234);         // register pair set to 0x1234
}

#[test]
fn test_ld_bc_nn() {
    test_ld_rr_nn("bc")
}

#[test]
fn test_ld_de_nn() {
    test_ld_rr_nn("de")
}

#[test]
fn test_ld_hl_nn() {
    test_ld_rr_nn("hl")
}

#[test]
fn test_ld_sp_nn() {
    test_ld_rr_nn("sp")
}

// ld SP,HL - opcode 0xF9

#[test]
fn test_ld_sp_hl() {
    let mut gmb = super::get_test_gmb(0xf9);
    gmb.cpu.set_hl(0x1234);
    assert_eq!(gmb.cycle(), 8);             // 8 cycles
    assert_eq!(gmb.cpu.get_rr("sp"), 0x1234);   // SP = HL
}

// push rr - opcode[0xc5, 0xd5, 0xe5, 0xf5]

#[cfg(test)]
fn push_rr(rr: &str) {
    let opcode = match rr {
        "bc" => 0xc5,
        "de" => 0xd5,
        "hl" => 0xe5,
        "af" => 0xf5,
        _ => panic!("Invalid register pair"),
    };

    let mut gmb = super::get_test_gmb(opcode);
    gmb.cpu.set_rr("sp", 0xfffe);
    gmb.cpu.set_rr(rr, 0x1234);
    assert_eq!(gmb.cycle(), 16);                    // 16 cycles
    assert_eq!(gmb.cpu.get_rr("sp"), 0xfffc);       // sp is decremented by 2
    assert_eq!(gmb.memory.read_byte(0xfffd), 0x12); // high byte is pushed first 
    assert_eq!(gmb.memory.read_byte(0xfffc), 0x34); // low byte is pushed second
}

#[test]
fn test_push_bc() {
    push_rr("bc")
}

#[test]
fn test_push_de() {
    push_rr("de")
}

#[test]
fn test_push_hl() {
    push_rr("hl")
}

#[test]
fn test_push_af() {
    push_rr("af")
}


// pop rr - opcode[0xc1, 0xd1, 0xe1, 0xf1]

#[cfg(test)]
fn test_pop_rr(rr: &str) {
    let opcode = match rr {
        "bc" => 0xc1,
        "de" => 0xd1,
        "hl" => 0xe1,
        "af" => 0xf1,
        _ => panic!("Invalid register pair"),
    };

    let mut gmb = super::get_test_gmb(opcode);
    gmb.cpu.set_rr("sp", 0xfffc);
    gmb.memory.write_byte(0xfffd, 0x12);
    gmb.memory.write_byte(0xfffc, 0x34);
    assert_eq!(gmb.cycle(), 12);                    // 12 cycles
    assert_eq!(gmb.cpu.get_rr("sp"), 0xfffe);       // sp is incremented by 2
    assert_eq!(gmb.cpu.get_rr(rr), 0x1234);         // register pair set to 0x1234
}

#[test]
fn test_pop_bc() {
    test_pop_rr("bc")
}

#[test]
fn test_pop_de() {
    test_pop_rr("de")
}

#[test]
fn test_pop_hl() {
    test_pop_rr("hl")
}

#[test]
fn test_pop_af() {
    test_pop_rr("af")
}
