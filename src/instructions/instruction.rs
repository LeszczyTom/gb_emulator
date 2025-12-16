use super::block_0::*;
use super::block_1::*;
use super::block_2::*;
use super::block_3::*;
use super::block_cb::*;
use crate::core::Core;
use Parameters::*;

#[derive(Debug, PartialEq)]
pub enum R8 {
    B,
    C,
    D,
    E,
    H,
    L,
    HL,
    A,
}

#[derive(Debug, PartialEq)]
pub enum R16 {
    BC,
    DE,
    HL,
    SP,
}

#[derive(Debug, PartialEq)]
pub enum R16stk {
    BC,
    DE,
    HL,
    AF,
}

#[derive(Debug, PartialEq)]
pub enum R16mem {
    BC,
    DE,
    HLi,
    HLd,
}

#[derive(Debug, PartialEq)]
pub enum Cond {
    NZ,
    Z,
    NC,
    C,
}

#[derive(Debug, PartialEq)]
pub enum Parameters {
    R8(R8),
    R16(R16),
    R16stk(R16stk),
    R16mem(R16mem),
    Cond(Cond),
    B3(u8),
    Tgt3(u16),
}

pub type Instruction = fn(&mut Core, &(Option<Parameters>, Option<Parameters>));
pub type InstructionParameters = (Option<Parameters>, Option<Parameters>);

pub fn invalid(_core: &mut Core, _p: &(Option<Parameters>, Option<Parameters>)) {
    panic!();
}

const INSTRUCTIONS_SIZE: usize = 256;

#[rustfmt::skip]
pub const INSTRUCTIONS: [Instruction; INSTRUCTIONS_SIZE] = [
    nop,           ld_r16_imm16, ld_r16mem_a,   inc_r16,  inc_r8,          dec_r8,      ld_r8_imm8, rlca,     
    ld_imm16_sp,   add_hl_r16,   ld_a_r16mem,   dec_r16,  inc_r8,          dec_r8,      ld_r8_imm8, rrca,
    stop,          ld_r16_imm16, ld_r16mem_a,   inc_r16,  inc_r8,          dec_r8,      ld_r8_imm8, rla,      
    jr_imm8,       add_hl_r16,   ld_a_r16mem,   dec_r16,  inc_r8,          dec_r8,      ld_r8_imm8, rra,
    jr_cond_imm8,  ld_r16_imm16, ld_r16mem_a,   inc_r16,  inc_r8,          dec_r8,      ld_r8_imm8, daa,      
    jr_cond_imm8,  add_hl_r16,   ld_a_r16mem,   dec_r16,  inc_r8,          dec_r8,      ld_r8_imm8, cpl,
    jr_cond_imm8,  ld_r16_imm16, ld_r16mem_a,   inc_r16,  inc_r8,          dec_r8,      ld_r8_imm8, scf,      
    jr_cond_imm8,  add_hl_r16,   ld_a_r16mem,   dec_r16,  inc_r8,          dec_r8,      ld_r8_imm8, ccf,
    ld_r8_r8,      ld_r8_r8,     ld_r8_r8,      ld_r8_r8, ld_r8_r8,        ld_r8_r8,    ld_r8_r8,   ld_r8_r8, 
    ld_r8_r8,      ld_r8_r8,     ld_r8_r8,      ld_r8_r8, ld_r8_r8,        ld_r8_r8,    ld_r8_r8,   ld_r8_r8,
    ld_r8_r8,      ld_r8_r8,     ld_r8_r8,      ld_r8_r8, ld_r8_r8,        ld_r8_r8,    ld_r8_r8,   ld_r8_r8, 
    ld_r8_r8,      ld_r8_r8,     ld_r8_r8,      ld_r8_r8, ld_r8_r8,        ld_r8_r8,    ld_r8_r8,   ld_r8_r8,
    ld_r8_r8,      ld_r8_r8,     ld_r8_r8,      ld_r8_r8, ld_r8_r8,        ld_r8_r8,    ld_r8_r8,   ld_r8_r8, 
    ld_r8_r8,      ld_r8_r8,     ld_r8_r8,      ld_r8_r8, ld_r8_r8,        ld_r8_r8,    ld_r8_r8,   ld_r8_r8,
    ld_r8_r8,      ld_r8_r8,     ld_r8_r8,      ld_r8_r8, ld_r8_r8,        ld_r8_r8,    halt,       ld_r8_r8, 
    ld_r8_r8,      ld_r8_r8,     ld_r8_r8,      ld_r8_r8, ld_r8_r8,        ld_r8_r8,    ld_r8_r8,   ld_r8_r8,
    add_a_r8,      add_a_r8,     add_a_r8,      add_a_r8, add_a_r8,        add_a_r8,    add_a_r8,   add_a_r8, 
    adc_a_r8,      adc_a_r8,     adc_a_r8,      adc_a_r8, adc_a_r8,        adc_a_r8,    adc_a_r8,   adc_a_r8, 
    sub_a_r8,      sub_a_r8,     sub_a_r8,      sub_a_r8, sub_a_r8,        sub_a_r8,    sub_a_r8,   sub_a_r8, 
    sbc_a_r8,      sbc_a_r8,     sbc_a_r8,      sbc_a_r8, sbc_a_r8,        sbc_a_r8,    sbc_a_r8,   sbc_a_r8, 
    and_a_r8,      and_a_r8,     and_a_r8,      and_a_r8, and_a_r8,        and_a_r8,    and_a_r8,   and_a_r8, 
    xor_a_r8,      xor_a_r8,     xor_a_r8,      xor_a_r8, xor_a_r8,        xor_a_r8,    xor_a_r8,   xor_a_r8, 
    or_a_r8,       or_a_r8,      or_a_r8,       or_a_r8,  or_a_r8,         or_a_r8,     or_a_r8,    or_a_r8,  
    cp_a_r8,       cp_a_r8,      cp_a_r8,       cp_a_r8,  cp_a_r8,         cp_a_r8,     cp_a_r8,    cp_a_r8, 
    ret_cond,      pop_r16stk,   jp_cond_imm16, jp_imm16, call_cond_imm16, push_r16stk, add_a_imm8, rst_tgt3, 
    ret_cond,      ret,          jp_cond_imm16, prefix,   call_cond_imm16, call_imm16,  adc_a_imm8, rst_tgt3,
    ret_cond,      pop_r16stk,   jp_cond_imm16, invalid,  call_cond_imm16, push_r16stk, sub_a_imm8, rst_tgt3, 
    ret_cond,      reti,         jp_cond_imm16, invalid,  call_cond_imm16, invalid,     sbc_a_imm8, rst_tgt3,
    ldh_imm8_a,    pop_r16stk,   ldh_c_a,       invalid,  invalid,         push_r16stk, and_a_imm8, rst_tgt3, 
    add_sp_imm8,   jp_hl,        ld_imm16_a,    invalid,  invalid,         invalid,     xor_a_imm8, rst_tgt3,
    ldh_a_imm8,    pop_r16stk,   ldh_a_c,       di,       invalid,         push_r16stk, or_a_imm8,  rst_tgt3, 
    ld_hl_sp_imm8, ld_sp_hl,     ld_a_imm16,    ei,       invalid,         invalid,     cp_a_imm8,  rst_tgt3
];

#[rustfmt::skip]
pub const INSTRUCTIONS_TIMING: [u8; INSTRUCTIONS_SIZE] = [
    1,3,2,2,1,1,2,1,5,2,2,2,1,1,2,1,
    0,3,2,2,1,1,2,1,3,2,2,2,1,1,2,1,
    2,3,2,2,1,1,2,1,2,2,2,2,1,1,2,1,
    2,3,2,2,3,3,3,1,2,2,2,2,1,1,2,1,
    1,1,1,1,1,1,2,1,1,1,1,1,1,1,2,1,
    1,1,1,1,1,1,2,1,1,1,1,1,1,1,2,1,
    1,1,1,1,1,1,2,1,1,1,1,1,1,1,2,1,
    2,2,2,2,2,2,0,2,1,1,1,1,1,1,2,1,
    1,1,1,1,1,1,2,1,1,1,1,1,1,1,2,1,
    1,1,1,1,1,1,2,1,1,1,1,1,1,1,2,1,
    1,1,1,1,1,1,2,1,1,1,1,1,1,1,2,1,
    1,1,1,1,1,1,2,1,1,1,1,1,1,1,2,1,
    2,3,3,4,3,4,2,4,2,4,3,0,3,6,2,4,
    2,3,3,0,3,4,2,4,2,4,3,0,3,0,2,4,
    3,3,2,0,0,4,2,4,4,1,4,0,0,0,2,4,
    3,3,2,1,0,4,2,4,3,2,4,1,0,0,2,4
];

#[rustfmt::skip]
pub const INSTRUCTIONS_PARAMETERS: [InstructionParameters; INSTRUCTIONS_SIZE] = [
    (None, None),                        (Some(R16(R16::BC)), None),          (Some(R16mem(R16mem::BC)), None),    (Some(R16(R16::BC)), None),          (Some(R8(R8::B)), None),             (Some(R8(R8::B)), None),             (Some(R8(R8::B)), None),             (None, None),
    (None, None),                        (Some(R16(R16::BC)), None),          (Some(R16mem(R16mem::BC)), None),    (Some(R16(R16::BC)), None),          (Some(R8(R8::C)), None),             (Some(R8(R8::C)), None),             (Some(R8(R8::C)), None),             (None, None),
    (None, None),                        (Some(R16(R16::DE)), None),          (Some(R16mem(R16mem::DE)), None),    (Some(R16(R16::DE)), None),          (Some(R8(R8::D)), None),             (Some(R8(R8::D)), None),             (Some(R8(R8::D)), None),             (None, None),
    (None, None),                        (Some(R16(R16::DE)), None),          (Some(R16mem(R16mem::DE)), None),    (Some(R16(R16::DE)), None),          (Some(R8(R8::E)), None),             (Some(R8(R8::E)), None),             (Some(R8(R8::E)), None),             (None, None),
    (Some(Cond(Cond::NZ)), None),        (Some(R16(R16::HL)), None),          (Some(R16mem(R16mem::HLi)), None),   (Some(R16(R16::HL)), None),          (Some(R8(R8::H)), None),             (Some(R8(R8::H)), None),             (Some(R8(R8::H)), None),             (None, None),
    (Some(Cond(Cond::Z)), None),         (Some(R16(R16::HL)), None),          (Some(R16mem(R16mem::HLi)), None),   (Some(R16(R16::HL)), None),          (Some(R8(R8::L)), None),             (Some(R8(R8::L)), None),             (Some(R8(R8::L)), None),             (None, None),
    (Some(Cond(Cond::NC)), None),        (Some(R16(R16::SP)), None),          (Some(R16mem(R16mem::HLd)), None),   (Some(R16(R16::SP)), None),          (Some(R8(R8::HL)), None),            (Some(R8(R8::HL)), None),            (Some(R8(R8::HL)), None),            (None, None),
    (Some(Cond(Cond::C)), None),         (Some(R16(R16::SP)), None),          (Some(R16mem(R16mem::HLd)), None),   (Some(R16(R16::SP)), None),          (Some(R8(R8::A)), None),             (Some(R8(R8::A)), None),             (Some(R8(R8::A)), None),             (None, None),
    (Some(R8(R8::B)), Some(R8(R8::B))),  (Some(R8(R8::B)), Some(R8(R8::C))),  (Some(R8(R8::B)), Some(R8(R8::D))),  (Some(R8(R8::B)), Some(R8(R8::E))),  (Some(R8(R8::B)), Some(R8(R8::H))),  (Some(R8(R8::B)), Some(R8(R8::L))),  (Some(R8(R8::B)), Some(R8(R8::HL))), (Some(R8(R8::B)), Some(R8(R8::A))),
    (Some(R8(R8::C)), Some(R8(R8::B))),  (Some(R8(R8::C)), Some(R8(R8::C))),  (Some(R8(R8::C)), Some(R8(R8::D))),  (Some(R8(R8::C)), Some(R8(R8::E))),  (Some(R8(R8::C)), Some(R8(R8::H))),  (Some(R8(R8::C)), Some(R8(R8::L))),  (Some(R8(R8::C)), Some(R8(R8::HL))), (Some(R8(R8::C)), Some(R8(R8::A))),
    (Some(R8(R8::D)), Some(R8(R8::B))),  (Some(R8(R8::D)), Some(R8(R8::C))),  (Some(R8(R8::D)), Some(R8(R8::D))),  (Some(R8(R8::D)), Some(R8(R8::E))),  (Some(R8(R8::D)), Some(R8(R8::H))),  (Some(R8(R8::D)), Some(R8(R8::L))),  (Some(R8(R8::D)), Some(R8(R8::HL))), (Some(R8(R8::D)), Some(R8(R8::A))),
    (Some(R8(R8::E)), Some(R8(R8::B))),  (Some(R8(R8::E)), Some(R8(R8::C))),  (Some(R8(R8::E)), Some(R8(R8::D))),  (Some(R8(R8::E)), Some(R8(R8::E))),  (Some(R8(R8::E)), Some(R8(R8::H))),  (Some(R8(R8::E)), Some(R8(R8::L))),  (Some(R8(R8::E)), Some(R8(R8::HL))), (Some(R8(R8::E)), Some(R8(R8::A))),
    (Some(R8(R8::H)), Some(R8(R8::B))),  (Some(R8(R8::H)), Some(R8(R8::C))),  (Some(R8(R8::H)), Some(R8(R8::D))),  (Some(R8(R8::H)), Some(R8(R8::E))),  (Some(R8(R8::H)), Some(R8(R8::H))),  (Some(R8(R8::H)), Some(R8(R8::L))),  (Some(R8(R8::H)), Some(R8(R8::HL))), (Some(R8(R8::H)), Some(R8(R8::A))),
    (Some(R8(R8::L)), Some(R8(R8::B))),  (Some(R8(R8::L)), Some(R8(R8::C))),  (Some(R8(R8::L)), Some(R8(R8::D))),  (Some(R8(R8::L)), Some(R8(R8::E))),  (Some(R8(R8::L)), Some(R8(R8::H))),  (Some(R8(R8::L)), Some(R8(R8::L))),  (Some(R8(R8::L)), Some(R8(R8::HL))), (Some(R8(R8::L)), Some(R8(R8::A))),
    (Some(R8(R8::HL)), Some(R8(R8::B))), (Some(R8(R8::HL)), Some(R8(R8::C))), (Some(R8(R8::HL)), Some(R8(R8::D))), (Some(R8(R8::HL)), Some(R8(R8::E))), (Some(R8(R8::HL)), Some(R8(R8::H))), (Some(R8(R8::HL)), Some(R8(R8::L))), (None, Some(R8(R8::HL))),            (Some(R8(R8::HL)), Some(R8(R8::A))),
    (Some(R8(R8::A)), Some(R8(R8::B))),  (Some(R8(R8::A)), Some(R8(R8::C))),  (Some(R8(R8::A)), Some(R8(R8::D))),  (Some(R8(R8::A)), Some(R8(R8::E))),  (Some(R8(R8::A)), Some(R8(R8::H))),  (Some(R8(R8::A)), Some(R8(R8::L))),  (Some(R8(R8::A)), Some(R8(R8::HL))), (Some(R8(R8::A)), Some(R8(R8::A))),
    (Some(R8(R8::B)), None),             (Some(R8(R8::C)), None),             (Some(R8(R8::D)), None),             (Some(R8(R8::E)), None),             (Some(R8(R8::H)), None),             (Some(R8(R8::L)), None),             (Some(R8(R8::HL)), None),            (Some(R8(R8::A)), None),
    (Some(R8(R8::B)), None),             (Some(R8(R8::C)), None),             (Some(R8(R8::D)), None),             (Some(R8(R8::E)), None),             (Some(R8(R8::H)), None),             (Some(R8(R8::L)), None),             (Some(R8(R8::HL)), None),            (Some(R8(R8::A)), None),
    (Some(R8(R8::B)), None),             (Some(R8(R8::C)), None),             (Some(R8(R8::D)), None),             (Some(R8(R8::E)), None),             (Some(R8(R8::H)), None),             (Some(R8(R8::L)), None),             (Some(R8(R8::HL)), None),            (Some(R8(R8::A)), None),
    (Some(R8(R8::B)), None),             (Some(R8(R8::C)), None),             (Some(R8(R8::D)), None),             (Some(R8(R8::E)), None),             (Some(R8(R8::H)), None),             (Some(R8(R8::L)), None),             (Some(R8(R8::HL)), None),            (Some(R8(R8::A)), None),
    (Some(R8(R8::B)), None),             (Some(R8(R8::C)), None),             (Some(R8(R8::D)), None),             (Some(R8(R8::E)), None),             (Some(R8(R8::H)), None),             (Some(R8(R8::L)), None),             (Some(R8(R8::HL)), None),            (Some(R8(R8::A)), None),
    (Some(R8(R8::B)), None),             (Some(R8(R8::C)), None),             (Some(R8(R8::D)), None),             (Some(R8(R8::E)), None),             (Some(R8(R8::H)), None),             (Some(R8(R8::L)), None),             (Some(R8(R8::HL)), None),            (Some(R8(R8::A)), None),
    (Some(R8(R8::B)), None),             (Some(R8(R8::C)), None),             (Some(R8(R8::D)), None),             (Some(R8(R8::E)), None),             (Some(R8(R8::H)), None),             (Some(R8(R8::L)), None),             (Some(R8(R8::HL)), None),            (Some(R8(R8::A)), None),
    (Some(R8(R8::B)), None),             (Some(R8(R8::C)), None),             (Some(R8(R8::D)), None),             (Some(R8(R8::E)), None),             (Some(R8(R8::H)), None),             (Some(R8(R8::L)), None),             (Some(R8(R8::HL)), None),            (Some(R8(R8::A)), None),
    (Some(Cond(Cond::NZ)), None),        (Some(R16stk(R16stk::BC)), None),    (Some(Cond(Cond::NZ)), None),        (None, None),                        (Some(Cond(Cond::NZ)), None),        (Some(R16stk(R16stk::BC)), None),    (None, None),                        (Some(Tgt3(0x00)), None),
    (Some(Cond(Cond::Z)), None),         (None, None),                        (Some(Cond(Cond::Z)), None),         (None, None),                        (Some(Cond(Cond::Z)), None),         (None, None),                        (None, None),                        (Some(Tgt3(0x08)), None),
    (Some(Cond(Cond::NC)), None),        (Some(R16stk(R16stk::DE)), None),    (Some(Cond(Cond::NC)), None),        (None, None),                        (Some(Cond(Cond::NC)), None),        (Some(R16stk(R16stk::DE)), None),    (None, None),                        (Some(Tgt3(0x10)), None),
    (Some(Cond(Cond::C)), None),         (None, None),                        (Some(Cond(Cond::C)), None),         (None, None),                        (Some(Cond(Cond::C)), None),         (None, None),                        (None, None),                        (Some(Tgt3(0x18)), None),
    (None, None),                        (Some(R16stk(R16stk::HL)), None),    (None, None),                        (None, None),                        (None, None),                        (Some(R16stk(R16stk::HL)), None),    (None, None),                        (Some(Tgt3(0x20)), None),
    (None, None),                        (None, None),                        (None, None),                        (None, None),                        (None, None),                        (None, None),                        (None, None),                        (Some(Tgt3(0x28)), None),
    (None, None),                        (Some(R16stk(R16stk::AF)), None),    (None, None),                        (None, None),                        (None, None),                        (Some(R16stk(R16stk::AF)), None),    (None, None),                        (Some(Tgt3(0x30)), None),
    (None, None),                        (None, None),                        (None, None),                        (None, None),                        (None, None),                        (None, None),                        (None, None),                        (Some(Tgt3(0x38)), None),
];

#[rustfmt::skip]
pub const CB_INSTRUCTIONS: [Instruction; INSTRUCTIONS_SIZE] = [
    rlc_r8,    rlc_r8,    rlc_r8,    rlc_r8,    rlc_r8,    rlc_r8,    rlc_r8,    rlc_r8,
    rrc_r8,    rrc_r8,    rrc_r8,    rrc_r8,    rrc_r8,    rrc_r8,    rrc_r8,    rrc_r8,
    rl_r8,     rl_r8,     rl_r8,     rl_r8,     rl_r8,     rl_r8,     rl_r8,     rl_r8, 
    rr_r8,     rr_r8,     rr_r8,     rr_r8,     rr_r8,     rr_r8,     rr_r8,     rr_r8, 
    sla_r8,    sla_r8,    sla_r8,    sla_r8,    sla_r8,    sla_r8,    sla_r8,    sla_r8, 
    sra_r8,    sra_r8,    sra_r8,    sra_r8,    sra_r8,    sra_r8,    sra_r8,    sra_r8, 
    swap_r8,   swap_r8,   swap_r8,   swap_r8,   swap_r8,   swap_r8,   swap_r8,   swap_r8, 
    srl_r8,    srl_r8,    srl_r8,    srl_r8,    srl_r8,    srl_r8,    srl_r8,    srl_r8, 
    bit_b3_r8, bit_b3_r8, bit_b3_r8, bit_b3_r8, bit_b3_r8, bit_b3_r8, bit_b3_r8, bit_b3_r8, 
    bit_b3_r8, bit_b3_r8, bit_b3_r8, bit_b3_r8, bit_b3_r8, bit_b3_r8, bit_b3_r8, bit_b3_r8, 
    bit_b3_r8, bit_b3_r8, bit_b3_r8, bit_b3_r8, bit_b3_r8, bit_b3_r8, bit_b3_r8, bit_b3_r8, 
    bit_b3_r8, bit_b3_r8, bit_b3_r8, bit_b3_r8, bit_b3_r8, bit_b3_r8, bit_b3_r8, bit_b3_r8, 
    bit_b3_r8, bit_b3_r8, bit_b3_r8, bit_b3_r8, bit_b3_r8, bit_b3_r8, bit_b3_r8, bit_b3_r8, 
    bit_b3_r8, bit_b3_r8, bit_b3_r8, bit_b3_r8, bit_b3_r8, bit_b3_r8, bit_b3_r8, bit_b3_r8, 
    bit_b3_r8, bit_b3_r8, bit_b3_r8, bit_b3_r8, bit_b3_r8, bit_b3_r8, bit_b3_r8, bit_b3_r8, 
    bit_b3_r8, bit_b3_r8, bit_b3_r8, bit_b3_r8, bit_b3_r8, bit_b3_r8, bit_b3_r8, bit_b3_r8, 
    res_b3_r8, res_b3_r8, res_b3_r8, res_b3_r8, res_b3_r8, res_b3_r8, res_b3_r8, res_b3_r8, 
    res_b3_r8, res_b3_r8, res_b3_r8, res_b3_r8, res_b3_r8, res_b3_r8, res_b3_r8, res_b3_r8, 
    res_b3_r8, res_b3_r8, res_b3_r8, res_b3_r8, res_b3_r8, res_b3_r8, res_b3_r8, res_b3_r8, 
    res_b3_r8, res_b3_r8, res_b3_r8, res_b3_r8, res_b3_r8, res_b3_r8, res_b3_r8, res_b3_r8, 
    res_b3_r8, res_b3_r8, res_b3_r8, res_b3_r8, res_b3_r8, res_b3_r8, res_b3_r8, res_b3_r8, 
    res_b3_r8, res_b3_r8, res_b3_r8, res_b3_r8, res_b3_r8, res_b3_r8, res_b3_r8, res_b3_r8, 
    res_b3_r8, res_b3_r8, res_b3_r8, res_b3_r8, res_b3_r8, res_b3_r8, res_b3_r8, res_b3_r8, 
    res_b3_r8, res_b3_r8, res_b3_r8, res_b3_r8, res_b3_r8, res_b3_r8, res_b3_r8, res_b3_r8, 
    set_b3_r8, set_b3_r8, set_b3_r8, set_b3_r8, set_b3_r8, set_b3_r8, set_b3_r8, set_b3_r8,
    set_b3_r8, set_b3_r8, set_b3_r8, set_b3_r8, set_b3_r8, set_b3_r8, set_b3_r8, set_b3_r8,
    set_b3_r8, set_b3_r8, set_b3_r8, set_b3_r8, set_b3_r8, set_b3_r8, set_b3_r8, set_b3_r8,
    set_b3_r8, set_b3_r8, set_b3_r8, set_b3_r8, set_b3_r8, set_b3_r8, set_b3_r8, set_b3_r8,
    set_b3_r8, set_b3_r8, set_b3_r8, set_b3_r8, set_b3_r8, set_b3_r8, set_b3_r8, set_b3_r8,
    set_b3_r8, set_b3_r8, set_b3_r8, set_b3_r8, set_b3_r8, set_b3_r8, set_b3_r8, set_b3_r8,
    set_b3_r8, set_b3_r8, set_b3_r8, set_b3_r8, set_b3_r8, set_b3_r8, set_b3_r8, set_b3_r8,
    set_b3_r8, set_b3_r8, set_b3_r8, set_b3_r8, set_b3_r8, set_b3_r8, set_b3_r8, set_b3_r8,
];

#[rustfmt::skip]
pub const CB_INSTRUCTIONS_TIMING: [u8; INSTRUCTIONS_SIZE] = [
    2,2,2,2,2,2,4,2,2,2,2,2,2,2,4,2,
    2,2,2,2,2,2,4,2,2,2,2,2,2,2,4,2,
    2,2,2,2,2,2,4,2,2,2,2,2,2,2,4,2,
    2,2,2,2,2,2,4,2,2,2,2,2,2,2,4,2,
    2,2,2,2,2,2,3,2,2,2,2,2,2,2,3,2,
    2,2,2,2,2,2,3,2,2,2,2,2,2,2,3,2,
    2,2,2,2,2,2,3,2,2,2,2,2,2,2,3,2,
    2,2,2,2,2,2,3,2,2,2,2,2,2,2,3,2,
    2,2,2,2,2,2,4,2,2,2,2,2,2,2,4,2,
    2,2,2,2,2,2,4,2,2,2,2,2,2,2,4,2,
    2,2,2,2,2,2,4,2,2,2,2,2,2,2,4,2,
    2,2,2,2,2,2,4,2,2,2,2,2,2,2,4,2,
    2,2,2,2,2,2,4,2,2,2,2,2,2,2,4,2,
    2,2,2,2,2,2,4,2,2,2,2,2,2,2,4,2,
    2,2,2,2,2,2,4,2,2,2,2,2,2,2,4,2,
    2,2,2,2,2,2,4,2,2,2,2,2,2,2,4,2,
];

#[rustfmt::skip]
pub const CB_INSTRUCTIONS_PARAMETERS: [InstructionParameters; INSTRUCTIONS_SIZE] = [
    (Some(R8(R8::B)), None),        (Some(R8(R8::C)), None),        (Some(R8(R8::D)), None),        (Some(R8(R8::E)), None),        (Some(R8(R8::H)), None),        (Some(R8(R8::L)), None),        (Some(R8(R8::HL)), None),        (Some(R8(R8::A)), None),
    (Some(R8(R8::B)), None),        (Some(R8(R8::C)), None),        (Some(R8(R8::D)), None),        (Some(R8(R8::E)), None),        (Some(R8(R8::H)), None),        (Some(R8(R8::L)), None),        (Some(R8(R8::HL)), None),        (Some(R8(R8::A)), None),
    (Some(R8(R8::B)), None),        (Some(R8(R8::C)), None),        (Some(R8(R8::D)), None),        (Some(R8(R8::E)), None),        (Some(R8(R8::H)), None),        (Some(R8(R8::L)), None),        (Some(R8(R8::HL)), None),        (Some(R8(R8::A)), None),
    (Some(R8(R8::B)), None),        (Some(R8(R8::C)), None),        (Some(R8(R8::D)), None),        (Some(R8(R8::E)), None),        (Some(R8(R8::H)), None),        (Some(R8(R8::L)), None),        (Some(R8(R8::HL)), None),        (Some(R8(R8::A)), None),
    (Some(R8(R8::B)), None),        (Some(R8(R8::C)), None),        (Some(R8(R8::D)), None),        (Some(R8(R8::E)), None),        (Some(R8(R8::H)), None),        (Some(R8(R8::L)), None),        (Some(R8(R8::HL)), None),        (Some(R8(R8::A)), None),
    (Some(R8(R8::B)), None),        (Some(R8(R8::C)), None),        (Some(R8(R8::D)), None),        (Some(R8(R8::E)), None),        (Some(R8(R8::H)), None),        (Some(R8(R8::L)), None),        (Some(R8(R8::HL)), None),        (Some(R8(R8::A)), None),
    (Some(R8(R8::B)), None),        (Some(R8(R8::C)), None),        (Some(R8(R8::D)), None),        (Some(R8(R8::E)), None),        (Some(R8(R8::H)), None),        (Some(R8(R8::L)), None),        (Some(R8(R8::HL)), None),        (Some(R8(R8::A)), None),
    (Some(R8(R8::B)), None),        (Some(R8(R8::C)), None),        (Some(R8(R8::D)), None),        (Some(R8(R8::E)), None),        (Some(R8(R8::H)), None),        (Some(R8(R8::L)), None),        (Some(R8(R8::HL)), None),        (Some(R8(R8::A)), None),
    (Some(B3(0)), Some(R8(R8::B))), (Some(B3(0)), Some(R8(R8::C))), (Some(B3(0)), Some(R8(R8::D))), (Some(B3(0)), Some(R8(R8::E))), (Some(B3(0)), Some(R8(R8::H))), (Some(B3(0)), Some(R8(R8::L))), (Some(B3(0)), Some(R8(R8::HL))), (Some(B3(0)), Some(R8(R8::A))), 
    (Some(B3(1)), Some(R8(R8::B))), (Some(B3(1)), Some(R8(R8::C))), (Some(B3(1)), Some(R8(R8::D))), (Some(B3(1)), Some(R8(R8::E))), (Some(B3(1)), Some(R8(R8::H))), (Some(B3(1)), Some(R8(R8::L))), (Some(B3(1)), Some(R8(R8::HL))), (Some(B3(1)), Some(R8(R8::A))), 
    (Some(B3(2)), Some(R8(R8::B))), (Some(B3(2)), Some(R8(R8::C))), (Some(B3(2)), Some(R8(R8::D))), (Some(B3(2)), Some(R8(R8::E))), (Some(B3(2)), Some(R8(R8::H))), (Some(B3(2)), Some(R8(R8::L))), (Some(B3(2)), Some(R8(R8::HL))), (Some(B3(2)), Some(R8(R8::A))), 
    (Some(B3(3)), Some(R8(R8::B))), (Some(B3(3)), Some(R8(R8::C))), (Some(B3(3)), Some(R8(R8::D))), (Some(B3(3)), Some(R8(R8::E))), (Some(B3(3)), Some(R8(R8::H))), (Some(B3(3)), Some(R8(R8::L))), (Some(B3(3)), Some(R8(R8::HL))), (Some(B3(3)), Some(R8(R8::A))), 
    (Some(B3(4)), Some(R8(R8::B))), (Some(B3(4)), Some(R8(R8::C))), (Some(B3(4)), Some(R8(R8::D))), (Some(B3(4)), Some(R8(R8::E))), (Some(B3(4)), Some(R8(R8::H))), (Some(B3(4)), Some(R8(R8::L))), (Some(B3(4)), Some(R8(R8::HL))), (Some(B3(4)), Some(R8(R8::A))), 
    (Some(B3(5)), Some(R8(R8::B))), (Some(B3(5)), Some(R8(R8::C))), (Some(B3(5)), Some(R8(R8::D))), (Some(B3(5)), Some(R8(R8::E))), (Some(B3(5)), Some(R8(R8::H))), (Some(B3(5)), Some(R8(R8::L))), (Some(B3(5)), Some(R8(R8::HL))), (Some(B3(5)), Some(R8(R8::A))), 
    (Some(B3(6)), Some(R8(R8::B))), (Some(B3(6)), Some(R8(R8::C))), (Some(B3(6)), Some(R8(R8::D))), (Some(B3(6)), Some(R8(R8::E))), (Some(B3(6)), Some(R8(R8::H))), (Some(B3(6)), Some(R8(R8::L))), (Some(B3(6)), Some(R8(R8::HL))), (Some(B3(6)), Some(R8(R8::A))), 
    (Some(B3(7)), Some(R8(R8::B))), (Some(B3(7)), Some(R8(R8::C))), (Some(B3(7)), Some(R8(R8::D))), (Some(B3(7)), Some(R8(R8::E))), (Some(B3(7)), Some(R8(R8::H))), (Some(B3(7)), Some(R8(R8::L))), (Some(B3(7)), Some(R8(R8::HL))), (Some(B3(7)), Some(R8(R8::A))), 
    (Some(B3(0)), Some(R8(R8::B))), (Some(B3(0)), Some(R8(R8::C))), (Some(B3(0)), Some(R8(R8::D))), (Some(B3(0)), Some(R8(R8::E))), (Some(B3(0)), Some(R8(R8::H))), (Some(B3(0)), Some(R8(R8::L))), (Some(B3(0)), Some(R8(R8::HL))), (Some(B3(0)), Some(R8(R8::A))), 
    (Some(B3(1)), Some(R8(R8::B))), (Some(B3(1)), Some(R8(R8::C))), (Some(B3(1)), Some(R8(R8::D))), (Some(B3(1)), Some(R8(R8::E))), (Some(B3(1)), Some(R8(R8::H))), (Some(B3(1)), Some(R8(R8::L))), (Some(B3(1)), Some(R8(R8::HL))), (Some(B3(1)), Some(R8(R8::A))), 
    (Some(B3(2)), Some(R8(R8::B))), (Some(B3(2)), Some(R8(R8::C))), (Some(B3(2)), Some(R8(R8::D))), (Some(B3(2)), Some(R8(R8::E))), (Some(B3(2)), Some(R8(R8::H))), (Some(B3(2)), Some(R8(R8::L))), (Some(B3(2)), Some(R8(R8::HL))), (Some(B3(2)), Some(R8(R8::A))), 
    (Some(B3(3)), Some(R8(R8::B))), (Some(B3(3)), Some(R8(R8::C))), (Some(B3(3)), Some(R8(R8::D))), (Some(B3(3)), Some(R8(R8::E))), (Some(B3(3)), Some(R8(R8::H))), (Some(B3(3)), Some(R8(R8::L))), (Some(B3(3)), Some(R8(R8::HL))), (Some(B3(3)), Some(R8(R8::A))), 
    (Some(B3(4)), Some(R8(R8::B))), (Some(B3(4)), Some(R8(R8::C))), (Some(B3(4)), Some(R8(R8::D))), (Some(B3(4)), Some(R8(R8::E))), (Some(B3(4)), Some(R8(R8::H))), (Some(B3(4)), Some(R8(R8::L))), (Some(B3(4)), Some(R8(R8::HL))), (Some(B3(4)), Some(R8(R8::A))), 
    (Some(B3(5)), Some(R8(R8::B))), (Some(B3(5)), Some(R8(R8::C))), (Some(B3(5)), Some(R8(R8::D))), (Some(B3(5)), Some(R8(R8::E))), (Some(B3(5)), Some(R8(R8::H))), (Some(B3(5)), Some(R8(R8::L))), (Some(B3(5)), Some(R8(R8::HL))), (Some(B3(5)), Some(R8(R8::A))), 
    (Some(B3(6)), Some(R8(R8::B))), (Some(B3(6)), Some(R8(R8::C))), (Some(B3(6)), Some(R8(R8::D))), (Some(B3(6)), Some(R8(R8::E))), (Some(B3(6)), Some(R8(R8::H))), (Some(B3(6)), Some(R8(R8::L))), (Some(B3(6)), Some(R8(R8::HL))), (Some(B3(6)), Some(R8(R8::A))), 
    (Some(B3(7)), Some(R8(R8::B))), (Some(B3(7)), Some(R8(R8::C))), (Some(B3(7)), Some(R8(R8::D))), (Some(B3(7)), Some(R8(R8::E))), (Some(B3(7)), Some(R8(R8::H))), (Some(B3(7)), Some(R8(R8::L))), (Some(B3(7)), Some(R8(R8::HL))), (Some(B3(7)), Some(R8(R8::A))),
    (Some(B3(0)), Some(R8(R8::B))), (Some(B3(0)), Some(R8(R8::C))), (Some(B3(0)), Some(R8(R8::D))), (Some(B3(0)), Some(R8(R8::E))), (Some(B3(0)), Some(R8(R8::H))), (Some(B3(0)), Some(R8(R8::L))), (Some(B3(0)), Some(R8(R8::HL))), (Some(B3(0)), Some(R8(R8::A))), 
    (Some(B3(1)), Some(R8(R8::B))), (Some(B3(1)), Some(R8(R8::C))), (Some(B3(1)), Some(R8(R8::D))), (Some(B3(1)), Some(R8(R8::E))), (Some(B3(1)), Some(R8(R8::H))), (Some(B3(1)), Some(R8(R8::L))), (Some(B3(1)), Some(R8(R8::HL))), (Some(B3(1)), Some(R8(R8::A))), 
    (Some(B3(2)), Some(R8(R8::B))), (Some(B3(2)), Some(R8(R8::C))), (Some(B3(2)), Some(R8(R8::D))), (Some(B3(2)), Some(R8(R8::E))), (Some(B3(2)), Some(R8(R8::H))), (Some(B3(2)), Some(R8(R8::L))), (Some(B3(2)), Some(R8(R8::HL))), (Some(B3(2)), Some(R8(R8::A))), 
    (Some(B3(3)), Some(R8(R8::B))), (Some(B3(3)), Some(R8(R8::C))), (Some(B3(3)), Some(R8(R8::D))), (Some(B3(3)), Some(R8(R8::E))), (Some(B3(3)), Some(R8(R8::H))), (Some(B3(3)), Some(R8(R8::L))), (Some(B3(3)), Some(R8(R8::HL))), (Some(B3(3)), Some(R8(R8::A))), 
    (Some(B3(4)), Some(R8(R8::B))), (Some(B3(4)), Some(R8(R8::C))), (Some(B3(4)), Some(R8(R8::D))), (Some(B3(4)), Some(R8(R8::E))), (Some(B3(4)), Some(R8(R8::H))), (Some(B3(4)), Some(R8(R8::L))), (Some(B3(4)), Some(R8(R8::HL))), (Some(B3(4)), Some(R8(R8::A))), 
    (Some(B3(5)), Some(R8(R8::B))), (Some(B3(5)), Some(R8(R8::C))), (Some(B3(5)), Some(R8(R8::D))), (Some(B3(5)), Some(R8(R8::E))), (Some(B3(5)), Some(R8(R8::H))), (Some(B3(5)), Some(R8(R8::L))), (Some(B3(5)), Some(R8(R8::HL))), (Some(B3(5)), Some(R8(R8::A))), 
    (Some(B3(6)), Some(R8(R8::B))), (Some(B3(6)), Some(R8(R8::C))), (Some(B3(6)), Some(R8(R8::D))), (Some(B3(6)), Some(R8(R8::E))), (Some(B3(6)), Some(R8(R8::H))), (Some(B3(6)), Some(R8(R8::L))), (Some(B3(6)), Some(R8(R8::HL))), (Some(B3(6)), Some(R8(R8::A))), 
    (Some(B3(7)), Some(R8(R8::B))), (Some(B3(7)), Some(R8(R8::C))), (Some(B3(7)), Some(R8(R8::D))), (Some(B3(7)), Some(R8(R8::E))), (Some(B3(7)), Some(R8(R8::H))), (Some(B3(7)), Some(R8(R8::L))), (Some(B3(7)), Some(R8(R8::HL))), (Some(B3(7)), Some(R8(R8::A))),
];
