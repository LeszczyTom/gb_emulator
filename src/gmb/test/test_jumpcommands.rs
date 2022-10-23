#[test]
fn test_jump_nn() {
    // from book
    // JP 0x8000 ; PC <- 0x8000
    let mut gmb = super::get_test_gmb(0xc3);
    gmb.memory.write_byte(0x101, 0x00);
    gmb.memory.write_byte(0x102, 0x80);
    assert_eq!(gmb.cycle(), 16);
    assert_eq!(gmb.cpu.get_pc(), 0x8000);
}

#[test]
fn test_jump_cc_nn() {
    // from book
    // When Z = 1 and C = 0
    // JP NZ, 8000h ;Moves to next instruction after 3 cycles
    let mut gmb = super::get_test_gmb(0xc2);
    gmb.memory.write_byte(0x101, 0x00);
    gmb.memory.write_byte(0x102, 0x80);
    gmb.cpu.set_flag("z", true);
    assert_eq!(gmb.cycle(), 12);
    assert_eq!(gmb.cpu.get_pc(), 0x103);

    // JP Z, 8000h ; Jumps to address 8000h
    let mut gmb = super::get_test_gmb(0xca);
    gmb.memory.write_byte(0x101, 0x00);
    gmb.memory.write_byte(0x102, 0x80);
    gmb.cpu.set_flag("z", true);
    assert_eq!(gmb.cycle(), 16);
    assert_eq!(gmb.cpu.get_pc(), 0x8000);

    // JP C, 8000h ; Moves to next instruction after 3 cycles
    let mut gmb = super::get_test_gmb(0xda);
    gmb.memory.write_byte(0x101, 0x00);
    gmb.memory.write_byte(0x102, 0x80);
    gmb.cpu.set_flag("c", false);
    assert_eq!(gmb.cycle(), 12);
    assert_eq!(gmb.cpu.get_pc(), 0x103);

    // JP NC, 8000h ; Jumps to address 8000h.
    let mut gmb = super::get_test_gmb(0xd2);
    gmb.memory.write_byte(0x101, 0x00);
    gmb.memory.write_byte(0x102, 0x80);
    gmb.cpu.set_flag("c", false);
    assert_eq!(gmb.cycle(), 16);
    assert_eq!(gmb.cpu.get_pc(), 0x8000);
}

#[test]
fn test_jr_e() {
    // JR 0x79 ; PC <- PC + 127 
    let mut gmb = super::get_test_gmb(0x18);
    gmb.memory.write_byte(0x101, 0x79);
    assert_eq!(gmb.cycle(), 12);
    assert_eq!(gmb.cpu.get_pc(), 0x17b);
    
    // JR 0x80 ; PC <- PC - 128
    let mut gmb = super::get_test_gmb(0x18);
    gmb.memory.write_byte(0x101, 0x80);
    assert_eq!(gmb.cycle(), 12);
    assert_eq!(gmb.cpu.get_pc(), 0x82);
}

#[test]
fn test_jr_cc_e() {
    // JR NZ, 0x79 ; PC <- PC + 127
    let mut gmb = super::get_test_gmb(0x20);
    gmb.memory.write_byte(0x101, 0x79);
    gmb.cpu.set_flag("z", true);
    assert_eq!(gmb.cycle(), 8);
    assert_eq!(gmb.cpu.get_pc(), 0x102);

    // JR Z, 0x79 ; PC <- PC + 127
    let mut gmb = super::get_test_gmb(0x28);
    gmb.memory.write_byte(0x101, 0x79);
    gmb.cpu.set_flag("z", true);
    assert_eq!(gmb.cycle(), 12);
    assert_eq!(gmb.cpu.get_pc(), 0x17b);

    // JR NC, 0x80 ; PC <- PC - 128
    let mut gmb = super::get_test_gmb(0x30);
    gmb.memory.write_byte(0x101, 0x80);
    gmb.cpu.set_flag("c", true);
    assert_eq!(gmb.cycle(), 8);
    assert_eq!(gmb.cpu.get_pc(), 0x102);

    // JR C, 0x80 ; PC <- PC - 128
    let mut gmb = super::get_test_gmb(0x38);
    gmb.memory.write_byte(0x101, 0x80);
    gmb.cpu.set_flag("c", true);
    assert_eq!(gmb.cycle(), 12);
    assert_eq!(gmb.cpu.get_pc(), 0x82);
}

#[test]
fn test_call_nn() {
    // When PC = 8000h and SP = FFFEh; nn = 1234h
    // CALL 8000h ; SP <- SP - 2, (SP) <- PC + 3, PC <- 8000h

    let mut gmb = super::get_test_gmb(0xcd);
    gmb.cpu.set_pc(0x8000);
    gmb.memory.write_byte(0x8000, 0xcd);
    gmb.memory.write_byte(0x8001, 0x34);
    gmb.memory.write_byte(0x8002, 0x12);
    gmb.cpu.set_sp(0xfffe);
    assert_eq!(gmb.cycle(), 24);
    assert_eq!(gmb.cpu.get_pc(), 0x1234);
    assert_eq!(gmb.cpu.get_sp(), 0xfffc);
    assert_eq!(gmb.memory.read_byte(0xfffd), 0x03);
    assert_eq!(gmb.memory.read_byte(0xfffc), 0x80);
}

#[test]
fn test_call_cc_nn() {
    // When Z = 1, nn = 1234h
    // 7FFCh CALL NZ, 1234h ; PC <- PC + 3
    let mut gmb = super::get_test_gmb(0xc4);
    gmb.cpu.set_pc(0x7ffc);
    gmb.memory.write_byte(0x7ffc, 0xc4);
    gmb.memory.write_byte(0x7ffd, 0x34);    
    gmb.memory.write_byte(0x7ffe, 0x12);
    gmb.cpu.set_flag("z", true);
    assert_eq!(gmb.cycle(), 12);
    assert_eq!(gmb.cpu.get_pc(), 0x7fff);

    // 8000h CALL Z, 1234h ; Pushes 8003h to the stack,
    // 8003h                   and jumps to 1234h
    let mut gmb = super::get_test_gmb(0xcc);
    gmb.cpu.set_pc(0x8000);
    gmb.memory.write_byte(0x8000, 0xcc);
    gmb.memory.write_byte(0x8001, 0x34);
    gmb.memory.write_byte(0x8002, 0x12);
    gmb.cpu.set_sp(0xfffe);
    gmb.cpu.set_flag("z", true);
    assert_eq!(gmb.cycle(), 24);
    assert_eq!(gmb.cpu.get_pc(), 0x1234);
    assert_eq!(gmb.cpu.get_sp(), 0xfffc);
    assert_eq!(gmb.memory.read_byte(0xfffd), 0x03);
    assert_eq!(gmb.memory.read_byte(0xfffc), 0x80);
}

#[test]
fn test_ret() {
    let mut gmb = super::get_test_gmb(0xc9);
    gmb.cpu.set_sp(0xfffe);
    gmb.memory.write_byte(0xfffe, 0x12);
    gmb.memory.write_byte(0xffff, 0x34);
    assert_eq!(gmb.cycle(), 16);
    assert_eq!(gmb.cpu.get_pc(), 0x1234);
    assert_eq!(gmb.cpu.get_sp(), 0x0000);
    
    let mut gmb = super::get_test_gmb(0xc9);
    gmb.cpu.set_sp(0x0111);
    gmb.memory.write_byte(0x111, 0x12);
    gmb.memory.write_byte(0x112, 0x34);
    assert_eq!(gmb.cycle(), 16);
    assert_eq!(gmb.cpu.get_pc(), 0x1234);
    assert_eq!(gmb.cpu.get_sp(), 0x0113);
}

#[test]
fn test_ret_cc() {
    let mut gmb = super::get_test_gmb(0xc0);
    gmb.cpu.set_sp(0xfffe);
    gmb.memory.write_byte(0xfffe, 0x12);
    gmb.memory.write_byte(0xffff, 0x34);
    gmb.cpu.set_flag("z", true);
    assert_eq!(gmb.cycle(), 8);
    assert_eq!(gmb.cpu.get_pc(), 0x0102);

    let mut gmb = super::get_test_gmb(0xc0);
    gmb.cpu.set_sp(0xfffe);
    gmb.memory.write_byte(0xfffe, 0x12);
    gmb.memory.write_byte(0xffff, 0x34);
    gmb.cpu.set_flag("z", false);
    assert_eq!(gmb.cycle(), 20);
    assert_eq!(gmb.cpu.get_pc(), 0x1234);
    assert_eq!(gmb.cpu.get_sp(), 0x0000);
}

#[test]
fn test_reti() {
    let mut gmb = super::get_test_gmb(0xd9);
    gmb.cpu.set_sp(0xfffe);
    gmb.memory.write_byte(0xfffe, 0x12);
    gmb.memory.write_byte(0xffff, 0x34);
    gmb.cpu.clear_ime();
    assert_eq!(gmb.cycle(), 16);
    assert_eq!(gmb.cpu.get_pc(), 0x1234);
    assert_eq!(gmb.cpu.get_sp(), 0x0000);
    assert_eq!(gmb.cpu.get_ime(), true);

    let mut gmb = super::get_test_gmb(0xd9);
    gmb.cpu.set_sp(0xfffe);
    gmb.memory.write_byte(0xfffe, 0x12);
    gmb.memory.write_byte(0xffff, 0x34);
    gmb.cpu.set_ime();
    assert_eq!(gmb.cycle(), 16);
    assert_eq!(gmb.cpu.get_pc(), 0x1234);
    assert_eq!(gmb.cpu.get_sp(), 0x0000);
    assert_eq!(gmb.cpu.get_ime(), true);
}

#[test]
fn test_rst() {
    let mut gmb = super::get_test_gmb(0xc7);
    gmb.cpu.set_sp(0xfffe);
    assert_eq!(gmb.cycle(), 16);
    assert_eq!(gmb.cpu.get_pc(), 0x0000);
    assert_eq!(gmb.cpu.get_sp(), 0xfffc);
    assert_eq!(gmb.memory.read_byte(0xfffd), 0x01);
    assert_eq!(gmb.memory.read_byte(0xfffc), 0x01);
    
    let mut gmb = super::get_test_gmb(0xf7);
    gmb.cpu.set_sp(0xfffe);
    gmb.cpu.set_pc(0x8000);
    gmb.memory.write_byte(0x8000, 0xf7);
    assert_eq!(gmb.cycle(), 16);
    assert_eq!(gmb.cpu.get_pc(), 0x0030);
    assert_eq!(gmb.cpu.get_sp(), 0xfffc);
    assert_eq!(gmb.memory.read_byte(0xfffd), 0x01);
    assert_eq!(gmb.memory.read_byte(0xfffc), 0x80);

    let mut gmb = super::get_test_gmb(0xd7);
    gmb.cpu.set_sp(0xfffe);
    gmb.cpu.set_pc(0x8000);
    gmb.memory.write_byte(0x8000, 0xd7);
    assert_eq!(gmb.cycle(), 16);
    assert_eq!(gmb.cpu.get_pc(), 0x0010);
    assert_eq!(gmb.cpu.get_sp(), 0xfffc);
    assert_eq!(gmb.memory.read_byte(0xfffd), 0x01);
    assert_eq!(gmb.memory.read_byte(0xfffc), 0x80);
}