use crate::{
    core::Core,
    cpu::cpu::Flag::*,
    instructions::instruction::{Parameters, R16stk},
};

pub fn add_a_imm8(core: &mut Core, _p: &(Option<Parameters>, Option<Parameters>)) {
    let a = core.cpu.af.get_high();
    let imm8 = core.fetch_imm8();
    let result = a.wrapping_add(imm8);

    core.cpu.af.set_high(result);

    core.cpu.set_flag(Z, result == 0);
    core.cpu.set_flag(N, false);
    core.cpu.set_flag(H, (result & 0xf) < (imm8 & 0xf));
    core.cpu.set_flag(C, (result as u16) < (imm8 as u16));
}

pub fn adc_a_imm8(core: &mut Core, _p: &(Option<Parameters>, Option<Parameters>)) {
    let carry = core.cpu.get_flag(C) as u8;
    let a = core.cpu.af.get_high();
    let value = core.fetch_imm8();
    let result = a.wrapping_add(value).wrapping_add(carry);

    core.cpu.af.set_high(result);

    core.cpu.set_flag(Z, result == 0);
    core.cpu.set_flag(N, false);
    core.cpu
        .set_flag(H, (a & 0xf) + (value & 0xf) + carry > 0xf);
    core.cpu
        .set_flag(C, a as u16 + value as u16 + carry as u16 > 0xff);
}

pub fn sub_a_imm8(core: &mut Core, _p: &(Option<Parameters>, Option<Parameters>)) {
    let a = core.cpu.af.get_high();
    let imm8 = core.fetch_imm8();
    let result = a.wrapping_sub(imm8);

    core.cpu.af.set_high(result);

    core.cpu.set_flag(Z, result == 0);
    core.cpu.set_flag(N, true);
    core.cpu
        .set_flag(H, (a & 0xf).wrapping_sub(imm8 & 0xf) & (0x10) != 0);
    core.cpu.set_flag(C, (a as u16) < (imm8 as u16));
}

pub fn sbc_a_imm8(core: &mut Core, _p: &(Option<Parameters>, Option<Parameters>)) {
    let value = core.fetch_imm8();
    let a = core.cpu.af.get_high();
    let carry = core.cpu.get_flag(C) as u8;
    let result = a.wrapping_sub(value).wrapping_sub(carry);

    core.cpu.set_flag(Z, result == 0);
    core.cpu.set_flag(N, true);
    core.cpu.set_flag(H, (a & 0xf) < (value & 0xf) + carry);
    core.cpu
        .set_flag(C, (a as u16) < (value as u16) + (carry as u16));

    core.cpu.af.set_high(result);
}

pub fn and_a_imm8(core: &mut Core, _p: &(Option<Parameters>, Option<Parameters>)) {
    let a = core.cpu.af.get_high();
    let imm8 = core.fetch_imm8();
    let result = a & imm8;

    core.cpu.af.set_high(result);

    core.cpu.set_flag(Z, result == 0);
    core.cpu.set_flag(C, false);
    core.cpu.set_flag(H, true);
    core.cpu.set_flag(N, false);
}

pub fn xor_a_imm8(core: &mut Core, _p: &(Option<Parameters>, Option<Parameters>)) {
    let a = core.cpu.af.get_high();
    let imm8 = core.fetch_imm8();
    let result = a ^ imm8;

    core.cpu.af.set_high(result);

    core.cpu.set_flag(Z, result == 0);
    core.cpu.set_flag(C, false);
    core.cpu.set_flag(H, false);
    core.cpu.set_flag(N, false);
}

pub fn or_a_imm8(core: &mut Core, _p: &(Option<Parameters>, Option<Parameters>)) {
    let value = core.fetch_imm8();
    let result = core.cpu.af.get_high() | value;
    core.cpu.af.set_high(result);

    core.cpu.set_flag(Z, result == 0);
    core.cpu.set_flag(C, false);
    core.cpu.set_flag(H, false);
    core.cpu.set_flag(N, false);
}

pub fn cp_a_imm8(core: &mut Core, _p: &(Option<Parameters>, Option<Parameters>)) {
    let a = core.cpu.af.get_high();
    let value = core.fetch_imm8();
    let result = a.wrapping_sub(value);

    core.cpu.set_flag(Z, result == 0);
    core.cpu.set_flag(C, value > a);
    core.cpu.set_flag(H, value & 0xf > a & 0xf);
    core.cpu.set_flag(N, true);
}

pub fn ret_cond(core: &mut Core, p: &(Option<Parameters>, Option<Parameters>)) {
    if let Parameters::Cond(cond) = p.0.as_ref().unwrap() {
        core.tick_timer(1);

        if core.is_condition_met(cond) {
            ret(core, p);
        }

        return;
    }

    unreachable!()
}

pub fn ret(core: &mut Core, _p: &(Option<Parameters>, Option<Parameters>)) {
    let value = core.pop();
    core.cpu.pc.set(value);
    core.tick_timer(1);
}

pub fn reti(core: &mut Core, p: &(Option<Parameters>, Option<Parameters>)) {
    core.cpu.set_ime = None;
    ret(core, p);
}

pub fn jp_cond_imm16(core: &mut Core, p: &(Option<Parameters>, Option<Parameters>)) {
    if let Parameters::Cond(cond) = p.0.as_ref().unwrap() {
        let address = core.fetch_imm16();

        if core.is_condition_met(cond) {
            core.cpu.pc.set(address);
            core.tick_timer(1);
        }

        return;
    }
    unreachable!()
}

pub fn jp_imm16(core: &mut Core, _p: &(Option<Parameters>, Option<Parameters>)) {
    let address = core.fetch_imm16();
    core.cpu.pc.set(address);
    core.tick_timer(1);
}

pub fn jp_hl(core: &mut Core, _p: &(Option<Parameters>, Option<Parameters>)) {
    let address = core.cpu.hl.get();
    core.cpu.pc.set(address);
}

pub fn call_cond_imm16(core: &mut Core, p: &(Option<Parameters>, Option<Parameters>)) {
    if let Parameters::Cond(cond) = p.0.as_ref().unwrap() {
        let addr = core.fetch_imm16();

        if core.is_condition_met(cond) {
            core.push(core.cpu.pc.get());
            core.cpu.pc.set(addr);
            core.tick_timer(1);
        }

        return;
    }

    unreachable!();
}

pub fn call_imm16(core: &mut Core, _p: &(Option<Parameters>, Option<Parameters>)) {
    let addr = core.fetch_imm16();
    core.push(core.cpu.pc.get());
    core.cpu.pc.set(addr);
    core.tick_timer(1);
}

pub fn rst_tgt3(core: &mut Core, p: &(Option<Parameters>, Option<Parameters>)) {
    if let Parameters::Tgt3(tgt3) = p.0.as_ref().unwrap() {
        core.push(core.cpu.pc.get());
        core.cpu.pc.set(*tgt3);
        core.tick_timer(1);

        return;
    }
    unreachable!()
}

pub fn pop_r16stk(core: &mut Core, p: &(Option<Parameters>, Option<Parameters>)) {
    if let Parameters::R16stk(r16stk) = p.0.as_ref().unwrap() {
        let value = core.pop();
        core.set_r16stk(r16stk, value);

        if r16stk == &R16stk::AF {
            core.cpu.af.set_low(core.cpu.af.get_low() & 0xF0);
        }

        return;
    }

    unreachable!()
}

pub fn push_r16stk(core: &mut Core, p: &(Option<Parameters>, Option<Parameters>)) {
    if let Parameters::R16stk(r16stk) = p.0.as_ref().unwrap() {
        let value = core.get_r16stk(r16stk);
        core.tick_timer(1);
        core.push(value);

        return;
    }

    unreachable!()
}

pub fn prefix(core: &mut Core, _p: &(Option<Parameters>, Option<Parameters>)) {
    core.cpu.cb = true;
}

pub fn ldh_c_a(core: &mut Core, _p: &(Option<Parameters>, Option<Parameters>)) {
    let address = (core.cpu.bc.get_low() as u16).wrapping_add(0xFF00);
    let value = core.cpu.af.get_high();
    core.mem_set(value, address);
}

pub fn ldh_imm8_a(core: &mut Core, _p: &(Option<Parameters>, Option<Parameters>)) {
    let addr = u16::from_be_bytes([0xff, core.fetch_imm8()]);
    core.mem_set(core.cpu.af.get_high(), addr);
}

pub fn ld_imm16_a(core: &mut Core, _p: &(Option<Parameters>, Option<Parameters>)) {
    let address = core.fetch_imm16();
    let value = core.cpu.af.get_high();

    core.mem_set(value, address);
}

pub fn ldh_a_c(core: &mut Core, _p: &(Option<Parameters>, Option<Parameters>)) {
    let address = (core.cpu.bc.get_low() as u16).wrapping_add(0xFF00);
    let value = core.mem_get(address);
    core.cpu.af.set_high(value);
}

pub fn ldh_a_imm8(core: &mut Core, _p: &(Option<Parameters>, Option<Parameters>)) {
    let addr = u16::from_be_bytes([0xff, core.fetch_imm8()]);
    let value = core.mem_get(addr);
    core.cpu.af.set_high(value);
}

pub fn ld_a_imm16(core: &mut Core, _p: &(Option<Parameters>, Option<Parameters>)) {
    let imm16 = core.fetch_imm16();
    let value = core.mem_get(imm16);
    core.cpu.af.set_high(value);
}

pub fn add_sp_imm8(core: &mut Core, _p: &(Option<Parameters>, Option<Parameters>)) {
    let sp = core.cpu.sp.get();
    let e8 = core.fetch_imm8() as i8 as u16;
    let result = core.cpu.sp.get().wrapping_add(e8);
    core.tick_timer(1);

    core.cpu.sp.set(result);

    core.cpu.set_flag(Z, false);
    core.cpu.set_flag(N, false);
    core.cpu.set_flag(H, (sp ^ e8 ^ result) & 0x10 == 0x10);
    core.cpu.set_flag(C, (sp ^ e8 ^ result) & 0x100 == 0x100);

    core.tick_timer(1);
}

pub fn ld_hl_sp_imm8(core: &mut Core, _p: &(Option<Parameters>, Option<Parameters>)) {
    let sp = core.cpu.sp.get();
    let e8 = core.fetch_imm8() as i8 as u16;
    let result = sp.wrapping_add(e8);

    core.cpu.hl.set(result);

    core.cpu.set_flag(Z, false);
    core.cpu.set_flag(N, false);
    core.cpu.set_flag(H, (sp ^ e8 ^ result) & 0x10 == 0x10);
    core.cpu.set_flag(C, (sp ^ e8 ^ result) & 0x100 == 0x100);

    core.tick_timer(1);
}

pub fn ld_sp_hl(core: &mut Core, _p: &(Option<Parameters>, Option<Parameters>)) {
    let value = core.cpu.hl.get();
    core.cpu.sp.set(value);
    core.tick_timer(1);
}

pub fn di(core: &mut Core, _p: &(Option<Parameters>, Option<Parameters>)) {
    core.cpu.ime = false;
}

pub fn ei(core: &mut Core, _p: &(Option<Parameters>, Option<Parameters>)) {
    core.cpu.set_ime = Some(1);
}
