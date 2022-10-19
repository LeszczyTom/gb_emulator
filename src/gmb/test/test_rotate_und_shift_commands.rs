/// r: (&str, u8) = (register, value)
#[cfg(test)] 
fn test_r(r: (&str, u8), opcode: u8, flags: [bool; 4], expected_result: (&str, u8), expected_flags: [bool; 4], expected_cycle: u8) {
    let mut gmb = super::get_test_gmb(opcode);
    super::set_flags(flags, &mut gmb);
    match r.0 {
        "hl" => {
            gmb.cpu.set_rr("hl", 0x1234);
            gmb.memory.write_byte(0x1234, r.1);
            let expected_flags = expected_flags;
            assert_eq!(gmb.cycle(), expected_cycle);
            assert_eq!(gmb.memory.read_byte(gmb.cpu.get_rr("hl")), expected_result.1);
            assert_eq!(super::check_flags(expected_flags, &gmb), true);
        },
        "n" => {
            println!("n not implemented");
        },
        _ => {
            gmb.cpu.set_r(r.0, r.1);
            assert_eq!(gmb.cycle(), expected_cycle);                            
            assert_eq!(gmb.cpu.get_r(expected_result.0), expected_result.1);
            assert_eq!(super::check_flags(expected_flags, &gmb), true);
        }
    }
}

// rlca
#[test]
fn test_rlca() {
    // from book
    // When A = 0x85 and C = 0
    // RLCA ; A <- 0x0a, C <- 1, Z <- 0, H <- 0, N <- 0  ???
    test_r(("a", 0x85), 0x07, [false, false, false, false], ("a", 0x0b), [false, false, false, true], 4);
}

// rla
#[test]
fn test_rla() {
    // from book
    // When A = 0x95 and C = 1
    // RLA ; A <- 0x2b, C <- 1, Z <- 0, H <- 0, N <- 0
    test_r(("a", 0x95), 0x17, [false, false, false, true], ("a", 0x2b), [false, false, false, true], 4);
}

// rrca
#[test]
fn test_rrca() {
    // from book
    // When A = 0x3b and C = 0
    // RRCA ; A <- 0x9d, C <- 1, Z <- 0, H <- 0, N <- 0
    test_r(("a", 0x3b), 0x0f, [false, false, false, false], ("a", 0x9d), [false, false, false, true], 4);
}

// rra
#[test]
fn test_rra() {
    // from book
    // When A = 0x81 and C = 0
    // RRA ; A <- 0x40, C <- 1, Z <- 0, H <- 0, N <- 0
    test_r(("a", 0x81), 0x1f, [false, false, false, false], ("a", 0x40), [false, false, false, true], 4);
}

// rlc m
#[test]
fn test_rlc_m() {
    // from book
    // When B = 0x85, (hl) = 0 and C = 0
    // RLC B ; B <- 0b, C <- 1, Z <- 0, H <- 0, N <- 0
    let mut gmb = super::get_test_gmb(0xcb);
    gmb.memory.write_byte(0x101, 0x00);
    gmb.cpu.set_r("b", 0x85);
    super::set_flags([false, false, false, false], &mut gmb);
    assert_eq!(gmb.cycle(), 8);
    assert_eq!(gmb.cpu.get_r("b"), 0x0b);
    assert_eq!(super::check_flags([false, false, false, true], &gmb), true);
    
    // RLC (hl) ; (hl) <- 00, C <- 0, Z <- 1, H <- 0, N <- 0
    let mut gmb = super::get_test_gmb(0xcb);
    gmb.memory.write_byte(0x101, 0x06);
    gmb.cpu.set_rr("hl", 0x1234);
    gmb.memory.write_byte(0x1234, 0x00);
    super::set_flags([false, false, false, false], &mut gmb);
    assert_eq!(gmb.cycle(), 16);
    assert_eq!(gmb.memory.read_byte(gmb.cpu.get_rr("hl")), 0x00);
    assert_eq!(super::check_flags([true, false, false, false], &gmb), true);
}

// rl m
#[test]
fn test_rl_m() {
    // from book
    // When L = 0x80, (hl) = 0x11 and C = 0
    // RL L ; L <- 0, C <- 1, Z <- 1, H <- 0, N <- 0
    let mut gmb = super::get_test_gmb(0xcb);
    gmb.memory.write_byte(0x101, 0x15);
    gmb.cpu.set_r("l", 0x80);
    super::set_flags([false, false, false, false], &mut gmb);
    assert_eq!(gmb.cycle(), 8);
    assert_eq!(gmb.cpu.get_r("l"), 0);
    assert_eq!(super::check_flags([true, false, false, true], &gmb), true);
    
    // RLC (hl) ; (hl) <- 0x22, C <- 0, Z <- 0, H <- 0, N <- 0
    let mut gmb = super::get_test_gmb(0xcb);
    gmb.memory.write_byte(0x101, 0x16);
    gmb.cpu.set_rr("hl", 0x1234);
    gmb.memory.write_byte(0x1234, 0x11);
    super::set_flags([false, false, false, false], &mut gmb);
    assert_eq!(gmb.cycle(), 16);
    assert_eq!(gmb.memory.read_byte(gmb.cpu.get_rr("hl")), 0x22);
    assert_eq!(super::check_flags([false, false, false, false], &gmb), true);
}

// rrc m
#[test]
fn test_rrc_m() {
    // from book
    // When C = 0x1, (hl) = 0 and C = 0
    // RRC C ; C <- 0x80, C <- 1, Z <- 0, H <- 0, N <- 0
    let mut gmb = super::get_test_gmb(0xcb);
    gmb.memory.write_byte(0x101, 0x09);
    gmb.cpu.set_r("c", 0x1);
    super::set_flags([false, false, false, false], &mut gmb);
    assert_eq!(gmb.cycle(), 8);
    assert_eq!(gmb.cpu.get_r("c"), 0x80);
    assert_eq!(super::check_flags([false, false, false, true], &gmb), true);

    // RRC (hl) ; (hl) <- 0x00, C <- 0, Z <- 1, H <- 0, N <- 0
    let mut gmb = super::get_test_gmb(0xcb);
    gmb.memory.write_byte(0x101, 0x0e);
    gmb.cpu.set_rr("hl", 0x1234);
    gmb.memory.write_byte(0x1234, 0x00);
    super::set_flags([false, false, false, false], &mut gmb);
    assert_eq!(gmb.cycle(), 16);
    assert_eq!(gmb.memory.read_byte(gmb.cpu.get_rr("hl")), 0x00);
    assert_eq!(super::check_flags([true, false, false, false], &gmb), true);
}

// rr m
#[test]
fn test_rr_m() {
    // from book
    // When A = 0x1, (hl) = 0x8a and C = 0
    // RR A ; A <- 0x0, C <- 1, Z <- 1, H <- 0, N <- 0
    let mut gmb = super::get_test_gmb(0xcb);
    gmb.memory.write_byte(0x101, 0x1f);
    gmb.cpu.set_r("a", 0x1);
    super::set_flags([false, false, false, false], &mut gmb);
    assert_eq!(gmb.cycle(), 8);
    assert_eq!(gmb.cpu.get_r("a"), 0);
    assert_eq!(super::check_flags([true, false, false, true], &gmb), true);

    // RR (hl) ; (hl) <- 0x45, C <- 0, Z <- 0, H <- 0, N <- 0
    let mut gmb = super::get_test_gmb(0xcb);
    gmb.memory.write_byte(0x101, 0x1e);
    gmb.cpu.set_rr("hl", 0x1234);
    gmb.memory.write_byte(0x1234, 0x8a);
    super::set_flags([false, false, false, false], &mut gmb);
    assert_eq!(gmb.cycle(), 16);
    assert_eq!(gmb.memory.read_byte(gmb.cpu.get_rr("hl")), 0x45);
}

// sla m
#[test]
fn test_sla_m() {
    // from book
    // When D = 0x80, (hl) = 0xff and C = 0
    // SLA D ; D <- 0, C <- 1, Z <- 1, H <- 0, N <- 0
    let mut gmb = super::get_test_gmb(0xcb);
    gmb.memory.write_byte(0x101, 0x22);
    gmb.cpu.set_r("d", 0x80);
    super::set_flags([false, false, false, false], &mut gmb);
    assert_eq!(gmb.cycle(), 8);
    assert_eq!(gmb.cpu.get_r("d"), 0);
    assert_eq!(super::check_flags([true, false, false, true], &gmb), true);

    // SLA (hl) ; (hl) <- 0xfe, C <- 1, Z <- 0, H <- 0, N <- 0
    let mut gmb = super::get_test_gmb(0xcb);
    gmb.memory.write_byte(0x101, 0x26);
    gmb.cpu.set_rr("hl", 0x1234);
    gmb.memory.write_byte(0x1234, 0xff);
    super::set_flags([false, false, false, false], &mut gmb);
    assert_eq!(gmb.cycle(), 16);
    assert_eq!(gmb.memory.read_byte(gmb.cpu.get_rr("hl")), 0xfe);
    assert_eq!(super::check_flags([false, false, false, true], &gmb), true);
}

// sra m
#[test]
fn test_sra_m() {
    // from book
    // When A = 0x8a, (hl) = 0x01 and C = 0
    // SRA A ; A <- 0xC5, C <- 0, Z <- 0, H <- 0, N <- 0
    let mut gmb = super::get_test_gmb(0xcb);
    gmb.memory.write_byte(0x101, 0x2f);
    gmb.cpu.set_r("a", 0x8a);
    super::set_flags([false, false, false, false], &mut gmb);
    assert_eq!(gmb.cycle(), 8);
    assert_eq!(gmb.cpu.get_r("a"), 0xc5);
    assert_eq!(super::check_flags([false, false, false, false], &gmb), true);

    // SRA (hl) ; (hl) <- 0x00, C <- 1, Z <- 1, H <- 0, N <- 0
    let mut gmb = super::get_test_gmb(0xcb);
    gmb.memory.write_byte(0x101, 0x2e);
    gmb.cpu.set_rr("hl", 0x1234);
    gmb.memory.write_byte(0x1234, 0x01);
    super::set_flags([false, false, false, false], &mut gmb);
    assert_eq!(gmb.cycle(), 16);
    assert_eq!(gmb.memory.read_byte(gmb.cpu.get_rr("hl")), 0x00);
    assert_eq!(super::check_flags([true, false, false, true], &gmb), true);
}

// srl m
#[test]
fn test_srl_m() {
    // from book
    // When A = 0x1, (hl) = 0xff and C = 0
    // SRL A ; A <- 0, C <- 1, Z <- 1, H <- 0, N <- 0
    let mut gmb = super::get_test_gmb(0xcb);
    gmb.memory.write_byte(0x101, 0x3f);
    gmb.cpu.set_r("a", 0x1);
    super::set_flags([false, false, false, false], &mut gmb);
    assert_eq!(gmb.cycle(), 8);
    assert_eq!(gmb.cpu.get_r("a"), 0);
    assert_eq!(super::check_flags([true, false, false, true], &gmb), true);

    // SRL (hl) ; (hl) <- 0x7f, C <- 1, Z <- 0, H <- 0, N <- 0
    let mut gmb = super::get_test_gmb(0xcb);
    gmb.memory.write_byte(0x101, 0x3e);
    gmb.cpu.set_rr("hl", 0x1234);
    gmb.memory.write_byte(0x1234, 0xff);
    super::set_flags([false, false, false, false], &mut gmb);
    assert_eq!(gmb.cycle(), 16);
    assert_eq!(gmb.memory.read_byte(gmb.cpu.get_rr("hl")), 0x7f);
    assert_eq!(super::check_flags([false, false, false, true], &gmb), true); 
}

// swap m
#[test]
fn test_swap_m() {
    // from book
    // When A = 0x00, (hl) = 0xf0 
    // SWAP A ; A <- 0, C <- 0, Z <- 1, H <- 0, N <- 0
    let mut gmb = super::get_test_gmb(0xcb);
    gmb.memory.write_byte(0x101, 0x37);
    gmb.cpu.set_r("a", 0x00);
    super::set_flags([false, false, false, false], &mut gmb);
    assert_eq!(gmb.cycle(), 8);
    assert_eq!(gmb.cpu.get_r("a"), 0);
    assert_eq!(super::check_flags([true, false, false, false], &gmb), true);

    // SWAP (hl) ; (hl) <- 0x0f, C <- 0, Z <- 0, H <- 0, N <- 0
    let mut gmb = super::get_test_gmb(0xcb);
    gmb.memory.write_byte(0x101, 0x36);
    gmb.cpu.set_rr("hl", 0x1234);
    gmb.memory.write_byte(0x1234, 0xf0);
    super::set_flags([false, false, false, false], &mut gmb);
    assert_eq!(gmb.cycle(), 16);
    assert_eq!(gmb.memory.read_byte(gmb.cpu.get_rr("hl")), 0x0f);
    assert_eq!(super::check_flags([false, false, false, false], &gmb), true);
}