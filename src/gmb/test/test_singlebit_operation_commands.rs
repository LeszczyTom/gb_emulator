#[cfg(test)] 
fn test_r(r: (&str, u8), opcode: u8, flags: [bool; 4],expected_result: (&str, u8), expected_flags: [bool; 4], expected_cycle: u8) {
    let mut gmb = super::get_test_gmb(0xcb);
    gmb.memory.write_byte(0x101, opcode);
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
        _ => {
            gmb.cpu.set_r(r.0, r.1);
            assert_eq!(gmb.cycle(), expected_cycle);     
            assert_eq!(gmb.cpu.get_r(expected_result.0), expected_result.1);                       
            assert_eq!(super::check_flags(expected_flags, &gmb), true);
        }
    }
}

// bit n, r
#[test]
fn test_bit_r() {
    // from book
    // When A = 80h and L = EFh
    // BIT 7, A ; Z<- 0, H <- 1, N <- 0
    test_r(("a", 0x80), 0x7f, [false; 4], ("a", 0x80), [false, false, true, false], 8);

    // BIT 4, L ; Z«- 1, H <- 1, N <-0
    test_r(("l", 0xef), 0x6c, [false; 4], ("l", 0xef), [true, false, true, false], 8);
}

// bit n, (hl)
#[test]
fn test_bit_hl() {
    // from book
    // When (HL) = FEh
    // BIT 0, (HL) ; Z «- 1, H «- 1, N <- 0 
    test_r(("hl", 0xfe), 0x46, [false; 4], ("hl", 0xfe), [true, false, true, false], 12);

    // BIT 1, (HL) ; Z«-0, H<-1,N<-0
    test_r(("hl", 0xfe), 0x4e, [false; 4], ("hl", 0xfe), [false, false, true, false], 12);
}

// set n, r
#[test]
fn test_set_r() {
    // from book
    // When A = 80h and L = 3Bh,
    // SET 3, A ; A <- 0x84 ???
    test_r(("a", 0x80), 0xdf, [false; 4], ("a", 0x88), [false; 4], 8);

    // SET 4, L ; L <- 0x3B
    test_r(("l", 0x3b), 0xcc, [false; 4], ("l", 0x3b), [false; 4], 8);
}

// set n, (hl)
#[test]
fn test_set_hl() {
    // from book
    // When OOh is the memory contents specified by H and L
    // SET 3, (HL) ; (HL) 4- 04H ??
    test_r(("hl", 0x00), 0xde, [false; 4], ("hl", 0x08), [false; 4], 16);
}

// res n, r
#[test]
fn test_res_r() {
    // from book
    // When A = 80h and L = 3Bh,
    // RES 7, A ; A 4- OOh 
    test_r(("a", 0x80), 0xbf, [false; 4], ("a", 0x00), [false; 4], 8);

    // RES 1, L ; l_4-39h
    test_r(("l", 0x3b), 0x8d, [false; 4], ("l", 0x39), [false; 4], 8);
}

// res n, (hl)
#[test]
fn test_res_hl() {
    // from book
    // When OxFF is the memory contents specified by H and L
    // RES 3, (HL) ; (HL) <- F7h
    test_r(("hl", 0xff), 0x9e, [false; 4], ("hl", 0xf7), [false; 4], 16);
}