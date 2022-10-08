// ld r,r - opcode[0x40 - 0x7f] without x6 & xe
#[test]
fn test_ld_r_r() {
    let opcodes: [u8; 49] = [0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x47, 
                            0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4f, 
                            0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x57,
                            0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5f, 
                            0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x67, 
                            0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6f, 
                            0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7f];

    for (i, opcode) in opcodes.iter().enumerate() {
        let r1 = match i {
            0..=6 => "b",
            7..=13 => "c",
            14..=20 => "d",
            21..=27 => "e",
            28..=34 => "h",
            35..=41 => "l",
            42..=48 => "a",
            _ => panic!("Invalid register"),
        };
        
        let r2 = match i % 7 {
            0 => "b",
            1 => "c",
            2 => "d",
            3 => "e",
            4 => "h",
            5 => "l",
            6 => "a",
            _ => panic!("Invalid register"),
        };

        let mut gmb = super::get_test_gmb(*opcode);
        gmb.cpu.set_r(r2, 0x12);
        assert_eq!(gmb.cycle(), 4);             // 4 cycles
        assert_eq!(gmb.cpu.get_r(r1), 0x12);    // r1 set to r2
    }
}

// ld r,n - opcode[0x06, 0x16, 0x26, 0x0e, 0x1e, 0x2e, 0x3e]
#[test]
fn test_ld_r_n() {
    let tuples = [(0x06, "b"), (0x16, "d"), (0x26, "h"), (0x0e, "c"), (0x1e, "e"), (0x2e, "l"), (0x3e, "a")];
    for tuple in tuples {
        let mut gmb = super::get_test_gmb(tuple.0);
        gmb.memory.write_byte(0x101, 0x12);
        assert_eq!(gmb.cycle(), 8);                 // 8 cycles
        assert_eq!(gmb.cpu.get_r(tuple.1), 0x12);   // register r = 0x12
    }
}

// ld r,(hl) - opcode[0x46, 0x4e, 0x56, 0x5e, 0x66, 0x6e, 0x7e]
#[test]
fn test_ld_r_hl() {
    let tuples = [(0x46, "b"), (0x4e, "c"), (0x56, "d"), (0x5e, "e"), (0x66, "h"), (0x6e, "l"), (0x7e, "a")];
    for tuple in tuples {
        let mut gmb = super::get_test_gmb(tuple.0);
        gmb.cpu.set_hl(0x1234);
        gmb.memory.write_byte(0x1234, 0x12);
        assert_eq!(gmb.cycle(), 8);                 // 8 cycles
        assert_eq!(gmb.cpu.get_r(tuple.1), 0x12);   // register r = 0x12
    }
}

// ld (hl),r - opcode[0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x77]
#[test]
fn test_ld_hl_r() {
    let tuples = [(0x70, "b"), (0x71, "c"), (0x72, "d"), (0x73, "e"), (0x74, "h"), (0x75, "l"), (0x77, "a")];
    for tuple in tuples {
        let mut gmb = super::get_test_gmb(tuple.0);
        gmb.cpu.set_hl(0x1234);
        gmb.cpu.set_r(tuple.1, 0x12);
        assert_eq!(gmb.cycle(), 8);                                     // 8 cycles
        assert_eq!(gmb.memory.read_byte(gmb.cpu.get_rr("hl")), 0x12);   // memory[hl] = 0x12
    }
}

// ld (hl),n - opcode[0x36]
#[test]
fn test_ld_hl_n() {
    let mut gmb = super::get_test_gmb(0x36);
    gmb.cpu.set_hl(0x1234);
    gmb.memory.write_byte(0x101, 0x12);
    assert_eq!(gmb.cycle(), 12);                                    // 12 cycles
    assert_eq!(gmb.memory.read_byte(gmb.cpu.get_rr("hl")), 0x12);   // memory[hl] = 0x12
}

// ld a,(bc) - opcode[0x0a]
#[test]
fn test_ld_a_bc() {
    let mut gmb = super::get_test_gmb(0x0a);
    gmb.cpu.set_rr("bc", 0x1234);
    gmb.memory.write_byte(0x1234, 0x12);
    assert_eq!(gmb.cycle(), 8);                 // 8 cycles
    assert_eq!(gmb.cpu.get_r("a"), 0x12);       // register a = 0x12
}

// ld a,(de) - opcode[0x1a]
#[test]
fn test_ld_a_de() {
    let mut gmb = super::get_test_gmb(0x1a);
    gmb.cpu.set_rr("de", 0x1234);
    gmb.memory.write_byte(0x1234, 0x12);
    assert_eq!(gmb.cycle(), 8);                 // 8 cycles
    assert_eq!(gmb.cpu.get_r("a"), 0x12);       // register a = 0x12
}

// ld a,(nn) - opcode[0xfa]
#[test]
fn test_ld_a_nn() {
    let mut gmb = super::get_test_gmb(0xfa);
    gmb.memory.write_byte(0x101, 0x34);
    gmb.memory.write_byte(0x102, 0x12);
    gmb.memory.write_byte(0x1234, 0x12);
    assert_eq!(gmb.cycle(), 16);                // 16 cycles
    assert_eq!(gmb.cpu.get_r("a"), 0x12);       // register a = 0x12 when reading from memory[0x1234]
}

// ld (bc),a - opcode[0x02]
#[test]
fn test_ld_bc_a() {
    let mut gmb = super::get_test_gmb(0x02);
    gmb.cpu.set_rr("bc", 0x1234);
    gmb.cpu.set_r("a", 0x12);
    assert_eq!(gmb.cycle(), 8);                                     // 8 cycles
    assert_eq!(gmb.memory.read_byte(gmb.cpu.get_rr("bc")), 0x12);   // memory[bc] = 0x12
}

// ld (de),a - opcode[0x12]
#[test]
fn test_ld_de_a() {
    let mut gmb = super::get_test_gmb(0x12);
    gmb.cpu.set_rr("de", 0x1234);
    gmb.cpu.set_r("a", 0x12);
    assert_eq!(gmb.cycle(), 8);                                     // 8 cycles
    assert_eq!(gmb.memory.read_byte(gmb.cpu.get_rr("de")), 0x12);   // memory[de] = 0x12
}

// ld (nn),a - opcode[0xea]
#[test]
fn test_ld_nn_a() {
    let mut gmb = super::get_test_gmb(0xea);
    gmb.memory.write_byte(0x101, 0x34);
    gmb.memory.write_byte(0x102, 0x12);
    gmb.cpu.set_r("a", 0x12);
    assert_eq!(gmb.cycle(), 16);                    // 16 cycles
    assert_eq!(gmb.memory.read_byte(0x1234), 0x12); // memory[0x1234] = 0x12
}

// ld a,(FF00+n) - opcode[0xf0]
#[test]
fn test_ld_a_ff00_n() {
    let mut gmb = super::get_test_gmb(0xf0);
    gmb.memory.write_byte(0x101, 0x12);
    gmb.memory.write_byte(0xff12, 0x34);
    assert_eq!(gmb.cycle(), 12);            // 12 cycles
    assert_eq!(gmb.cpu.get_r("a"), 0x34);   // register a = 0x34
}

// ld (FF00+n),A - opcode[0xe0]
#[test]
fn test_ld_ff00_n_a() {
    let mut gmb = super::get_test_gmb(0xe0);
    gmb.memory.write_byte(0x101, 0x34);
    gmb.cpu.set_r("a", 0x12);
    assert_eq!(gmb.cycle(), 12);                            // 12 cycles
    assert_eq!(gmb.memory.read_byte(0xff00 + 0x34), 0x12);  // memory[0xff00 + n] = 0x12
}

// ld a,(FF00+c) - opcode[0xf2]
#[test]
fn test_ld_a_ff00_c() {
    let mut gmb = super::get_test_gmb(0xf2);
    gmb.cpu.set_r("c", 0x12);
    gmb.memory.write_byte(0xff12, 0x34);
    assert_eq!(gmb.cycle(), 8);             // 8 cycles
    assert_eq!(gmb.cpu.get_r("a"), 0x34);   // register a = 0x34
}

// ld (FF00+c),a - opcode[0xe2]
#[test]
fn test_ld_ff00_c_a() {
    let mut gmb = super::get_test_gmb(0xe2);
    gmb.cpu.set_r("c", 0x12);
    gmb.cpu.set_r("a", 0x34);
    assert_eq!(gmb.cycle(), 8);                             // 8 cycles
    assert_eq!(gmb.memory.read_byte(0xff00 + 0x12), 0x34);  // memory[0xff00 + c] = 0x34
}

// ldi (hl),a - opcode[0x22]
#[test]
fn test_ldi_hl_a() {
    let mut gmb = super::get_test_gmb(0x22);
    gmb.cpu.set_rr("hl", 0x1234);
    gmb.cpu.set_r("a", 0x12);
    assert_eq!(gmb.cycle(), 8);                                                     // 8 cycles
    assert_eq!(gmb.memory.read_byte(gmb.cpu.get_rr("hl").wrapping_sub(1)), 0x12);   // memory[hl - 1] = 0x12
    assert_eq!(gmb.cpu.get_rr("hl"), 0x1235);                                       // hl = 0x1235
}

// ldi a,(hl) - opcode[0x2a]
#[test]
fn test_ldi_a_hl() {
    let mut gmb = super::get_test_gmb(0x2a);
    gmb.cpu.set_rr("hl", 0x1234);
    gmb.memory.write_byte(0x1234, 0x12);
    assert_eq!(gmb.cycle(), 8);                 // 8 cycles
    assert_eq!(gmb.cpu.get_r("a"), 0x12);       // register a = 0x12
    assert_eq!(gmb.cpu.get_rr("hl"), 0x1235);   // hl = 0x1235
}

// ldd (hl),a - opcode[0x32]
#[test]
fn test_ldd_hl_a() {
    let mut gmb = super::get_test_gmb(0x32);
    gmb.cpu.set_rr("hl", 0x1234);
    gmb.cpu.set_r("a", 0x12);
    assert_eq!(gmb.cycle(), 8);                                                     // 8 cycles
    assert_eq!(gmb.memory.read_byte(gmb.cpu.get_rr("hl").wrapping_add(1)), 0x12);   // memory[hl + 1] = 0x12
    assert_eq!(gmb.cpu.get_rr("hl"), 0x1233);                                       // hl = 0x1233
}

// ldd a,(hl) - opcode[0x3a]
#[test]
fn test_ldd_a_hl() {
    let mut gmb = super::get_test_gmb(0x3a);
    gmb.cpu.set_rr("hl", 0x1234);
    gmb.memory.write_byte(0x1234, 0x12);
    assert_eq!(gmb.cycle(), 8);                 // 8 cycles
    assert_eq!(gmb.cpu.get_r("a"), 0x12);       // register a = 0x12
    assert_eq!(gmb.cpu.get_rr("hl"), 0x1233);   // hl = 0x1233
}