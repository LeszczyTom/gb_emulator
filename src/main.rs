pub mod rom;
pub mod memory;
pub mod cpu;
pub mod gameboy;

use gameboy::GAMEBOY;

//http://bgb.bircd.org/pandocs.htm#mbc1max2mbyteromandor32kbyteram

fn main() {
    let rom_path = "./resources/rom.gbc";
    let mut gb = GAMEBOY::new();
    gb.load_rom(rom_path);

    /*let op = gb.fetch_opcode();
    println!("{:02x}, cycles: {}", op, gb.execute_opcode(0x18));*/

    loop {
        let op = gb.fetch_opcode();
        println!("{:02x}, cycles: {}", op, gb.execute_opcode(op));
    }
}