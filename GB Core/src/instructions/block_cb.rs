use crate::{core::Core, cpu::cpu::Flag::*, instructions::instruction::Parameters};

pub fn rlc_r8(core: &mut Core, p: &(Option<Parameters>, Option<Parameters>)) {
    if let Parameters::R8(r8) = p.0.as_ref().unwrap() {
        let value = core.get_r8(r8);
        let result = value.rotate_left(1);

        core.cpu.set_flag(Z, result == 0);
        core.cpu.set_flag(N, false);
        core.cpu.set_flag(H, false);
        core.cpu.set_flag(C, value >> 7 == 1);

        core.set_r8(r8, result);

        return;
    }
    unreachable!()
}

pub fn rrc_r8(core: &mut Core, p: &(Option<Parameters>, Option<Parameters>)) {
    if let Parameters::R8(r8) = p.0.as_ref().unwrap() {
        let value = core.get_r8(r8);
        let result = value.rotate_right(1);

        core.cpu.set_flag(Z, result == 0);
        core.cpu.set_flag(N, false);
        core.cpu.set_flag(H, false);
        core.cpu.set_flag(C, value & 0x01 == 1);

        core.set_r8(r8, result);

        return;
    }
    unreachable!()
}

pub fn rl_r8(core: &mut Core, p: &(Option<Parameters>, Option<Parameters>)) {
    if let Parameters::R8(r8) = p.0.as_ref().unwrap() {
        let value = core.get_r8(r8);
        let carry = core.cpu.get_flag(C) as u8;
        let result = (value << 1) | carry as u8;

        core.cpu.set_flag(Z, result == 0);
        core.cpu.set_flag(N, false);
        core.cpu.set_flag(H, false);
        core.cpu.set_flag(C, value >> 7 == 1);

        core.set_r8(r8, result);

        return;
    }

    unreachable!();
}

pub fn rr_r8(core: &mut Core, p: &(Option<Parameters>, Option<Parameters>)) {
    if let Parameters::R8(r8) = p.0.as_ref().unwrap() {
        let value = core.get_r8(r8);
        let carry = core.cpu.get_flag(C) as u8;
        let result = (value >> 1) | (carry << 7);

        core.cpu.set_flag(Z, result == 0);
        core.cpu.set_flag(N, false);
        core.cpu.set_flag(H, false);
        core.cpu.set_flag(C, value & 0x01 == 1);

        core.set_r8(r8, result);

        return;
    }
    unreachable!()
}

pub fn sla_r8(core: &mut Core, p: &(Option<Parameters>, Option<Parameters>)) {
    if let Parameters::R8(r8) = p.0.as_ref().unwrap() {
        let value = core.get_r8(r8);
        let result = value << 1;

        core.cpu.set_flag(Z, result == 0);
        core.cpu.set_flag(N, false);
        core.cpu.set_flag(H, false);
        core.cpu.set_flag(C, value & 0x80 == 0x80);

        core.set_r8(r8, result);

        return;
    }

    unreachable!()
}

pub fn sra_r8(core: &mut Core, p: &(Option<Parameters>, Option<Parameters>)) {
    if let Parameters::R8(r8) = p.0.as_ref().unwrap() {
        let value = core.get_r8(r8);
        let result = (value >> 1) | (value & 0x80);

        core.cpu.set_flag(Z, result == 0);
        core.cpu.set_flag(N, false);
        core.cpu.set_flag(H, false);
        core.cpu.set_flag(C, value & 0x01 == 0x01);

        core.set_r8(r8, result);

        return;
    }
    unreachable!()
}

pub fn swap_r8(core: &mut Core, p: &(Option<Parameters>, Option<Parameters>)) {
    if let Parameters::R8(r8) = p.0.as_ref().unwrap() {
        let value = core.get_r8(r8);
        let result = value << 4 & 0xF0 | value >> 4;

        core.cpu.set_flag(Z, result == 0);
        core.cpu.set_flag(N, false);
        core.cpu.set_flag(H, false);
        core.cpu.set_flag(C, false);

        core.set_r8(r8, result);

        return;
    }

    unreachable!()
}

pub fn srl_r8(core: &mut Core, p: &(Option<Parameters>, Option<Parameters>)) {
    if let Parameters::R8(r8) = p.0.as_ref().unwrap() {
        let value = core.get_r8(r8);
        let result = value >> 1;

        core.cpu.set_flag(Z, result == 0);
        core.cpu.set_flag(N, false);
        core.cpu.set_flag(H, false);
        core.cpu.set_flag(C, value & 0x01 == 0x01);

        core.set_r8(r8, result);

        return;
    }
    unreachable!();
}

pub fn bit_b3_r8(core: &mut Core, p: &(Option<Parameters>, Option<Parameters>)) {
    if let Parameters::B3(b3) = p.0.as_ref().unwrap() {
        if let Parameters::R8(r8) = p.1.as_ref().unwrap() {
            let zero = core.get_r8(r8) & (1 << b3) == 0;
            core.cpu.set_flag(Z, zero);
            core.cpu.set_flag(N, false);
            core.cpu.set_flag(H, true);

            return;
        }
    }

    unreachable!()
}

pub fn res_b3_r8(core: &mut Core, p: &(Option<Parameters>, Option<Parameters>)) {
    if let Parameters::B3(b3) = p.0.as_ref().unwrap() {
        if let Parameters::R8(r8) = p.1.as_ref().unwrap() {
            let value = core.get_r8(r8);
            core.set_r8(r8, value & !(1 << b3));

            return;
        }
    }

    unreachable!()
}

pub fn set_b3_r8(core: &mut Core, p: &(Option<Parameters>, Option<Parameters>)) {
    if let Parameters::B3(b3) = p.0.as_ref().unwrap() {
        if let Parameters::R8(r8) = p.1.as_ref().unwrap() {
            let value = core.get_r8(r8);
            core.set_r8(r8, value | (1 << b3));

            return;
        }
    }

    unreachable!();
}
