use crate::cpu::cpu::{Cpu, RegisterPair};
use crate::cpu::gmb_16_bit_loadcommands::push_rr;
use crate::memory::mmu::Mmu;

const INTERRUPT_ENABLE_ADDRESS: u16 = 0xFFFF;
const INTERRUPT_FLAG_ADDRESS: u16 = 0xFF0F;

pub fn exectute_interrupts(cpu: &mut Cpu, mmu: &mut Mmu) -> u8 {
    let interrupt_flag = mmu.read_byte(INTERRUPT_FLAG_ADDRESS);
    let interrupt_enable = mmu.read_byte(INTERRUPT_ENABLE_ADDRESS);

    if !cpu.ime {
        return 0;
    }

    for flag in 0..5 {
        if interrupt_flag & (1 << flag) == 0 {
            continue;
        }

        if interrupt_enable & (1 << flag) == 0 {
            continue;
        }

        cpu.ime = false;
        cpu.halt = false;
        reset_interrupt_flag(mmu, flag);
        push_rr(RegisterPair::PC, cpu, mmu);
        cpu.set_rr(RegisterPair::PC, get_interrupt_address(flag));

        return 20;
    }

    return 0;
}

fn reset_interrupt_flag(memory: &mut Mmu, flag: u8) {
    let mut interrupt_flag = memory.read_byte(INTERRUPT_FLAG_ADDRESS);
    interrupt_flag &= !(1 << flag);
    memory.write_byte(INTERRUPT_FLAG_ADDRESS, interrupt_flag);
}

fn get_interrupt_address(flag: u8) -> u16 {
    match flag {
        0 => 0x40,
        1 => 0x48,
        2 => 0x50,
        3 => 0x58,
        4 => 0x60,
        _ => panic!("Invalid interrupt flag"),
    }
}
