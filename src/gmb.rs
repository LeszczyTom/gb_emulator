use super::cpu::LR35902;
use super::memory::MEMORY;

// http://bgb.bircd.org/pandocs.htm
pub struct GMB {
    cpu: LR35902,
    memory: MEMORY,
}

impl GMB {
    pub fn new() -> GMB {
        GMB {
            cpu: LR35902::new(),
            memory: MEMORY::new(),
        }
    }

    pub fn run(&mut self) {
        let mut max = 75;
        loop {
            if max <= 0 {
                break;
            }
            max -= 1;
            let addr = self.cpu.get_pc();
            let op = self.fetch_opcode();
            let mut cycle = self.execute_opcode(op);
            println!("{:04x}: {:02x}", addr, op);
            while cycle != 0 {
                cycle -= 4;
                let sec = core::time::Duration::from_millis(10);
                std::thread::sleep(sec);
            }
        }
        //self.memory.dump_vram();
    }

    pub fn load_rom(&mut self, rom: Vec<u8>) {
        self.memory.load_rom(rom);
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
        self.memory.reset();
    }

    fn fetch_opcode(&mut self) -> u8 {
        let opcode = self.memory.read_byte(self.cpu.get_pc());
        self.cpu.set_pc(self.cpu.get_pc().wrapping_add(1));
        opcode
    }

    fn execute_opcode(&mut self, op: u8) -> u8 {
        match op {
            0x00 => self.nop(),
            0x01 => self.load_rr_nn("bc"),
            0x02 => self.load_bc_a(),
            0x03 => self.inc_rr("bc"),
            0x04 => self.inc_r("b"),
            0x05 => self.dec_r("b"),
            0x06 => self.load_r_n("b"),
            0x07 => self.rlca(),
            0x08 => 0,
            0x09 => self.add_hl_rr("bc"),
            0x0A => self.load_a_bc(),
            0x0B => self.dec_rr("bc"),
            0x0C => self.inc_r("c"),
            0x0D => self.dec_r("c"),
            0x0E => self.load_r_n("c"),
            0x0F => self.rrca(),

            0x10 => self.stop(),
            0x11 => self.load_rr_nn("de"),
            0x12 => self.load_de_a(),
            0x13 => self.inc_rr("de"),
            0x14 => self.inc_r("d"),
            0x15 => self.dec_r("d"),
            0x16 => self.load_r_n("d"),
            0x17 => self.rla(),
            0x18 => self.jr_pc_dd(),
            0x19 => self.add_hl_rr("de"),
            0x1A => self.load_a_de(),
            0x1B => self.dec_rr("de"),
            0x1C => self.inc_r("e"),
            0x1D => self.dec_r("e"),
            0x1E => self.load_r_n("e"),
            0x1F => self.rra(),
            
            0x20 => self.jr_f_pc_dd("nz"),
            0x21 => self.load_rr_nn("hl"),
            0x22 => self.load_hli_a(),
            0x23 => self.inc_rr("hl"),
            0x24 => self.inc_r("h"),
            0x25 => self.dec_r("h"),
            0x26 => self.load_r_n("h"),
            0x27 => self.daa(),
            0x28 => self.jr_f_pc_dd("z"),
            0x29 => self.add_hl_rr("hl"),
            0x2A => self.load_a_hli(),
            0x2B => self.dec_rr("hl"),
            0x2C => self.inc_r("l"),
            0x2D => self.dec_r("l"),
            0x2E => self.load_r_n("l"),
            0x2F => self.cpl(),

            0x30 => self.jr_f_pc_dd("nc"),
            0x31 => self.load_rr_nn("sp"),
            0x32 => self.load_hld_a(),
            0x33 => self.inc_rr("sp"),
            0x34 => self.inc_hl(),
            0x35 => self.dec_hl(),
            0x36 => self.load_hl_n(),
            0x37 => self.scf(),
            0x38 => self.jr_f_pc_dd("c"),
            0x39 => self.add_hl_rr("sp"),
            0x3A => self.load_a_hld(),
            0x3B => self.dec_rr("sp"),
            0x3C => self.inc_r("a"),
            0x3D => self.dec_r("a"),
            0x3E => self.load_r_n("a"),
            0x3F => self.ccf(),

            0x40 => self.load_r_r("b", "b"),
            0x41 => self.load_r_r("b", "c"),
            0x42 => self.load_r_r("b", "d"),
            0x43 => self.load_r_r("b", "e"),
            0x44 => self.load_r_r("b", "h"),
            0x45 => self.load_r_r("b", "l"),
            0x46 => self.load_r_hl("b"),
            0x47 => self.load_r_r("b", "a"),
            0x48 => self.load_r_r("c", "b"),
            0x49 => self.load_r_r("c", "c"),
            0x4A => self.load_r_r("c", "d"),
            0x4B => self.load_r_r("c", "e"),
            0x4C => self.load_r_r("c", "h"),
            0x4D => self.load_r_r("c", "l"),
            0x4E => self.load_r_hl("c"),
            0x4F => self.load_r_r("c", "a"),

            0x50 => self.load_r_r("d", "b"),
            0x51 => self.load_r_r("d", "c"),
            0x52 => self.load_r_r("d", "d"),
            0x53 => self.load_r_r("d", "e"),
            0x54 => self.load_r_r("d", "h"),
            0x55 => self.load_r_r("d", "l"),
            0x56 => self.load_r_hl("d"),
            0x57 => self.load_r_r("d", "a"),
            0x58 => self.load_r_r("e", "b"),
            0x59 => self.load_r_r("e", "c"),
            0x5A => self.load_r_r("e", "d"),
            0x5B => self.load_r_r("e", "e"),
            0x5C => self.load_r_r("e", "h"),
            0x5D => self.load_r_r("e", "l"),
            0x5E => self.load_r_hl("e"),
            0x5F => self.load_r_r("e", "a"),

            0x60 => self.load_r_r("h", "b"),
            0x61 => self.load_r_r("h", "c"),
            0x62 => self.load_r_r("h", "d"),
            0x63 => self.load_r_r("h", "e"),
            0x64 => self.load_r_r("h", "h"),
            0x65 => self.load_r_r("h", "l"),
            0x66 => self.load_r_hl("h"),
            0x67 => self.load_r_r("h", "a"),
            0x68 => self.load_r_r("l", "b"),
            0x69 => self.load_r_r("l", "c"),
            0x6A => self.load_r_r("l", "d"),
            0x6B => self.load_r_r("l", "e"),
            0x6C => self.load_r_r("l", "h"),
            0x6D => self.load_r_r("l", "l"),
            0x6E => self.load_r_hl("l"),
            0x6F => self.load_r_r("l", "a"),

            0x70 => self.load_hl_r("b"),
            0x71 => self.load_hl_r("c"),
            0x72 => self.load_hl_r("d"),
            0x73 => self.load_hl_r("e"),
            0x74 => self.load_hl_r("h"),
            0x75 => self.load_hl_r("l"),
            0x76 => self.halt(),
            0x77 => self.load_hl_r("a"),
            0x78 => self.load_r_r("a", "b"),
            0x79 => self.load_r_r("a", "c"),
            0x7A => self.load_r_r("a", "d"),
            0x7B => self.load_r_r("a", "e"),
            0x7C => self.load_r_r("a", "h"),
            0x7D => self.load_r_r("a", "l"),
            0x7E => self.load_r_hl("a"),
            0x7F => self.load_r_r("a", "a"),

            0x80 => self.add_r("b"),
            0x81 => self.add_r("c"),
            0x82 => self.add_r("d"),
            0x83 => self.add_r("e"),
            0x84 => self.add_r("h"),
            0x85 => self.add_r("l"),
            0x86 => self.add_hl(),
            0x87 => self.add_r("a"),
            0x88 => self.adc_a_r("b"),
            0x89 => self.adc_a_r("c"),
            0x8A => self.adc_a_r("d"),
            0x8B => self.adc_a_r("e"),
            0x8C => self.adc_a_r("h"),
            0x8D => self.adc_a_r("l"),
            0x8E => self.adc_a_hl(),
            0x8F => self.adc_a_r("a"),

            0x90 => self.sub_r("b"),
            0x91 => self.sub_r("c"),
            0x92 => self.sub_r("d"),
            0x93 => self.sub_r("e"),
            0x94 => self.sub_r("h"),
            0x95 => self.sub_r("l"),
            0x96 => self.sub_hl(),
            0x97 => self.sub_r("a"),
            0x98 => self.sbc_a_r("b"),
            0x99 => self.sbc_a_r("c"),
            0x9A => self.sbc_a_r("d"),
            0x9B => self.sbc_a_r("e"),
            0x9C => self.sbc_a_r("h"),
            0x9D => self.sbc_a_r("l"),
            0x9E => self.sbc_a_hl(),
            0x9F => self.sbc_a_r("a"),

            0xA0 => self.and_r("b"),
            0xA1 => self.and_r("c"),
            0xA2 => self.and_r("d"),
            0xA3 => self.and_r("e"),
            0xA4 => self.and_r("h"),
            0xA5 => self.and_r("l"),
            0xA6 => self.and_hl(),
            0xA7 => self.and_r("a"),
            0xA8 => self.xor_r("b"),
            0xA9 => self.xor_r("c"),
            0xAA => self.xor_r("d"),
            0xAB => self.xor_r("e"),
            0xAC => self.xor_r("h"),
            0xAD => self.xor_r("l"),
            0xAE => self.xor_hl(),
            0xAF => self.xor_r("a"),

            0xB0 => self.or_r("b"),
            0xB1 => self.or_r("c"),
            0xB2 => self.or_r("d"),
            0xB3 => self.or_r("e"),
            0xB4 => self.or_r("h"),
            0xB5 => self.or_r("l"),
            0xB6 => self.or_hl(),
            0xB7 => self.or_r("a"),
            0xB8 => self.cp_r("b"),
            0xB9 => self.cp_r("c"),
            0xBA => self.cp_r("d"),
            0xBB => self.cp_r("e"),
            0xBC => self.cp_r("h"),
            0xBD => self.cp_r("l"),
            0xBE => self.cp_hl(),
            0xBF => self.cp_r("a"),

            0xC0 => self.ret_f("nz"),
            0xC1 => self.pop_rr("bc"),
            0xC2 => self.jp_f_nn("nz"),
            0xC3 => self.jp_nn(),
            0xC4 => self.call_f_nn("nz"),
            0xC5 => self.push_rr("bc"),
            0xC6 => self.add_n(),
            0xC7 => self.rst(0x00),
            0xC8 => self.ret_f("z"),
            0xC9 => self.ret(),
            0xCA => self.jp_f_nn("z"),
            0xCB => self.cb(),
            0xCC => self.call_f_nn("z"),
            0xCD => self.call_nn(),
            0xCE => self.adc_a_n(),
            0xCF => self.rst(0x08),

            0xD0 => self.ret_f("nc"),
            0xD1 => self.pop_rr("de"),
            0xD2 => self.jp_f_nn("nc"),
            0xD4 => self.call_f_nn("nc"),
            0xD5 => self.push_rr("de"),
            0xD6 => self.sub_n(),
            0xD7 => self.rst(0x10),
            0xD8 => self.ret_f("c"),
            0xD9 => self.reti(),
            0xDA => self.jp_f_nn("c"),
            0xDC => self.call_f_nn("c"),
            0xDE => self.sbc_a_n(),
            0xDF => self.rst(0x18),

            0xE0 => self.loadh_n_a(),
            0xE1 => self.pop_rr("hl"),
            0xE2 => self.load_c_a(),
            0xE5 => self.push_rr("hl"),
            0xE6 => self.and_n(),
            0xE7 => self.rst(0x20),
            0xE8 => self.add_sp_r8(),
            0xE9 => self.jp_hl(),
            0xEA => self.load_nn_a(),
            0xEE => self.xor_n(),
            0xEF => self.rst(0x28),

            0xF0 => self.loadh_a_n(),
            0xF1 => self.pop_rr("af"),
            0xF2 => self.load_a_c(),
            0xF3 => self.di(),
            0xF5 => self.push_rr("af"),
            0xF6 => self.or_n(),
            0xF7 => self.rst(0x30),
            0xF8 => self.ldhl_sp_r8(),
            0xF9 => self.load_sp_hl(),
            0xFA => self.load_a_nn(),
            0xFB => self.ei(),
            0xFE => self.cp_n(),
            0xFF => self.rst(0x38),

            _ => 0,
        }
    }

    fn cb(&mut self) -> u8 {
        let n = self.fetch_opcode();
        match n {
            0x00 => self.rlc_r("b"),
            0x01 => self.rlc_r("c"),
            0x02 => self.rlc_r("d"),
            0x03 => self.rlc_r("e"),
            0x04 => self.rlc_r("h"),
            0x05 => self.rlc_r("l"),
            0x06 => self.rlc_hl(),
            0x07 => self.rlc_r("a"),
            0x08 => self.rrc_r("b"),
            0x09 => self.rrc_r("c"),
            0x0A => self.rrc_r("d"),
            0x0B => self.rrc_r("e"),
            0x0C => self.rrc_r("h"),
            0x0D => self.rrc_r("l"),
            0x0E => self.rrc_hl(),
            0x0F => self.rrc_r("a"),

            0x10 => self.rl_r("b"),
            0x11 => self.rl_r("c"),
            0x12 => self.rl_r("d"),
            0x13 => self.rl_r("e"),
            0x14 => self.rl_r("h"),
            0x15 => self.rl_r("l"),
            0x16 => self.rl_hl(),
            0x17 => self.rl_r("a"),
            0x18 => self.rr_r("b"),
            0x19 => self.rr_r("c"),
            0x1A => self.rr_r("d"),
            0x1B => self.rr_r("e"),
            0x1C => self.rr_r("h"),
            0x1D => self.rr_r("l"),
            0x1E => self.rr_hl(),
            0x1F => self.rr_r("a"),

            0x20 => self.sla_r("b"),
            0x21 => self.sla_r("c"),
            0x22 => self.sla_r("d"),
            0x23 => self.sla_r("e"),
            0x24 => self.sla_r("h"),
            0x25 => self.sla_r("l"),
            0x26 => self.sla_hl(),
            0x27 => self.sla_r("a"),
            0x28 => self.sra_r("b"),
            0x29 => self.sra_r("c"),
            0x2A => self.sra_r("d"),
            0x2B => self.sra_r("e"),
            0x2C => self.sra_r("h"),
            0x2D => self.sra_r("l"),
            0x2E => self.sra_hl(),
            0x2F => self.sra_r("a"),

            0x30 => self.swap_r("b"),
            0x31 => self.swap_r("c"),
            0x32 => self.swap_r("d"),
            0x33 => self.swap_r("e"),
            0x34 => self.swap_r("h"),
            0x35 => self.swap_r("l"),
            0x36 => self.swap_hl(),
            0x37 => self.swap_r("a"),
            0x38 => self.srl_r("b"),
            0x39 => self.srl_r("c"),
            0x3A => self.srl_r("d"),
            0x3B => self.srl_r("e"),
            0x3C => self.srl_r("h"),
            0x3D => self.srl_r("l"),
            0x3E => self.srl_hl(),
            0x3F => self.srl_r("a"),

            0x40 => self.bit_n_r(0, "b"),
            0x41 => self.bit_n_r(0, "c"),
            0x42 => self.bit_n_r(0, "d"),
            0x43 => self.bit_n_r(0, "e"),
            0x44 => self.bit_n_r(0, "h"),
            0x45 => self.bit_n_r(0, "l"),
            0x46 => self.bit_n_hl(0),
            0x47 => self.bit_n_r(0, "a"),
            0x48 => self.bit_n_r(1, "b"),
            0x49 => self.bit_n_r(1, "c"),
            0x4A => self.bit_n_r(1, "d"),
            0x4B => self.bit_n_r(1, "e"),
            0x4C => self.bit_n_r(1, "h"),
            0x4D => self.bit_n_r(1, "l"),
            0x4E => self.bit_n_hl(1),
            0x4F => self.bit_n_r(1, "a"),

            0x50 => self.bit_n_r(2, "b"),
            0x51 => self.bit_n_r(2, "c"),
            0x52 => self.bit_n_r(2, "d"),
            0x53 => self.bit_n_r(2, "e"),
            0x54 => self.bit_n_r(2, "h"),
            0x55 => self.bit_n_r(2, "l"),
            0x56 => self.bit_n_hl(2),
            0x57 => self.bit_n_r(2, "a"),
            0x58 => self.bit_n_r(3, "b"),
            0x59 => self.bit_n_r(3, "c"),
            0x5A => self.bit_n_r(3, "d"),
            0x5B => self.bit_n_r(3, "e"),
            0x5C => self.bit_n_r(3, "h"),
            0x5D => self.bit_n_r(3, "l"),
            0x5E => self.bit_n_hl(3),
            0x5F => self.bit_n_r(3, "a"),

            0x60 => self.bit_n_r(4, "b"),
            0x61 => self.bit_n_r(4, "c"),
            0x62 => self.bit_n_r(4, "d"),
            0x63 => self.bit_n_r(4, "e"),
            0x64 => self.bit_n_r(4, "h"),
            0x65 => self.bit_n_r(4, "l"),
            0x66 => self.bit_n_hl(4),
            0x67 => self.bit_n_r(4, "a"),
            0x68 => self.bit_n_r(5, "b"),
            0x69 => self.bit_n_r(5, "c"),
            0x6A => self.bit_n_r(5, "d"),
            0x6B => self.bit_n_r(5, "e"),
            0x6C => self.bit_n_r(5, "h"),
            0x6D => self.bit_n_r(5, "l"),
            0x6E => self.bit_n_hl(5),
            0x6F => self.bit_n_r(5, "a"),

            0x70 => self.bit_n_r(6, "b"),
            0x71 => self.bit_n_r(6, "c"),
            0x72 => self.bit_n_r(6, "d"),
            0x73 => self.bit_n_r(6, "e"),
            0x74 => self.bit_n_r(6, "h"),
            0x75 => self.bit_n_r(6, "l"),
            0x76 => self.bit_n_hl(6),
            0x77 => self.bit_n_r(6, "a"),
            0x78 => self.bit_n_r(7, "b"),
            0x79 => self.bit_n_r(7, "c"),
            0x7A => self.bit_n_r(7, "d"),
            0x7B => self.bit_n_r(7, "e"),
            0x7C => self.bit_n_r(7, "h"),
            0x7D => self.bit_n_r(7, "l"),
            0x7E => self.bit_n_hl(7),
            0x7F => self.bit_n_r(7, "a"),

            0x80 => self.res_n_r(0, "b"),
            0x81 => self.res_n_r(0, "c"),
            0x82 => self.res_n_r(0, "d"),
            0x83 => self.res_n_r(0, "e"),
            0x84 => self.res_n_r(0, "h"),
            0x85 => self.res_n_r(0, "l"),
            0x86 => self.res_n_hl(0),
            0x87 => self.res_n_r(0, "a"),
            0x88 => self.res_n_r(1, "b"),
            0x89 => self.res_n_r(1, "c"),
            0x8A => self.res_n_r(1, "d"),
            0x8B => self.res_n_r(1, "e"),
            0x8C => self.res_n_r(1, "h"),
            0x8D => self.res_n_r(1, "l"),
            0x8E => self.res_n_hl(1),
            0x8F => self.res_n_r(1, "a"),

            0x90 => self.res_n_r(2, "b"),
            0x91 => self.res_n_r(2, "c"),
            0x92 => self.res_n_r(2, "d"),
            0x93 => self.res_n_r(2, "e"),
            0x94 => self.res_n_r(2, "h"),
            0x95 => self.res_n_r(2, "l"),
            0x96 => self.res_n_hl(2),
            0x97 => self.res_n_r(2, "a"),
            0x98 => self.res_n_r(3, "b"),
            0x99 => self.res_n_r(3, "c"),
            0x9A => self.res_n_r(3, "d"),
            0x9B => self.res_n_r(3, "e"),
            0x9C => self.res_n_r(3, "h"),
            0x9D => self.res_n_r(3, "l"),
            0x9E => self.res_n_hl(3),
            0x9F => self.res_n_r(3, "a"),

            0xA0 => self.res_n_r(4, "b"),
            0xA1 => self.res_n_r(4, "c"),
            0xA2 => self.res_n_r(4, "d"),
            0xA3 => self.res_n_r(4, "e"),
            0xA4 => self.res_n_r(4, "h"),
            0xA5 => self.res_n_r(4, "l"),
            0xA6 => self.res_n_hl(4),
            0xA7 => self.res_n_r(4, "a"),
            0xA8 => self.res_n_r(5, "b"),
            0xA9 => self.res_n_r(5, "c"),
            0xAA => self.res_n_r(5, "d"),
            0xAB => self.res_n_r(5, "e"),
            0xAC => self.res_n_r(5, "h"),
            0xAD => self.res_n_r(5, "l"),
            0xAE => self.res_n_hl(5),
            0xAF => self.res_n_r(5, "a"),

            0xB0 => self.res_n_r(6, "b"),
            0xB1 => self.res_n_r(6, "c"),
            0xB2 => self.res_n_r(6, "d"),
            0xB3 => self.res_n_r(6, "e"),
            0xB4 => self.res_n_r(6, "h"),
            0xB5 => self.res_n_r(6, "l"),
            0xB6 => self.res_n_hl(6),
            0xB7 => self.res_n_r(6, "a"),
            0xB8 => self.res_n_r(7, "b"),
            0xB9 => self.res_n_r(7, "c"),
            0xBA => self.res_n_r(7, "d"),
            0xBB => self.res_n_r(7, "e"),
            0xBC => self.res_n_r(7, "h"),
            0xBD => self.res_n_r(7, "l"),
            0xBE => self.res_n_hl(7),
            0xBF => self.res_n_r(7, "a"),

            0xC0 => self.set_n_r(0, "b"),
            0xC1 => self.set_n_r(0, "c"),
            0xC2 => self.set_n_r(0, "d"),
            0xC3 => self.set_n_r(0, "e"),
            0xC4 => self.set_n_r(0, "h"),
            0xC5 => self.set_n_r(0, "l"),
            0xC6 => self.set_n_hl(0),
            0xC7 => self.set_n_r(0, "a"),
            0xC8 => self.set_n_r(1, "b"),
            0xC9 => self.set_n_r(1, "c"),
            0xCA => self.set_n_r(1, "d"),
            0xCB => self.set_n_r(1, "e"),
            0xCC => self.set_n_r(1, "h"),
            0xCD => self.set_n_r(1, "l"),
            0xCE => self.set_n_hl(1),
            0xCF => self.set_n_r(1, "a"),

            0xD0 => self.set_n_r(2, "b"),
            0xD1 => self.set_n_r(2, "c"),
            0xD2 => self.set_n_r(2, "d"),
            0xD3 => self.set_n_r(2, "e"),
            0xD4 => self.set_n_r(2, "h"),
            0xD5 => self.set_n_r(2, "l"),
            0xD6 => self.set_n_hl(2),
            0xD7 => self.set_n_r(2, "a"),
            0xD8 => self.set_n_r(3, "b"),
            0xD9 => self.set_n_r(3, "c"),
            0xDA => self.set_n_r(3, "d"),
            0xDB => self.set_n_r(3, "e"),
            0xDC => self.set_n_r(3, "h"),
            0xDD => self.set_n_r(3, "l"),
            0xDE => self.set_n_hl(3),
            0xDF => self.set_n_r(3, "a"),

            0xE0 => self.set_n_r(4, "b"),
            0xE1 => self.set_n_r(4, "c"),
            0xE2 => self.set_n_r(4, "d"),
            0xE3 => self.set_n_r(4, "e"),
            0xE4 => self.set_n_r(4, "h"),
            0xE5 => self.set_n_r(4, "l"),
            0xE6 => self.set_n_hl(4),
            0xE7 => self.set_n_r(4, "a"),
            0xE8 => self.set_n_r(5, "b"),
            0xE9 => self.set_n_r(5, "c"),
            0xEA => self.set_n_r(5, "d"),
            0xEB => self.set_n_r(5, "e"),
            0xEC => self.set_n_r(5, "h"),
            0xED => self.set_n_r(5, "l"),
            0xEE => self.set_n_hl(5),
            0xEF => self.set_n_r(5, "a"),

            0xF0 => self.set_n_r(6, "b"),
            0xF1 => self.set_n_r(6, "c"),
            0xF2 => self.set_n_r(6, "d"),
            0xF3 => self.set_n_r(6, "e"),
            0xF4 => self.set_n_r(6, "h"),
            0xF5 => self.set_n_r(6, "l"),
            0xF6 => self.set_n_hl(6),
            0xF7 => self.set_n_r(6, "a"),
            0xF8 => self.set_n_r(7, "b"),
            0xF9 => self.set_n_r(7, "c"),
            0xFA => self.set_n_r(7, "d"),
            0xFB => self.set_n_r(7, "e"),
            0xFC => self.set_n_r(7, "h"),
            0xFD => self.set_n_r(7, "l"),
            0xFE => self.set_n_hl(7),
            0xFF => self.set_n_r(7, "a"),
        }
    }

    fn read_nn(&mut self) -> u16 {
        let nn = self.memory.read_word(self.cpu.get_pc());
        self.cpu.set_pc(self.cpu.get_pc() + 2);
        nn
    }
    
    //GMB 8bit-Loadcommands
    // ld r,r
    pub fn load_r_r(&mut self, r1: &str, r2: &str) -> u8 {
        let r2 = self.cpu.get_r(r2) as u16;
        self.cpu.set_r(r1, self.memory.read_byte(r2));
        4
    }

    // ld r,n
    pub fn load_r_n(&mut self, r: &str) -> u8 {
        let n = self.fetch_opcode();
        self.cpu.set_r(r, n);
        8
    }

    // ld r,(hl)
    pub fn load_r_hl(&mut self, r: &str) -> u8 {
        let hl = self.cpu.get_rr("hl");
        self.cpu.set_r(r, self.memory.read_byte(hl));
        8
    }

    // ld (hl),r
    pub fn load_hl_r(&mut self, r: &str) -> u8 {
        let r = self.cpu.get_r(r);
        let hl = self.cpu.get_rr("hl");
        self.memory.write_byte(hl, r);
        8
    }

    // ld (hl),n
    pub fn load_hl_n(&mut self) -> u8 {
        let hl = self.cpu.get_rr("hl");
        let n = self.fetch_opcode();
        self.memory.write_byte(hl, n);
        12
    }

    // ld a,(bc)
    pub fn load_a_bc(&mut self) -> u8 {
        let bc = self.cpu.get_rr("bc");
        self.cpu.set_r("a", self.memory.read_byte(bc));
        8
    }
    
    // ld a,(de)
    pub fn load_a_de(&mut self) -> u8 {
        let de = self.cpu.get_rr("de");
        self.cpu.set_r("a", self.memory.read_byte(de));
        8
    }

    // ld a,(nn)
    pub fn load_a_nn(&mut self) -> u8 {
        let nn = self.read_nn();
        self.cpu.set_r("a", self.memory.read_byte(nn));
        16
    }

    // ld (bc),a
    pub fn load_bc_a(&mut self) -> u8 {
        let bc = self.cpu.get_rr("bc");
        let a = self.cpu.get_r("a");
        self.memory.write_byte(bc, a);
        8
    }

    // ld (de),a
    pub fn load_de_a(&mut self) -> u8 {
        let de = self.cpu.get_rr("de");
        let a = self.cpu.get_r("a");
        self.memory.write_byte(de, a);
        8
    }

    // ld (nn),a
    pub fn load_nn_a(&mut self) -> u8 {
        let nn = self.read_nn();
        let a = self.cpu.get_r("a");
        self.memory.write_byte(nn, a);
        16
    }

    // LDH A,(a8) or ld A,(FF00+n)
    pub fn loadh_a_n(&mut self) -> u8 {
        let n = self.fetch_opcode();
        let addr = 0xFF00 + n as u16;
        self.cpu.set_r("a", self.memory.read_byte(addr));
        12
    }

    // LDH (a8),A or ld (FF00+n),A
    pub fn loadh_n_a(&mut self) -> u8 {
        let n = self.fetch_opcode();
        let addr = 0xFF00 + n as u16;
        let a = self.cpu.get_r("a");
        self.memory.write_byte(addr, a);
        12
    }

    // ld a,(c) or ld A,(FF00+C)
    pub fn load_a_c(&mut self) -> u8 {
        let c = self.cpu.get_r("c");
        let addr = 0xFF00 + c as u16;
        self.cpu.set_r("a", self.memory.read_byte(addr));
        8
    }

    // ld (c),a or ld (FF00+C),A
    pub fn load_c_a(&mut self) -> u8 {
        let c = self.cpu.get_r("c");
        let addr = 0xFF00 + c as u16;
        let a = self.cpu.get_r("a");
        self.memory.write_byte(addr, a);
        8
    }

    // LD (HL+),A or LD (HLI),A or LDI (HL),A 
    pub fn load_hli_a(&mut self) -> u8 {
        let hl = self.cpu.get_rr("hl");
        let a = self.cpu.get_r("a");
        self.memory.write_byte(hl, a);
        self.cpu.set_rr("hl", hl + 1);
        8
    }

    // LD A,(HL+) or LD A,(HLI) or LDI A,(HL)
    pub fn load_a_hli(&mut self) -> u8 {
        let hl = self.cpu.get_rr("hl");
        self.cpu.set_r("a", self.memory.read_byte(hl));
        self.cpu.set_rr("hl", hl + 1);
        8
    }

    // LD (HL-),A or LD (HLD),A or LDD (HL),A 
    pub fn load_hld_a(&mut self) -> u8 {
        let hl = self.cpu.get_rr("hl");
        let a = self.cpu.get_r("a");
        self.memory.write_byte(hl, a);
        self.cpu.set_rr("hl", hl - 1);
        8
    }

    // LD A,(HL-) or LD A,(HLD) or LDD A,(HL)
    pub fn load_a_hld(&mut self) -> u8 {
        let hl = self.cpu.get_rr("hl");
        self.cpu.set_r("a", self.memory.read_byte(hl));
        self.cpu.set_rr("hl", hl - 1);
        8
    }

    // GMB 16bit-Loadcommands
    // ld rr,nn
    pub fn load_rr_nn(&mut self, rr: &str) -> u8 {
        let nn = self.read_nn();
        self.cpu.set_rr(rr, nn);
        12
    }

    // ld sp,hl
    pub fn load_sp_hl(&mut self) -> u8 {
        let hl = self.cpu.get_rr("hl");
        self.cpu.set_rr("sp", hl);
        8
    }

    // push rr
    pub fn push_rr(&mut self, rr: &str) -> u8 {
        let sp = self.cpu.get_rr("sp");
        let value = self.cpu.get_rr(rr);
        self.memory.write_word(sp - 2, value);
        self.cpu.set_rr("sp", sp - 2);
        16
    }

    // pop rr
    pub fn pop_rr(&mut self, rr: &str) -> u8 {
        let sp = self.cpu.get_rr("sp");
        let value = self.memory.read_word(sp);
        if rr == "af" {
            self.cpu.set_rr(rr, value & 0xFFF0);
        } else {
            self.cpu.set_rr(rr, value);
        }
        self.cpu.set_rr("sp", sp + 2);
        12
    }

    //GMB 8bit-Arithmetic/logical Commands
    // add a,r
    pub fn add_r(&mut self, r: &str) -> u8 {
        let a = self.cpu.get_r("a");
        let value = self.cpu.get_r(r);
        let result = a.overflowing_add(value);
        self.cpu.set_r("a", result.0);
        self.cpu.set_flag("z", result.0 == 0);
        self.cpu.set_flag("n", false);
        self.cpu.set_flag("h", result.0 & 0x10 == 0x10);
        self.cpu.set_flag("c", result.1);
        4
    }

    // add a,n
    pub fn add_n(&mut self) -> u8 {
        let a = self.cpu.get_r("a");
        let n = self.fetch_opcode();
        let result = a.overflowing_add(n);
        self.cpu.set_r("a", result.0);
        self.cpu.set_flag("z", result.0 == 0);
        self.cpu.set_flag("n", false);
        self.cpu.set_flag("h", result.0 & 0x10 == 0x10);
        self.cpu.set_flag("c", result.1);
        8
    }

    // add a,(hl)
    pub fn add_hl(&mut self) -> u8 {
        let a = self.cpu.get_r("a");
        let hl = self.cpu.get_rr("hl");
        let value = self.memory.read_byte(hl);
        let result = a.overflowing_add(value);
        self.cpu.set_r("a", result.0);
        self.cpu.set_flag("z", result.0 == 0);
        self.cpu.set_flag("n", false);
        self.cpu.set_flag("h", result.0 & 0x10 == 0x10);
        self.cpu.set_flag("c", result.1);
        8
    }

    // adc a,r
    pub fn adc_a_r(&mut self, r: &str) -> u8 {
        let a = self.cpu.get_r("a");
        let value = self.cpu.get_r(r);
        let carry = self.cpu.get_flag("c");
        let result = a.overflowing_add(value);
        let result = result.0.overflowing_add(carry as u8);
        self.cpu.set_r("a", result.0);
        self.cpu.set_flag("z", result.0 == 0);
        self.cpu.set_flag("n", false);
        self.cpu.set_flag("h", result.0 & 0x10 == 0x10);
        self.cpu.set_flag("c", result.1);
        4
    }

    // adc a,n
    pub fn adc_a_n(&mut self) -> u8 {
        let a = self.cpu.get_r("a");
        let n = self.fetch_opcode();
        let carry = self.cpu.get_flag("c");
        let result = a.overflowing_add(n);
        let result = result.0.overflowing_add(carry as u8);
        self.cpu.set_r("a", result.0);
        self.cpu.set_flag("z", result.0 == 0);
        self.cpu.set_flag("n", false);
        self.cpu.set_flag("h", result.0 & 0x10 == 0x10);
        self.cpu.set_flag("c", result.1);
        8
    }

    // adc a,(hl)
    pub fn adc_a_hl(&mut self) -> u8 {
        let a = self.cpu.get_r("a");
        let hl = self.cpu.get_rr("hl");
        let value = self.memory.read_byte(hl);
        let carry = self.cpu.get_flag("c");
        let result = a.overflowing_add(value);
        let result = result.0.overflowing_add(carry as u8);
        self.cpu.set_r("a", result.0);
        self.cpu.set_flag("z", result.0 == 0);
        self.cpu.set_flag("n", false);
        self.cpu.set_flag("h", result.0 & 0x10 == 0x10);
        self.cpu.set_flag("c", result.1);
        8
    }

    // sub r
    pub fn sub_r(&mut self, r: &str) -> u8 {
        let a = self.cpu.get_r("a");
        let value = self.cpu.get_r(r);
        let result = a.overflowing_sub(value);
        self.cpu.set_r("a", result.0);
        self.cpu.set_flag("z", result.0 == 0);
        self.cpu.set_flag("n", true);
        self.cpu.set_flag("h", result.0 & 0x10 == 0x10);
        self.cpu.set_flag("c", result.1);
        4
    }

    // sub n
    pub fn sub_n(&mut self) -> u8 {
        let a = self.cpu.get_r("a");
        let n = self.fetch_opcode();
        let result = a.overflowing_sub(n);
        self.cpu.set_r("a", result.0);
        self.cpu.set_flag("z", result.0 == 0);
        self.cpu.set_flag("n", true);
        self.cpu.set_flag("h", result.0 & 0x10 == 0x10);
        self.cpu.set_flag("c", result.1);
        8
    }

    // sub (hl)
    pub fn sub_hl(&mut self) -> u8 {
        let a = self.cpu.get_r("a");
        let hl = self.cpu.get_rr("hl");
        let value = self.memory.read_byte(hl);
        let result = a.overflowing_sub(value);
        self.cpu.set_r("a", result.0);
        self.cpu.set_flag("z", result.0 == 0);
        self.cpu.set_flag("n", true);
        self.cpu.set_flag("h", result.0 & 0x10 == 0x10);
        self.cpu.set_flag("c", result.1);
        8
    }

    // sbc a,r
    pub fn sbc_a_r(&mut self, r: &str) -> u8 {
        let a = self.cpu.get_r("a");
        let value = self.cpu.get_r(r);
        let carry = self.cpu.get_flag("c");
        let result = a.overflowing_sub(value);
        let result = result.0.overflowing_sub(carry as u8);
        self.cpu.set_r("a", result.0);
        self.cpu.set_flag("z", result.0 == 0);
        self.cpu.set_flag("n", true);
        self.cpu.set_flag("h", result.0 & 0x10 == 0x10);
        self.cpu.set_flag("c", result.1);
        4
    }

    // sbc a,n
    pub fn sbc_a_n(&mut self) -> u8 {
        let a = self.cpu.get_r("a");
        let n = self.fetch_opcode();
        let carry = self.cpu.get_flag("c");
        let result = a.overflowing_sub(n);
        let result = result.0.overflowing_sub(carry as u8);
        self.cpu.set_r("a", result.0);
        self.cpu.set_flag("z", result.0 == 0);
        self.cpu.set_flag("n", true);
        self.cpu.set_flag("h", result.0 & 0x10 == 0x10);
        self.cpu.set_flag("c", result.1);
        8
    }

    // sbc a,(hl)
    pub fn sbc_a_hl(&mut self) -> u8 {
        let a = self.cpu.get_r("a");
        let hl = self.cpu.get_rr("hl");
        let value = self.memory.read_byte(hl);
        let carry = self.cpu.get_flag("c");
        let result = a.overflowing_sub(value);
        let result = result.0.overflowing_sub(carry as u8);
        self.cpu.set_r("a", result.0);
        self.cpu.set_flag("z", result.0 == 0);
        self.cpu.set_flag("n", true);
        self.cpu.set_flag("h", result.0 & 0x10 == 0x10);
        self.cpu.set_flag("c", result.1);
        8
    }

    // and r
    pub fn and_r(&mut self, r: &str) -> u8 {
        let a = self.cpu.get_r("a");
        let value = self.cpu.get_r(r);
        let result = a & value;
        self.cpu.set_r("a", result);
        self.cpu.set_flag("z", result == 0);
        self.cpu.set_flag("n", false);
        self.cpu.set_flag("h", true);
        self.cpu.set_flag("c", false);
        4
    }

    // and n
    pub fn and_n(&mut self) -> u8 {
        let a = self.cpu.get_r("a");
        let n = self.fetch_opcode();
        let result = a & n;
        self.cpu.set_r("a", result);
        self.cpu.set_flag("z", result == 0);
        self.cpu.set_flag("n", false);
        self.cpu.set_flag("h", true);
        self.cpu.set_flag("c", false);
        8
    }

    // and (hl)
    pub fn and_hl(&mut self) -> u8 {
        let a = self.cpu.get_r("a");
        let hl = self.cpu.get_rr("hl");
        let value = self.memory.read_byte(hl);
        let result = a & value;
        self.cpu.set_r("a", result);
        self.cpu.set_flag("z", result == 0);
        self.cpu.set_flag("n", false);
        self.cpu.set_flag("h", true);
        self.cpu.set_flag("c", false);
        8
    }

    // xor r
    pub fn xor_r(&mut self, r: &str) -> u8 {
        let a = self.cpu.get_r("a");
        let value = self.cpu.get_r(r);
        let result = a ^ value;
        self.cpu.set_r("a", result);
        self.cpu.set_flag("z", result == 0);
        self.cpu.set_flag("n", false);
        self.cpu.set_flag("h", false);
        self.cpu.set_flag("c", false);
        4
    }

    // xor n
    pub fn xor_n(&mut self) -> u8 {
        let a = self.cpu.get_r("a");
        let n = self.fetch_opcode();
        let result = a ^ n;
        self.cpu.set_r("a", result);
        self.cpu.set_flag("z", result == 0);
        self.cpu.set_flag("n", false);
        self.cpu.set_flag("h", false);
        self.cpu.set_flag("c", false);
        8
    }

    // xor (hl)
    pub fn xor_hl(&mut self) -> u8 {
        let a = self.cpu.get_r("a");
        let hl = self.cpu.get_rr("hl");
        let value = self.memory.read_byte(hl);
        let result = a ^ value;
        self.cpu.set_r("a", result);
        self.cpu.set_flag("z", result == 0);
        self.cpu.set_flag("n", false);
        self.cpu.set_flag("h", false);
        self.cpu.set_flag("c", false);
        8
    }

    // or r
    pub fn or_r(&mut self, r: &str) -> u8 {
        let a = self.cpu.get_r("a");
        let value = self.cpu.get_r(r);
        let result = a | value;
        self.cpu.set_r("a", result);
        self.cpu.set_flag("z", result == 0);
        self.cpu.set_flag("n", false);
        self.cpu.set_flag("h", false);
        self.cpu.set_flag("c", false);
        4
    }

    // or n
    pub fn or_n(&mut self) -> u8 {
        let a = self.cpu.get_r("a");
        let n = self.fetch_opcode();
        let result = a | n;
        self.cpu.set_r("a", result);
        self.cpu.set_flag("z", result == 0);
        self.cpu.set_flag("n", false);
        self.cpu.set_flag("h", false);
        self.cpu.set_flag("c", false);
        8
    }

    // or (hl)
    pub fn or_hl(&mut self) -> u8 {
        let a = self.cpu.get_r("a");
        let hl = self.cpu.get_rr("hl");
        let value = self.memory.read_byte(hl);
        let result = a | value;
        self.cpu.set_r("a", result);
        self.cpu.set_flag("z", result == 0);
        self.cpu.set_flag("n", false);
        self.cpu.set_flag("h", false);
        self.cpu.set_flag("c", false);
        8
    }

    // cp r
    pub fn cp_r(&mut self, r: &str) -> u8 {
        let a = self.cpu.get_r("a");
        let value = self.cpu.get_r(r);
        let result = a.overflowing_sub(value);
        self.cpu.set_flag("z", result.0 == 0);
        self.cpu.set_flag("n", true);
        self.cpu.set_flag("h", result.0 & 0x10 == 0x10);
        self.cpu.set_flag("c", result.1);
        4
    }

    // cp n
    pub fn cp_n(&mut self) -> u8 {
        let a = self.cpu.get_r("a");
        let n = self.fetch_opcode();
        let result = a.overflowing_sub(n);
        self.cpu.set_flag("z", result.0 == 0);
        self.cpu.set_flag("n", true);
        self.cpu.set_flag("h", result.0 & 0x10 == 0x10);
        self.cpu.set_flag("c", result.1);
        8
    }

    // cp (hl)
    pub fn cp_hl(&mut self) -> u8 {
        let a = self.cpu.get_r("a");
        let hl = self.cpu.get_rr("hl");
        let value = self.memory.read_byte(hl);
        let result = a.overflowing_sub(value);
        self.cpu.set_flag("z", result.0 == 0);
        self.cpu.set_flag("n", true);
        self.cpu.set_flag("h", result.0 & 0x10 == 0x10);
        self.cpu.set_flag("c", result.1);
        8
    }

    // inc r
    pub fn inc_r(&mut self, r: &str) -> u8 {
        let value = self.cpu.get_r(r);
        let result = value.wrapping_add(1);
        self.cpu.set_r(r, result);
        self.cpu.set_flag("z", result == 0);
        self.cpu.set_flag("n", false);
        self.cpu.set_flag("h", result & 0x10 == 0x10);
        4
    }

    // inc (hl)
    pub fn inc_hl(&mut self) -> u8 {
        let hl = self.cpu.get_rr("hl");
        let value = self.memory.read_byte(hl);
        let result = value.wrapping_add(1);
        self.memory.write_byte(hl, result);
        self.cpu.set_flag("z", result == 0);
        self.cpu.set_flag("n", false);
        self.cpu.set_flag("h", result & 0x10 == 0x10);
        12
    }

    // dec r
    pub fn dec_r(&mut self, r: &str) -> u8 {
        let value = self.cpu.get_r(r);
        let result = value.wrapping_sub(1);
        self.cpu.set_r(r, result);
        self.cpu.set_flag("z", result == 0);
        self.cpu.set_flag("n", true);
        self.cpu.set_flag("h", result & 0x10 == 0x10);
        4
    }

    // dec (hl)
    pub fn dec_hl(&mut self) -> u8 {
        let hl = self.cpu.get_rr("hl");
        let value = self.memory.read_byte(hl);
        let result = value.wrapping_sub(1);
        self.memory.write_byte(hl, result);
        self.cpu.set_flag("z", result == 0);
        self.cpu.set_flag("n", true);
        self.cpu.set_flag("h", result & 0x10 == 0x10);
        12
    }

    // daa
    pub fn daa(&mut self) -> u8 {
        //TODO: implement
        //self.memory.dump_vram();
        //panic!("daa not implemented");
        4
    }

    // cpl
    pub fn cpl(&mut self) -> u8 {
        let a = self.cpu.get_r("a");
        let result = a ^ 0xFF;	
        self.cpu.set_r("a", result);
        self.cpu.set_flag("n", true);
        self.cpu.set_flag("h", true);
        4
    }

    // GMB 16bit-Arithmetic/logical Commands
    // add hl, rr
    pub fn add_hl_rr(&mut self, rr: &str) -> u8 {
        let hl = self.cpu.get_rr("hl");
        let value = self.cpu.get_rr(rr);
        let result = hl.overflowing_add(value);
        self.cpu.set_rr("hl", result.0);
        self.cpu.set_flag("n", false);
        self.cpu.set_flag("h", result.0 & 0x1000 == 0x1000);
        self.cpu.set_flag("c", result.1);
        8
    }

    // inc rr
    pub fn inc_rr(&mut self, rr: &str) -> u8 {
        let value = self.cpu.get_rr(rr);
        let result = value.wrapping_add(1);
        self.cpu.set_rr(rr, result);
        8
    }

    // dec rr
    pub fn dec_rr(&mut self, rr: &str) -> u8 {
        let value = self.cpu.get_rr(rr);
        let result = value.wrapping_sub(1);
        self.cpu.set_rr(rr, result);
        8
    }

    // ADD SP,r8
    pub fn add_sp_r8(&mut self) -> u8 {
        let sp = self.cpu.get_rr("sp");
        let r8 = self.fetch_opcode() as u16;
        let result = sp.overflowing_add(r8);
        self.cpu.set_rr("sp", result.0);
        self.cpu.set_flag("z", false);
        self.cpu.set_flag("n", false);
        self.cpu.set_flag("h", result.0 & 0x10 == 0x10);
        self.cpu.set_flag("c", result.1);
        16
    }

    // LD HL,SP+r8 or LDHL SP,r8
    pub fn ldhl_sp_r8(&mut self) -> u8 {
        let sp = self.cpu.get_rr("sp");
        let r8 = self.fetch_opcode() as u16;
        let result = sp.overflowing_add(r8);
        self.cpu.set_rr("hl", result.0);
        self.cpu.set_flag("z", false);
        self.cpu.set_flag("n", false);
        self.cpu.set_flag("h", result.0 & 0x10 == 0x10);
        self.cpu.set_flag("c", result.1);
        12
    }

    // GMB Rotate- und Shift-Commands
    // rlca
    pub fn rlca(&mut self) -> u8 {
        let a = self.cpu.get_r("a");
        let res = a.rotate_left(1);
        self.cpu.set_r("a", res);
        self.cpu.set_flag("z", false);
        self.cpu.set_flag("n", false);
        self.cpu.set_flag("h", false);
        self.cpu.set_flag("c", a & 0x80 == 0x80);
        4
    }

    // rla
    pub fn rla(&mut self) -> u8 {
        let a = self.cpu.get_r("a");
        let res = a.rotate_left(1);
        if self.cpu.get_flag("c") {
            self.cpu.set_r("a", res | 0x01);
        } else {
            self.cpu.set_r("a", res & 0xFE);
        }
        self.cpu.set_r("a", res);
        self.cpu.set_flag("z", false);
        self.cpu.set_flag("n", false);
        self.cpu.set_flag("h", false);
        self.cpu.set_flag("c", a & 0x80 == 0x80);
        4
    }

    // rrca
    pub fn rrca(&mut self) -> u8 {
        let a = self.cpu.get_r("a");
        let res = a.rotate_right(1);
        self.cpu.set_r("a", res);
        self.cpu.set_flag("z", false);
        self.cpu.set_flag("n", false);
        self.cpu.set_flag("h", false);
        self.cpu.set_flag("c", a & 0x01 == 0x01);
        4
    }

    // rra
    pub fn rra(&mut self) -> u8 {
        let a = self.cpu.get_r("a");
        let res = a.rotate_right(1);
        if self.cpu.get_flag("c") {
            self.cpu.set_r("a", res | 0x80);
        } else {
            self.cpu.set_r("a", res & 0x7F);
        }
        self.cpu.set_r("a", res);
        self.cpu.set_flag("z", false);
        self.cpu.set_flag("n", false);
        self.cpu.set_flag("h", false);
        self.cpu.set_flag("c", a & 0x01 == 0x01);
        4
    }

    // rlc r
    pub fn rlc_r(&mut self, r: &str) -> u8 {
        let value = self.cpu.get_r(r);
        let result = value.rotate_left(1);
        self.cpu.set_r(r, result);
        self.cpu.set_flag("z", result == 0);
        self.cpu.set_flag("n", false);
        self.cpu.set_flag("h", false);
        self.cpu.set_flag("c", value & 0x80 == 0x80);
        8
    }

    // rlc (hl)
    pub fn rlc_hl(&mut self) -> u8 {
        let hl = self.cpu.get_rr("hl");
        let value = self.memory.read_byte(hl);
        let result = value.rotate_left(1);
        self.memory.write_byte(hl, result);
        self.cpu.set_flag("z", result == 0);
        self.cpu.set_flag("n", false);
        self.cpu.set_flag("h", false);
        self.cpu.set_flag("c", value & 0x80 == 0x80);
        16
    }

    // rl r
    pub fn rl_r(&mut self, r: &str) -> u8 {
        let value = self.cpu.get_r(r);
        let result = value.rotate_left(1);
        if self.cpu.get_flag("c") {
            self.cpu.set_r(r, result | 0x01);
        } else {
            self.cpu.set_r(r, result & 0xFE);
        }
        self.cpu.set_flag("z", result == 0);
        self.cpu.set_flag("n", false);
        self.cpu.set_flag("h", false);
        self.cpu.set_flag("c", value & 0x80 == 0x80);
        8
    }

    // rl (hl)
    pub fn rl_hl(&mut self) -> u8 {
        let hl = self.cpu.get_rr("hl");
        let value = self.memory.read_byte(hl);
        let result = value.rotate_left(1);
        if self.cpu.get_flag("c") {
            self.memory.write_byte(hl, result | 0x01);
        } else {
            self.memory.write_byte(hl, result & 0xFE);
        }
        self.cpu.set_flag("z", result == 0);
        self.cpu.set_flag("n", false);
        self.cpu.set_flag("h", false);
        self.cpu.set_flag("c", value & 0x80 == 0x80);
        16
    }

    // rrc r
    pub fn rrc_r(&mut self, r: &str) -> u8 {
        let value = self.cpu.get_r(r);
        let result = value.rotate_right(1);
        self.cpu.set_r(r, result);
        self.cpu.set_flag("z", result == 0);
        self.cpu.set_flag("n", false);
        self.cpu.set_flag("h", false);
        self.cpu.set_flag("c", value & 0x01 == 0x01);
        8
    }

    // rrc (hl)
    pub fn rrc_hl(&mut self) -> u8 {
        let hl = self.cpu.get_rr("hl");
        let value = self.memory.read_byte(hl);
        let result = value.rotate_right(1);
        self.memory.write_byte(hl, result);
        self.cpu.set_flag("z", result == 0);
        self.cpu.set_flag("n", false);
        self.cpu.set_flag("h", false);
        self.cpu.set_flag("c", value & 0x01 == 0x01);
        16
    }

    // rr r
    pub fn rr_r(&mut self, r: &str) -> u8 {
        let value = self.cpu.get_r(r);
        let result = value.rotate_right(1);
        if self.cpu.get_flag("c") {
            self.cpu.set_r(r, result | 0x80);
        } else {
            self.cpu.set_r(r, result & 0x7F);
        }
        self.cpu.set_flag("z", result == 0);
        self.cpu.set_flag("n", false);
        self.cpu.set_flag("h", false);
        self.cpu.set_flag("c", value & 0x01 == 0x01);
        8
    }

    // rr (hl)
    pub fn rr_hl(&mut self) -> u8 {
        let hl = self.cpu.get_rr("hl");
        let value = self.memory.read_byte(hl);
        let result = value.rotate_right(1);
        if self.cpu.get_flag("c") {
            self.memory.write_byte(hl, result | 0x80);
        } else {
            self.memory.write_byte(hl, result & 0x7F);
        }
        self.cpu.set_flag("z", result == 0);
        self.cpu.set_flag("n", false);
        self.cpu.set_flag("h", false);
        self.cpu.set_flag("c", value & 0x01 == 0x01);
        16
    }

    // sla r
    pub fn sla_r(&mut self, r: &str) -> u8 {
        let value = self.cpu.get_r(r);
        let result = value.overflowing_shl(1);
        self.cpu.set_r(r, result.0);
        self.cpu.set_flag("z", result.0 == 0);
        self.cpu.set_flag("n", false);
        self.cpu.set_flag("h", false);
        self.cpu.set_flag("c", result.1);
        8
    }

    // sla (hl)
    pub fn sla_hl(&mut self) -> u8 {
        let hl = self.cpu.get_rr("hl");
        let value = self.memory.read_byte(hl);
        let result = value.overflowing_shl(1);
        self.memory.write_byte(hl, result.0);
        self.cpu.set_flag("z", result.0 == 0);
        self.cpu.set_flag("n", false);
        self.cpu.set_flag("h", false);
        self.cpu.set_flag("c", result.1);
        16
    }

    // swap r
    pub fn swap_r(&mut self, r: &str) -> u8 {
        let value = self.cpu.get_r(r);
        let result = value.reverse_bits();
        self.cpu.set_r(r, result);
        self.cpu.set_flag("z", result == 0);
        self.cpu.set_flag("n", false);
        self.cpu.set_flag("h", false);
        self.cpu.set_flag("c", false);
        8
    }

    // swap (hl)
    pub fn swap_hl(&mut self) -> u8 {
        let hl = self.cpu.get_rr("hl");
        let value = self.memory.read_byte(hl);
        let result = value.reverse_bits();
        self.memory.write_byte(hl, result);
        self.cpu.set_flag("z", result == 0);
        self.cpu.set_flag("n", false);
        self.cpu.set_flag("h", false);
        self.cpu.set_flag("c", false);
        16
    }

    // sra r
    pub fn sra_r(&mut self, r: &str) -> u8 {
        let value = self.cpu.get_r(r);
        let result = value.overflowing_shr(1);
        self.cpu.set_r(r, result.0);
        self.cpu.set_flag("z", result.0 == 0);
        self.cpu.set_flag("n", false);
        self.cpu.set_flag("h", false);
        self.cpu.set_flag("c", result.1);
        8
    }

    // sra (hl)
    pub fn sra_hl(&mut self) -> u8 {
        let hl = self.cpu.get_rr("hl");
        let value = self.memory.read_byte(hl);
        let result = value.overflowing_shr(1);
        self.memory.write_byte(hl, result.0);
        self.cpu.set_flag("z", result.0 == 0);
        self.cpu.set_flag("n", false);
        self.cpu.set_flag("h", false);
        self.cpu.set_flag("c", result.1);
        16
    }

    // srl r
    pub fn srl_r(&mut self, r: &str) -> u8 {
        let value = self.cpu.get_r(r);
        let result = value.overflowing_shr(1);
        self.cpu.set_r(r, result.0);
        self.cpu.set_flag("z", result.0 == 0);
        self.cpu.set_flag("n", false);
        self.cpu.set_flag("h", false);
        self.cpu.set_flag("c", value & 0x01 == 0x01);
        8
    }

    // srl (hl)
    pub fn srl_hl(&mut self) -> u8 {
        let hl = self.cpu.get_rr("hl");
        let value = self.memory.read_byte(hl);
        let result = value.overflowing_shr(1);
        self.memory.write_byte(hl, result.0);
        self.cpu.set_flag("z", result.0 == 0);
        self.cpu.set_flag("n", false);
        self.cpu.set_flag("h", false);
        self.cpu.set_flag("c", value & 0x01 == 0x01);
        16
    }

    // GMB Singlebit Operation Commands
    // bit  n,r
    pub fn bit_n_r(&mut self, n: u8, r: &str) -> u8 {
        let value = self.cpu.get_r(r);
        self.cpu.set_flag("z", value & (1 << n) == 0);
        self.cpu.set_flag("n", false);
        self.cpu.set_flag("h", true);
        8
    }

    // bit  n,(hl)
    pub fn bit_n_hl(&mut self, n: u8) -> u8 {
        let hl = self.cpu.get_rr("hl");
        let value = self.memory.read_byte(hl);
        self.cpu.set_flag("z", value & (1 << n) == 0);
        self.cpu.set_flag("n", false);
        self.cpu.set_flag("h", true);
        12
    }

    // set  n,r
    pub fn set_n_r(&mut self, n: u8, r: &str) -> u8 {
        let value = self.cpu.get_r(r);
        self.cpu.set_r(r, value | (1 << n));
        8
    }

    // set  n,(hl)
    pub fn set_n_hl(&mut self, n: u8) -> u8 {
        let hl = self.cpu.get_rr("hl");
        let value = self.memory.read_byte(hl);
        self.memory.write_byte(hl, value | (1 << n));
        16
    }

    // res n,r
    pub fn res_n_r(&mut self, n: u8, r: &str) -> u8 {
        let value = self.cpu.get_r(r);
        self.cpu.set_r(r, value & !(1 << n));
        8
    }

    // res n,(hl)
    pub fn res_n_hl(&mut self, n: u8) -> u8 {
        let hl = self.cpu.get_rr("hl");
        let value = self.memory.read_byte(hl);
        self.memory.write_byte(hl, value & !(1 << n));
        16
    }

    // GMB CPU-Controlcommands
    // ccf
    pub fn ccf(&mut self) -> u8 {
        let c = self.cpu.get_flag("c");
        self.cpu.set_flag("n", false);
        self.cpu.set_flag("h", false);
        self.cpu.set_flag("c", !c);
        4
    }

    // scf
    pub fn scf(&mut self) -> u8 {
        self.cpu.set_flag("n", false);
        self.cpu.set_flag("h", false);
        self.cpu.set_flag("c", true);
        4
    }

    // nop
    pub fn nop(&mut self) -> u8 {
        4
    }

    // halt
    pub fn halt(&mut self) -> u8 {
        // TODO: halt
        self.memory.dump_vram();
        panic!("halt not implemented");
        4
    }

    // stop
    pub fn stop(&mut self) -> u8 {
        // TODO: stop
        self.memory.dump_vram();
        panic!("stop not implemented");
        4
    }

    // di
    pub fn di(&mut self) -> u8 {
        // TODO: disable interrupts
        //self.memory.dump_vram();
        //panic!("di not implemented");
        4
    }

    // ei
    pub fn ei(&mut self) -> u8 {
        // TODO: enable interrupts
        //self.memory.dump_vram();
        //panic!("ei not implemented");
        4
    }

    // GMB Jumpcommands
    // jp nn
    pub fn jp_nn(&mut self) -> u8 {
        let nn = self.memory.read_word(self.cpu.get_pc());
        self.cpu.set_pc(nn);
        16
    }

    // jp HL
    pub fn jp_hl(&mut self) -> u8 {
        let hl = self.cpu.get_rr("hl");
        self.cpu.set_pc(hl);
        4
    }

    // jp f,nn
    pub fn jp_f_nn(&mut self, f: &str) -> u8 {
        let nn = self.memory.read_word(self.cpu.get_pc());
        if self.cpu.get_flag(f) {
            self.cpu.set_pc(nn);
            16
        } else {
            12
        }
    }

    // jr PC+dd 
    pub fn jr_pc_dd(&mut self) -> u8 {
        let dd = self.memory.read_byte(self.cpu.get_pc());
        let pc = self.cpu.get_pc();
        let result = pc.overflowing_add(dd as u16);
        self.cpu.set_pc(result.0);
        12
    }

    // jr f,PC+dd
    pub fn jr_f_pc_dd(&mut self, f: &str) -> u8 {
        let dd = self.memory.read_byte(self.cpu.get_pc());
        let pc = self.cpu.get_pc();
        let result = pc.overflowing_add(dd as u16);
        if self.cpu.get_flag(f) {
            self.cpu.set_pc(result.0);
            12
        } else {
            8
        }
    }

    // call nn
    pub fn call_nn(&mut self) -> u8 {
        let nn = self.memory.read_word(self.cpu.get_pc());
        let pc = self.cpu.get_pc();
        let sp = self.cpu.get_rr("sp");
        let result = sp.overflowing_sub(2);
        self.memory.write_word(result.0, pc);   
        self.cpu.set_pc(nn);     
        24
    }

    // call f,nn
    pub fn call_f_nn(&mut self, f: &str) -> u8 {
        if self.cpu.get_flag(f) {
            self.call_nn()
        } else {
            12
        }
    }

    // ret
    pub fn ret(&mut self) -> u8 {
        let sp = self.cpu.get_rr("sp");
        self.cpu.set_pc(self.memory.read_word(sp));
        self.cpu.set_sp(sp.wrapping_add(2));
        16
    }

    // ret f
    pub fn ret_f(&mut self, f: &str) -> u8 {
        if self.cpu.get_flag(f) {
            self.ret()
        } else {
            8
        }
    }

    // reti
    pub fn reti(&mut self) -> u8 {
        // TODO: return and enable interrupts
        16
    }

    // rst n
    pub fn rst(&mut self, n: u16) -> u8 {
        let pc = self.cpu.get_pc();
        let sp = self.cpu.get_rr("sp");
        let result = sp.overflowing_sub(2);
        self.memory.write_word(result.0, pc);   
        self.cpu.set_pc(n);     
        16
    }
}