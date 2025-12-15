use crate::{
    core::Core,
    cpu::cpu::Flag::*,
    instructions::instruction::{Parameters, R16},
};

pub fn nop(_core: &mut Core, _p: &(Option<Parameters>, Option<Parameters>)) {}

pub fn ld_r16_imm16(core: &mut Core, p: &(Option<Parameters>, Option<Parameters>)) {
    if let Parameters::R16(r16) = p.0.as_ref().unwrap() {
        let n16 = core.fetch_imm16();

        match r16 {
            R16::BC => core.cpu.bc.set(n16),
            R16::DE => core.cpu.de.set(n16),
            R16::HL => core.cpu.hl.set(n16),
            R16::SP => core.cpu.sp.set(n16),
        };
        return;
    }

    unreachable!();
}

pub fn ld_r16mem_a(core: &mut Core, p: &(Option<Parameters>, Option<Parameters>)) {
    if let Parameters::R16mem(r16mem) = p.0.as_ref().unwrap() {
        let value = core.cpu.af.get_high();

        core.set_r16mem(r16mem, value);

        return;
    }

    unreachable!()
}

pub fn ld_a_r16mem(core: &mut Core, p: &(Option<Parameters>, Option<Parameters>)) {
    if let Parameters::R16mem(r16mem) = p.0.as_ref().unwrap() {
        let value = core.get_r16mem(r16mem);

        core.cpu.af.set_high(value);
        return;
    }

    unreachable!()
}

pub fn ld_imm16_sp(core: &mut Core, _p: &(Option<Parameters>, Option<Parameters>)) {
    let address = core.fetch_imm16();

    core.mmu.set((core.cpu.sp.get() & 0x00FF) as u8, address);
    core.mmu
        .set((core.cpu.sp.get() >> 8) as u8, address.wrapping_add(1));
}

pub fn inc_r16(core: &mut Core, p: &(Option<Parameters>, Option<Parameters>)) {
    if let Parameters::R16(r16) = p.0.as_ref().unwrap() {
        let value = core.get_r16(r16).wrapping_add(1);
        core.set_r16(r16, value);
        return;
    }

    unreachable!()
}

pub fn dec_r16(core: &mut Core, p: &(Option<Parameters>, Option<Parameters>)) {
    if let Parameters::R16(r16) = p.0.as_ref().unwrap() {
        let value = core.get_r16(r16).wrapping_sub(1);
        core.set_r16(r16, value);

        return;
    }

    unimplemented!();
}

pub fn add_hl_r16(core: &mut Core, p: &(Option<Parameters>, Option<Parameters>)) {
    if let Parameters::R16(r16) = p.0.as_ref().unwrap() {
        let value = core.get_r16(r16);
        let result = core.cpu.hl.get().overflowing_add(value);

        core.cpu.hl.set(result.0);

        core.cpu.set_flag(H, value & 0x0fff > result.0 & 0x0fff);
        core.cpu.set_flag(C, result.1);
        core.cpu.set_flag(N, false);

        return;
    }

    unreachable!();
}

pub fn inc_r8(core: &mut Core, p: &(Option<Parameters>, Option<Parameters>)) {
    if let Parameters::R8(r8) = p.0.as_ref().unwrap() {
        let value = core.get_r8(r8);
        let result = value.wrapping_add(1);

        core.set_r8(r8, result);

        core.cpu.set_flag(Z, result == 0);
        core.cpu.set_flag(N, false);
        core.cpu.set_flag(H, value & 0xf == 0xf);

        return;
    }
    unreachable!();
}

pub fn dec_r8(core: &mut Core, p: &(Option<Parameters>, Option<Parameters>)) {
    if let Parameters::R8(r8) = p.0.as_ref().unwrap() {
        let value = core.get_r8(r8);
        let result = value.wrapping_sub(1);

        core.set_r8(r8, result);

        core.cpu.set_flag(Z, result == 0);
        core.cpu.set_flag(N, true);
        core.cpu.set_flag(H, value & 0xf == 0);

        return;
    }
    unreachable!();
}

pub fn ld_r8_imm8(core: &mut Core, p: &(Option<Parameters>, Option<Parameters>)) {
    if let Parameters::R8(r8) = p.0.as_ref().unwrap() {
        let n8 = core.fetch_imm8();

        core.set_r8(r8, n8);
        return;
    }

    unreachable!()
}

pub fn rlca(core: &mut Core, _p: &(Option<Parameters>, Option<Parameters>)) {
    let value = core.cpu.af.get_high();

    core.cpu.af.set_high((value << 1) | (value >> 7));

    core.cpu.set_flag(Z, false);
    core.cpu.set_flag(N, false);
    core.cpu.set_flag(H, false);
    core.cpu.set_flag(C, value > 0x7f);
}

pub fn rrca(core: &mut Core, _p: &(Option<Parameters>, Option<Parameters>)) {
    let value = core.cpu.af.get_high();
    let result = value.rotate_right(1);

    core.cpu.af.set_high(result);

    core.cpu.set_flag(Z, false);
    core.cpu.set_flag(N, false);
    core.cpu.set_flag(H, false);
    core.cpu.set_flag(C, value & 0x01 == 1);
}

pub fn rla(core: &mut Core, _p: &(Option<Parameters>, Option<Parameters>)) {
    let value = core.cpu.af.get_high();
    let carry = core.cpu.get_flag(C);
    let result = (value << 1) | carry as u8;

    core.cpu.af.set_high(result);

    core.cpu.set_flag(Z, false);
    core.cpu.set_flag(N, false);
    core.cpu.set_flag(H, false);
    core.cpu.set_flag(C, value >> 7 == 1);
}

pub fn rra(core: &mut Core, _p: &(Option<Parameters>, Option<Parameters>)) {
    let value = core.cpu.af.get_high();
    let carry = core.cpu.get_flag(C) as u8;

    core.cpu.af.set_high((value >> 1) | (carry << 7));

    core.cpu.set_flag(Z, false);
    core.cpu.set_flag(N, false);
    core.cpu.set_flag(H, false);
    core.cpu.set_flag(C, value & 0x01 == 1);
}

pub fn daa(core: &mut Core, _p: &(Option<Parameters>, Option<Parameters>)) {
    let mut result: u8 = core.cpu.af.get_high();

    if core.cpu.get_flag(N) {
        let mut adjustement = 0;

        if core.cpu.get_flag(H) {
            adjustement = 0x06;
        }

        if core.cpu.get_flag(C) {
            adjustement += 0x60;
        }

        result = result.wrapping_sub(adjustement);
    } else {
        let mut adjustement = 0;

        if core.cpu.get_flag(H) || (core.cpu.af.get_high() & 0x0F) > 0x09 {
            adjustement += 0x06;
        }

        if core.cpu.get_flag(C) || core.cpu.af.get_high() > 0x99 {
            adjustement += 0x60;
            core.cpu.set_flag(C, true);
        }

        result = result.wrapping_add(adjustement);
    }

    core.cpu.af.set_high(result);
    core.cpu.set_flag(Z, result == 0);
    core.cpu.set_flag(H, false);
}

pub fn cpl(core: &mut Core, _p: &(Option<Parameters>, Option<Parameters>)) {
    core.cpu.af.set_high(!core.cpu.af.get_high());

    core.cpu.set_flag(N, true);
    core.cpu.set_flag(H, true);
}

pub fn scf(core: &mut Core, _p: &(Option<Parameters>, Option<Parameters>)) {
    core.cpu.set_flag(N, false);
    core.cpu.set_flag(H, false);
    core.cpu.set_flag(C, true);
}

pub fn ccf(core: &mut Core, _p: &(Option<Parameters>, Option<Parameters>)) {
    core.cpu.set_flag(N, false);
    core.cpu.set_flag(H, false);
    core.cpu.set_flag(C, !core.cpu.get_flag(C));
}

pub fn jr_imm8(core: &mut Core, _p: &(Option<Parameters>, Option<Parameters>)) {
    let offset = core.fetch_imm8() as i8 as u16;

    core.cpu.pc.set(core.cpu.pc.get().wrapping_add(offset));
}

pub fn jr_cond_imm8(core: &mut Core, p: &(Option<Parameters>, Option<Parameters>)) {
    if let Parameters::Cond(cond) = p.0.as_ref().unwrap() {
        let offset = core.fetch_imm8() as i8 as u16;

        if core.is_condition_met(cond) {
            core.cpu.condition_met = true;
            core.cpu.pc.set(core.cpu.pc.get().wrapping_add(offset));
        }

        return;
    }
    unreachable!();
}

pub fn stop(_core: &mut Core, _p: &(Option<Parameters>, Option<Parameters>)) {
    todo!()
}
