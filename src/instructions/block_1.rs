use crate::{core::Core, instructions::instruction::Parameters};

pub fn ld_r8_r8(core: &mut Core, p: &(Option<Parameters>, Option<Parameters>)) {
    if let Parameters::R8(r8) = p.1.as_ref().unwrap() {
        let value = core.get_r8(r8);

        if let Parameters::R8(r8) = p.0.as_ref().unwrap() {
            core.set_r8(r8, value);
            return;
        }
    }

    unreachable!();
}

pub fn halt(core: &mut Core, _p: &(Option<Parameters>, Option<Parameters>)) {
    core.cpu.halted = true;
}
