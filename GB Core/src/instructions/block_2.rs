use crate::{core::Core, cpu::cpu::Flag::*, instructions::instruction::Parameters};

pub fn add_a_r8(core: &mut Core, p: &(Option<Parameters>, Option<Parameters>)) {
    if let Parameters::R8(r8) = p.0.as_ref().unwrap() {
        let value = core.get_r8(r8);
        let a = core.cpu.af.get_high();
        let result = value.wrapping_add(a);

        core.cpu.af.set_high(result);

        core.cpu.set_flag(Z, result == 0);
        core.cpu.set_flag(N, false);
        core.cpu.set_flag(H, (result & 0xf) < (value & 0xf));
        core.cpu.set_flag(C, (result as u16) < (value as u16));

        return;
    }

    unimplemented!();
}

pub fn adc_a_r8(core: &mut Core, p: &(Option<Parameters>, Option<Parameters>)) {
    if let Parameters::R8(r8) = p.0.as_ref().unwrap() {
        let value = core.get_r8(r8);
        let carry = core.cpu.get_flag(C) as u8;
        let a = core.cpu.af.get_high();
        let result = a.wrapping_add(value).wrapping_add(carry);

        core.cpu.af.set_high(result);

        core.cpu.set_flag(Z, result == 0);
        core.cpu.set_flag(N, false);
        core.cpu
            .set_flag(H, (a & 0xf) + (value & 0xf) + carry > 0xf);
        core.cpu
            .set_flag(C, a as u16 + value as u16 + carry as u16 > 0xff);

        return;
    }
    unreachable!()
}

pub fn sub_a_r8(core: &mut Core, p: &(Option<Parameters>, Option<Parameters>)) {
    if let Parameters::R8(r8) = p.0.as_ref().unwrap() {
        let value = core.get_r8(r8);
        let a = core.cpu.af.get_high();
        let result = a.wrapping_sub(value);

        core.cpu.af.set_high(result);

        core.cpu.set_flag(Z, result == 0);
        core.cpu.set_flag(N, true);
        core.cpu
            .set_flag(H, (a & 0xf).wrapping_sub(value & 0xf) & (0x10) != 0);
        core.cpu.set_flag(C, (a as u16) < (value as u16));

        return;
    }
    unreachable!();
}

pub fn sbc_a_r8(core: &mut Core, p: &(Option<Parameters>, Option<Parameters>)) {
    if let Parameters::R8(r8) = p.0.as_ref().unwrap() {
        let value = core.get_r8(r8);
        let a = core.cpu.af.get_high();
        let carry = core.cpu.get_flag(C) as u8;
        let result = a.wrapping_sub(value).wrapping_sub(carry);

        core.cpu.af.set_high(result);

        core.cpu.set_flag(Z, result == 0);
        core.cpu.set_flag(N, true);
        core.cpu.set_flag(H, (a & 0xf) < (value & 0xf) + carry);
        core.cpu
            .set_flag(C, (a as u16) < (value as u16) + (carry as u16));

        return;
    }
    unreachable!();
}

pub fn and_a_r8(core: &mut Core, p: &(Option<Parameters>, Option<Parameters>)) {
    if let Parameters::R8(r8) = p.0.as_ref().unwrap() {
        let a = core.cpu.af.get_high();
        let r8 = core.get_r8(r8);
        let result = a & r8;

        core.cpu.af.set_high(result);

        core.cpu.set_flag(Z, result == 0);
        core.cpu.set_flag(C, false);
        core.cpu.set_flag(H, true);
        core.cpu.set_flag(N, false);

        return;
    }

    unreachable!();
}

pub fn xor_a_r8(core: &mut Core, p: &(Option<Parameters>, Option<Parameters>)) {
    if let Parameters::R8(r8) = p.0.as_ref().unwrap() {
        let a = core.cpu.af.get_high();
        let result = a ^ core.get_r8(r8);

        core.cpu.af.set_high(result);

        core.cpu.set_flag(Z, result == 0);
        core.cpu.set_flag(C, false);
        core.cpu.set_flag(H, false);
        core.cpu.set_flag(N, false);

        return;
    }

    unreachable!();
}

pub fn or_a_r8(core: &mut Core, p: &(Option<Parameters>, Option<Parameters>)) {
    if let Parameters::R8(r8) = p.0.as_ref().unwrap() {
        let value = core.get_r8(r8);
        let result = core.cpu.af.get_high() | value;
        core.cpu.af.set_high(result);

        core.cpu.set_flag(Z, result == 0);
        core.cpu.set_flag(C, false);
        core.cpu.set_flag(H, false);
        core.cpu.set_flag(N, false);

        return;
    }

    unreachable!();
}

pub fn cp_a_r8(core: &mut Core, p: &(Option<Parameters>, Option<Parameters>)) {
    if let Parameters::R8(r8) = p.0.as_ref().unwrap() {
        let a = core.cpu.af.get_high();
        let value = core.get_r8(r8);
        let result = a.wrapping_sub(value);

        core.cpu.set_flag(Z, result == 0);
        core.cpu.set_flag(C, value > a);
        core.cpu.set_flag(H, value & 0xf > a & 0xf);
        core.cpu.set_flag(N, true);

        return;
    }
    unreachable!();
}
