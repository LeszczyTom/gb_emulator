use crate::memory::mmu::Mmu;

const LCDC: u16 = 0xFF40;

pub fn get_lcdc_n(bit: u8, mmu: &Mmu) -> bool {
    mmu.read_byte(LCDC) & 1 << bit == 1 << bit
}

pub fn is_ldc_and_ppu_enable(mmu: &Mmu) -> bool {
    get_lcdc_n(7, mmu)
}

// pub fn get_window_tile_map_area(memory: &mut Memory, joypad: &Joypad) -> u16 {
//     if get_lcdc_n(6, memory, joypad) {
//         0x9C00
//     } else {
//         0x9800
//     }
// }

// pub fn is_window_enable(memory: &mut Memory, joypad: &Joypad) -> bool {
//     get_lcdc_n(5, memory, joypad)
// }

// pub fn get_bg_and_tile_data_area(memory: &mut Memory, joypad: &Joypad) -> u16 {
//     if get_lcdc_n(4, memory, joypad) {
//         0x8000
//     } else {
//         0x8800
//     }
// }

// pub fn get_bg_tile_map_area(memory: &mut Memory, joypad: &Joypad) -> u16 {
//     if get_lcdc_n(3, memory, joypad) {
//         0x9C00
//     } else {
//         0x9800
//     }
// }

// pub fn is_bg_and_window_enable(memory: &mut Memory, joypad: &Joypad) -> bool {
//     get_lcdc_n(0, memory, joypad)
// }
