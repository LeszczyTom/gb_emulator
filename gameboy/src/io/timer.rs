use crate::memory::mmu::Mmu;

const DIV_LOW_ADDRESS: u16 = 0xFF03;
const DIV_HIGH_ADDRESS: u16 = 0xFF04;
const TIMA_ADDRESS: u16 = 0xFF05;
const TMA_ADDRESS: u16 = 0xFF06;
const TAC_ADDRESS: u16 = 0xFF07;

pub fn update(mmu: &mut Mmu) {
    increment_divder(mmu);

    if !is_timer_enable(mmu) {
        return;
    }

    tick(mmu);
}

fn tick(mmu: &mut Mmu) {
    let div_counter: u16 = u16::from_be_bytes([
        mmu.read_byte(DIV_HIGH_ADDRESS),
        mmu.read_byte(DIV_LOW_ADDRESS),
    ]);

    let tac_frequency = get_tac_frequency(mmu);
    if div_counter & tac_frequency != 0 && div_counter - 1 & tac_frequency == 0 {
        reset_divider(mmu);
        increment_tima(mmu);
    }
}

/// Returns true if the timer is enabled, false otherwise.
fn is_timer_enable(mmu: &mut Mmu) -> bool {
    mmu.read_byte(TAC_ADDRESS) & 4 == 4
}

/// Increment the divider register. The divider register is on 2 bytes. When the lower byte overflow, the higher byte is incremented.
fn increment_divder(mmu: &mut Mmu) {
    let div_low = mmu.read_byte(DIV_LOW_ADDRESS);

    if div_low == 255 {
        mmu.reset_div_low();
        mmu.increment_div_high();
        return;
    }

    mmu.increment_div_low();
}

/// Reset the divider register. The divider register is on 2 bytes, so both bytes are set to 0.
fn reset_divider(mmu: &mut Mmu) {
    mmu.reset_div_low();
    mmu.reset_div_high();
}

/// Increment the TIMA register. If the TIMA register overflow, the TMA register is copied to the TIMA register and the interrupt flag is set.
fn increment_tima(mmu: &mut Mmu) {
    let tima = mmu.read_byte(TIMA_ADDRESS);

    if tima == 255 {
        mmu.write_byte(TIMA_ADDRESS, mmu.read_byte(TMA_ADDRESS));
        mmu.set_interrupt_flag(2);
        return;
    }

    mmu.write_byte(TIMA_ADDRESS, tima + 1);
}

/// Returns the frequency of the timer.
/// Bits 1-0 - Input Clock Select
/// 00: CPU Clock / 1024 (DMG, SGB2, CGB Single Speed Mode:   4096 Hz, SGB1:   ~4194 Hz, CGB Double Speed Mode:   8192 Hz)
/// 01: CPU Clock / 16   (DMG, SGB2, CGB Single Speed Mode: 262144 Hz, SGB1: ~268400 Hz, CGB Double Speed Mode: 524288 Hz)
/// 10: CPU Clock / 64   (DMG, SGB2, CGB Single Speed Mode:  65536 Hz, SGB1:  ~67110 Hz, CGB Double Speed Mode: 131072 Hz)
/// 11: CPU Clock / 256  (DMG, SGB2, CGB Single Speed Mode:  16384 Hz, SGB1:  ~16780 Hz, CGB Double Speed Mode:  32768 Hz)
fn get_tac_frequency(mmu: &mut Mmu) -> u16 {
    match mmu.read_byte(TAC_ADDRESS) & 0x3 {
        0 => 512,
        1 => 8,
        2 => 32,
        3 => 128,
        _ => unreachable!(),
    }
}
