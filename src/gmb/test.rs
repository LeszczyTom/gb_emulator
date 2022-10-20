use super::GMB;

mod test_8bit_loadcommands;
mod test_16bit_loadcomands;
mod test_8bit_arithmetic_logical_commands;
mod test_16bit_arithmetic_logical_commands;
mod test_rotate_und_shift_commands;
mod test_singlebit_operation_commands;
mod test_jumpcommands;

pub fn get_test_gmb(opcode: u8) -> GMB {
    let mut gmb = GMB::new();
    gmb.reset();
    gmb.memory.write_byte(0x100, opcode);
    gmb
}

/// [z,n,h,c] 
pub fn check_flags(flags: [bool; 4], gmb: &GMB) -> bool {
    if gmb.cpu.get_flag("z") != flags[0] { 
        println!("z flag is not correct");
        return false;
    }
    if gmb.cpu.get_flag("n") != flags[1] { 
        println!("n flag is not correct");
        return false; 
    }
    if gmb.cpu.get_flag("h") != flags[2] { 
        println!("h flag is not correct");
        return false; 
    }
    if gmb.cpu.get_flag("c") != flags[3] { 
        println!("c flag is not correct");
        return false; 
    }
    true
}

pub fn set_flags(flags: [bool; 4], gmb: &mut GMB) {
    gmb.cpu.set_flag("z", flags[0]);
    gmb.cpu.set_flag("n", flags[1]);
    gmb.cpu.set_flag("h", flags[2]);
    gmb.cpu.set_flag("c", flags[3]);
}