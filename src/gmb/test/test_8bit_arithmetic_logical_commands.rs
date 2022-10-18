/// r: (&str, u8) = (register, value)
/// r, n, and (HL) are used for operand s
#[cfg(test)] 
fn test_r_r(r: (&str, u8), s: (&str, u8), opcode: u8, flags: [bool; 4], expected_result: (&str, u8), expected_flags: [bool; 4], expected_cycle: u8) {
    //println!("Testing ADD A, {}", s.0);
    let mut gmb = super::get_test_gmb(opcode);
    super::set_flags(flags, &mut gmb);
    match s.0 {
        "hl" => {
            gmb.cpu.set_rr("hl", 0x1234);
            gmb.cpu.set_r(r.0, r.1);
            gmb.memory.write_byte(0x1234, s.1);
            let expected_flags = expected_flags;
            assert_eq!(gmb.cycle(), expected_cycle);
            assert_eq!(gmb.cpu.get_r(expected_result.0), expected_result.1);
            assert_eq!(super::check_flags(expected_flags, &gmb), true);
        },
        "n" => {
            gmb.cpu.set_r(r.0, r.1);
            gmb.memory.write_byte(0x101, s.1);
            let expected_flags = expected_flags;
            assert_eq!(gmb.cycle(), expected_cycle);
            assert_eq!(gmb.cpu.get_r(expected_result.0), expected_result.1);
            assert_eq!(super::check_flags(expected_flags, &gmb), true);
        },
        _ => {
            gmb.cpu.set_r(r.0, r.1);
            gmb.cpu.set_r(s.0, s.1);
            assert_eq!(gmb.cycle(), expected_cycle);                            
            assert_eq!(gmb.cpu.get_r(expected_result.0), expected_result.1);
            assert_eq!(super::check_flags(expected_flags, &gmb), true);
        }
    }
}

// add a,s - opcode[0x80-0x87 + 0xc6]
#[test]
fn test_add_a() {
    let tuples: Vec<(&str, u8)> = [("b", 0x80), ("c", 0x81), ("d", 0x82), ("e", 0x83),
                  ("h", 0x84), ("l", 0x85), ("hl", 0x86), ("a", 0x87), ("n", 0xc6)].to_vec();
    
    for tuple in tuples {
        let cycle = match tuple.0 {
            "hl" => 8,
            "n" => 8,
            _ => 4
        };
            
        // When A = 0x01, S = 0x01, C = 0
        // ADD A, S ; A <- 0x02, Z <- 0, H <- 0, C <- 0
        test_r_r(("a", 0x01), 
            (tuple.0, 0x01), 
            tuple.1,
            [false, false, false, false],
            ("a", 0x02),
            [false, false, false, false],
            cycle);

        // When A = 0x80, S = 0x80, C = 0
        // ADD A, S ; A <- 0x00, Z <- 1, H <- 0, C <- 1
        test_r_r(("a", 0x80), 
            (tuple.0, 0x80), 
            tuple.1,
            [false, false, false, false],
            ("a", 0),
            [true, false, false, true],
            cycle);

        // When A = 0x08, S = 0x08, C = 0
        // ADD A, S ; A <- 0x10, Z <- 0, H <- 1, C <- 0
        test_r_r(("a", 0x08), 
            (tuple.0, 0x08), 
            tuple.1,
            [false, false, false, false],
            ("a", 0x10),
            [false, false, true, false],
            cycle);
    }
}

// adc a,s - opcode[0x88-0x8f + 0xce]
#[test]
fn test_adc_a_s() {
    let tuples: Vec<(&str, u8)> = [("b", 0x88), ("c", 0x89), ("d", 0x8a), ("e", 0x8b),
                  ("h", 0x8c), ("l", 0x8d), ("hl", 0x8e), ("a", 0x8f), ("n", 0xce)].to_vec();

    // from book
    // When A = 0xe1, E = 0x0f, (hl) = 0x1e, C = 1
    // ADC A, E ; A <- 0xf1, Z <- 0, H <- 1, C <- 0
    test_r_r(("a", 0xe1), 
        ("e", 0x0f), 
        0x8b,
        [false, false, false, true],
        ("a", 0xf1),
        [false, false, true, false],
        4);

    // ADC A, 0x3b ; A <- 0x1d, Z <- 0, H <- 0, C <- 1
    test_r_r(("a", 0xe1), 
        ("n", 0x3b), 
        0xce,
        [false, false, false, true],
        ("a", 0x1d),
        [false, false, false, true],
        8);

    // ADC A, (HL) ; A <- 0x00, Z <- 1, H <- 1, C <- 1
    test_r_r(("a", 0xe1), 
        ("hl", 0x1e), 
        0x8e,
        [false, false, false, true],
        ("a", 0),
        [true, false, true, true],
        8);

    for tuple in tuples {
        let cycle = match tuple.0 {
            "hl" => 8,
            "n" => 8,
            _ => 4
        };

        // When A = 0x01, S = 0x01, C = 0
        // ADC A, S ; A <- 0x02, Z <- 0, H <- 0, C <- 0
        test_r_r(("a", 0x01), 
            (tuple.0, 0x01), 
            tuple.1,
            [false, false, false, false],
            ("a", 0x02),
            [false, false, false, false],
            cycle);
        
        // When A = 0x01, S = 0x01, C = 1
        // ADC A, S ; A <- 0x03, Z <- 0, H <- 0, C <- 0
        test_r_r(("a", 0x01), 
            (tuple.0, 0x01), 
            tuple.1,
            [false, false, false, true],
            ("a", 0x03),
            [false, false, false, false],
            cycle);

        // When A = 0x80, S = 0x80, C = 0
        // ADC A, S ; A <- 0x00, Z <- 1, H <- 0, C <- 1
        test_r_r(("a", 0x80), 
            (tuple.0, 0x80), 
            tuple.1,
            [false, false, false, false],
            ("a", 0),
            [true, false, false, true],
            cycle);
        
        // When A = 0x80, S = 0x80, C = 1
        // ADC A, S ; A <- 0x01, Z <- 0, H <- 0, C <- 1
        test_r_r(("a", 0x80), 
            (tuple.0, 0x80), 
            tuple.1,
            [false, false, false, true],
            ("a", 0x01),
            [false, false, false, true],
            cycle);

        // When A = 0x08, S = 0x08, C = 0
        // ADC A, S ; A <- 0x10, Z <- 0, H <- 1, C <- 0
        test_r_r(("a", 0x08), 
            (tuple.0, 0x08), 
            tuple.1,
            [false, false, false, false],
            ("a", 0x10),
            [false, false, true, false],
            cycle);

        // When A = 0x08, S = 0x08, C = 1
        // ADC A, S ; A <- 0x11, Z <- 0, H <- 1, C <- 0
        test_r_r(("a", 0x08), 
            (tuple.0, 0x08), 
            tuple.1,
            [false, false, false, true],
            ("a", 0x11),
            [false, false, true, false],
            cycle);
    }
}

// sub s - opcode[0x90-0x97 + 0xd6]
#[test]
fn test_sub_s() {
    /*let tuples: Vec<(&str, u8)> = [("b", 0x90), ("c", 0x91), ("d", 0x92), ("e", 0x93),
                  ("h", 0x94), ("l", 0x95), ("hl", 0x96), ("a", 0x97), ("n", 0xd6)].to_vec();*/

    // from book
    // When A = 0x3e, E = 0x3e, (hl) = 0x40
    // SUB E ; A <- 0x00, Z <- 1, N <- 1, H <- 0, C <- 0
    test_r_r(("a", 0x3e), 
        ("e", 0x3e), 
        0x93,
        [false, false, false, false],
        ("a", 0x00),
        [true, true, false, false],
        4);

    // SUB 0x0f ; A <- 0x2f, Z <- 0, N <- 1, H <- 1, C <- 0
    test_r_r(("a", 0x3e), 
        ("n", 0x0f), 
        0xd6,
        [false, false, false, false],
        ("a", 0x2f),
        [false, true, true, false],
        8);
    
    // SUB (HL) ; A <- 0xfe, Z <- 0, N <- 1, H <- 0, C <- 1
    test_r_r(("a", 0x3e), 
        ("hl", 0x40), 
        0x96,
        [false, false, false, false],
        ("a", 0xfe),
        [false, true, false, true],
        8);

    //TODO: test all cases (carry, half carry, zero)
}

// sbc s - opcode[0x98-0x9f + 0xde]
#[test]
fn test_sbc_s() {
    /*let tuples: Vec<(&str, u8)> = [("b", 0x98), ("c", 0x99), ("d", 0x9a), ("e", 0x9b),
                  ("h", 0x9c), ("l", 0x9d), ("hl", 0x9e), ("a", 0x9f), ("n", 0xde)].to_vec();*/

    // from book
    // When A = 0x3b, (hl) = 0x4f, H = 2a, C = 1
    // SBC A, H ; A <- 0x10, Z <- 0, N <- 1, H <- 0, C <- 0
    test_r_r(("a", 0x3b), 
        ("h", 0x2a), 
        0x9c,
        [false, false, false, true],
        ("a", 0x10),
        [false, true, false, false],
        4);

    // SBC A, 0x3a ; A <- 0x00, Z <- 1, N <- 1, H <- 0, C <- 0
    test_r_r(("a", 0x3b), 
        ("n", 0x3a), 
        0xde,
        [false, false, false, true],
        ("a", 0x00),
        [true, true, false, false],
        8);

    // SBC A, (HL) ; A <- 0xeb, Z <- 0, N <- 1, H <- 1, C <- 1
    test_r_r(("a", 0x3b), 
        ("hl", 0x4f), 
        0x9e,
        [false, false, false, true],
        ("a", 0xeb),
        [false, true, true, true],
        8);

    //TODO: test all cases (carry, half carry, zero)
}

// and s - opcode[0xa0-0xa7 + 0xe6]
#[test]
fn test_and_s() {
    let tuples = [("b", 0xa0), ("c", 0xa1), ("d", 0xa2), ("e", 0xa3),
                  ("h", 0xa4), ("l", 0xa5), ("hl", 0xa6), ("a", 0xa7), ("n", 0xe6)];
    // from book
    // When A = 0x5a, l = 0x3f, (hl) = 0x00
    // AND l ; A <- 0x1a, Z <- 0, N <- 0, H <- 1, C <- 0
    test_r_r(("a", 0x5a), 
        ("l", 0x3f), 
        0xa5,
        [false, false, false, false],
        ("a", 0x1a),
        [false, false, true, false],
        4);

    // AND 0x38 ; A <- 0x18, Z <- 0, N <- 0, H <- 1, C <- 0
    test_r_r(("a", 0x5a), 
        ("n", 0x38), 
        0xe6,
        [false, false, false, false],
        ("a", 0x18),
        [false, false, true, false],
        8);

    // AND (HL) ; A <- 0x00, Z <- 1, N <- 0, H <- 1, C <- 0
    test_r_r(("a", 0x5a), 
        ("hl", 0x00), 
        0xa6,
        [false, false, false, false],
        ("a", 0x00),
        [true, false, true, false],
        8);

    for tuple in tuples {
        match tuple.0 {
            "n" | "hl" => {
                // When A = 0x01, S = 0x01, (hl) = 0x01
                // AND S ; A <- 0x01, Z <- 0, N <- 0, H <- 1, C <- 0
                test_r_r(("a", 0x01), 
                (tuple.0, 0x01), 
                tuple.1,
                [false, false, false, false],
                ("a", 0x01),
                [false, false, true, false],
                8);
                
                // When A = 0x01, S = 0x10, (hl) = 0x10
                // AND S ; A <- 0x1a, Z <- 0, N <- 0, H <- 1, C <- 0
                test_r_r(("a", 0x01), 
                (tuple.0, 0x10), 
                tuple.1,
                [false, false, false, false],
                ("a", 0x00),
                [true, false, true, false],
                8);
            },
            _ => {
                if tuple.0 == "a" {
                    // TODO
                    continue;
                }
                // When A = 0x01, S = 0x01, (hl) = 0x01
                // AND S ; A <- 0x1a, Z <- 0, N <- 0, H <- 1, C <- 0
                test_r_r(("a", 0x5a), 
                (tuple.0, 0x3f), 
                tuple.1,
                [false, false, false, false],
                ("a", 0x1a),
                [false, false, true, false],
                4);

                // When A = 0x01, S = 0x10, (hl) = 0x10
                // AND S ; A <- 0x1a, Z <- 0, N <- 0, H <- 1, C <- 0
                test_r_r(("a", 0x01), 
                (tuple.0, 0x10), 
                tuple.1,
                [false, false, false, false],
                ("a", 0x00),
                [true, false, true, false],
                4);
            }
        }
    }
}

// xor s - opcode[0xa8-0xaf + 0xee]
#[test]
fn test_xor_s() {
    let tuples = [("b", 0xa8), ("c", 0xa9), ("d", 0xaa), ("e", 0xab),
                  ("h", 0xac), ("l", 0xad), ("hl", 0xae), ("a", 0xaf), ("n", 0xee)];
    // from book
    // When A = 0xff, (hl) = 0x8a
    // XOR A ; A <- 0x00, Z <- 1, N <- 0, H <- 0, C <- 0
    test_r_r(("a", 0xff), 
        ("a", 0xff), 
        0xaf,
        [false, false, false, false],
        ("a", 0x00),
        [true, false, false, false],
        4);

    // XOR 0x0f ; A <- 0xf0, Z <- 0, N <- 0, H <- 0, C <- 0
    test_r_r(("a", 0xff), 
        ("n", 0x0f), 
        0xee,
        [false, false, false, false],
        ("a", 0xf0),
        [false, false, false, false],
        8);

    // XOR (HL) ; A <- 0x75, Z <- 0, N <- 0, H <- 0, C <- 0
    test_r_r(("a", 0xff), 
        ("hl", 0x8a), 
        0xae,
        [false, false, false, false],
        ("a", 0x75),
        [false, false, false, false],
        8);

    for tuple in tuples {
        match tuple.0 {
            "hl" | "n" => {
                // When A = 0x01, S = 0x01, (hl) = 0x01
                // XOR S ; A <- 0x00, Z <- 1, N <- 0, H <- 0, C <- 0
                test_r_r(("a", 0x01), 
                (tuple.0, 0x01), 
                tuple.1,
                [false, false, false, false],
                ("a", 0x00),
                [true, false, false, false],
                8);
                
                // When A = 0x01, S = 0x10, (hl) = 0x10
                // XOR S ; A <- 0x1a, Z <- 0, N <- 0, H <- 1, C <- 0
                test_r_r(("a", 0x01), 
                (tuple.0, 0x10), 
                tuple.1,
                [false, false, false, false],
                ("a", 0x11),
                [false, false, false, false],
                8);
            },
            _ => {
                if tuple.0 == "a" {
                    // TODO
                    continue;
                }
                // When A = 0x01, S = 0x01, (hl) = 0x01
                // XOR S ; A <- 0x00, Z <- 1, N <- 0, H <- 0, C <- 0
                test_r_r(("a", 0x01), 
                (tuple.0, 0x01), 
                tuple.1,
                [false, false, false, false],
                ("a", 0x00),
                [true, false, false, false],
                4);
                
                // When A = 0x01, S = 0x10, (hl) = 0x10
                // XOR S ; A <- 0x1a, Z <- 0, N <- 0, H <- 1, C <- 0
                test_r_r(("a", 0x01), 
                (tuple.0, 0x10), 
                tuple.1,
                [false, false, false, false],
                ("a", 0x11),
                [false, false, false, false],
                4); 
            }
        }
    }
}

// or s - opcode[0xb0-0xb7 + 0xf6]
#[test]
fn test_or_s() {
    let tuples = [("b", 0xb0), ("c", 0xb1), ("d", 0xb2), ("e", 0xb3),
                  ("h", 0xb4), ("l", 0xb5), ("hl", 0xb6), ("a", 0xb7), ("n", 0xf6)];

    // from book
    // When A = 0x5a, (hl) = 0x0f
    // OR A ; A <- 0x5a, Z <- 0, N <- 0, H <- 0, C <- 0
    test_r_r(("a", 0x5a), 
        ("a", 0x5a), 
        0xb7,
        [false, false, false, false],
        ("a", 0x5a),
        [false, false, false, false],
        4);

    // OR 0x3 ; A <- 0x5b, Z <- 0, N <- 0, H <- 0, C <- 0
    test_r_r(("a", 0x5a), 
        ("n", 0x3), 
        0xf6,
        [false, false, false, false],
        ("a", 0x5b),
        [false, false, false, false],
        8);

    // OR (HL) ; A <- 0x5f, Z <- 0, N <- 0, H <- 0, C <- 0
    test_r_r(("a", 0x5a), 
        ("hl", 0x0f), 
        0xb6,
        [false, false, false, false],
        ("a", 0x5f),
        [false, false, false, false],
        8);

    for tuple in tuples {
        match tuple.0 {
            "hl" | "n" => {
                // When A = 0x01, S = 0x01, (hl) = 0x01
                // OR S ; A <- 0x01, Z <- 0, N <- 0, H <- 0, C <- 0
                test_r_r(("a", 0x01), 
                (tuple.0, 0x01), 
                tuple.1,
                [false, false, false, false],
                ("a", 0x01),
                [false, false, false, false],
                8);
                
                // When A = 0x01, S = 0x10, (hl) = 0x10
                // OR S ; A <- 0x11, Z <- 0, N <- 0, H <- 0, C <- 0
                test_r_r(("a", 0x01), 
                (tuple.0, 0x10), 
                tuple.1,
                [false, false, false, false],
                ("a", 0x11),
                [false, false, false, false],
                8);
            },
            _ => {
                if tuple.0 == "a" {
                    // TODO
                    continue;
                }
                // When A = 0x01, S = 0x01
                // OR S ; A <- 0x01, Z <- 0, N <- 0, H <- 0, C <- 0
                test_r_r(("a", 0x01),
                (tuple.0, 0x01),
                tuple.1,
                [false, false, false, false],
                ("a", 0x01),
                [false, false, false, false],
                4);

                // When A = 0x00, S = 0x00
                // OR S ; A <- 0x00, Z <- 1, N <- 0, H <- 0, C <- 0
                test_r_r(("a", 0x00),
                (tuple.0, 0x00),
                tuple.1,
                [false, false, false, false],
                ("a", 0x00),
                [true, false, false, false],
                4);
            }
        }
    }
}