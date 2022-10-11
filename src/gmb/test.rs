use super::GMB;

mod test_8bit_loadcommands;
mod test_16bit_loadcomands;
mod test_16bit_arithmetic_logical_commands;

pub fn get_test_gmb(opcode: u8) -> GMB {
    let mut gmb = GMB::new();
    gmb.reset();
    gmb.memory.write_byte(0x100, opcode);
    gmb
}

pub fn check_flags(flags: [bool; 4], gmb: &GMB) -> bool {
    if gmb.cpu.get_flag("z") != flags[0] { return false; }
    if gmb.cpu.get_flag("n") != flags[1] { return false; }
    if gmb.cpu.get_flag("h") != flags[2] { return false; }
    if gmb.cpu.get_flag("c") != flags[3] { return false; }
    true
}