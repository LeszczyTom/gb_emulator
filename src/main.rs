pub mod rom;
pub mod memory;
pub mod cpu;
pub mod gameboy;

use gameboy::GAMEBOY;

//http://bgb.bircd.org/pandocs.htm#mbc1max2mbyteromandor32kbyteram

fn main() {
    let rom_path = "./resources/tetris.gb";
    let mut gb = GAMEBOY::new();
    gb.load_rom(rom_path);

    /*let op = gb.fetch_opcode();
    println!("{:02x}, cycles: {}", op, gb.execute_opcode(0x18));*/

    let mut max: i32 = 1000;
    loop {
        if max <= 0 {
            break;
        }
        let op = gb.fetch_opcode();
        max -= gb.execute_opcode(op) as i32;
        println!("{:x}", op,);
    }

    println!("38 {:02x}", gb.memory.read_byte(0x38));
}