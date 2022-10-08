use super::GMB;

mod test_8bit_loadcommands;
mod test_16bit_loadcomands;

pub fn get_test_gmb(opcode: u8) -> GMB {
    let mut gmb = GMB::new();
    gmb.reset();
    gmb.memory.write_byte(0x100, opcode);
    gmb
}

