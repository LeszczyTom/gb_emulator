pub mod cpu;
pub mod memory;
pub mod rom;
pub mod gmb;

fn main() {
    let rom_path = "./resources/tetris.gb";
    let mut gmb = gmb::GMB::new();
    let mut rom = rom::ROM::new();
    rom.load_rom(rom_path);
    //rom.dump_cartrige_info();
    gmb.load_rom(rom.data);
    gmb.reset();

    gmb.run();
}
