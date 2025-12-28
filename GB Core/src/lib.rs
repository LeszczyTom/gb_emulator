pub mod core;
mod cpu;
mod instructions;
mod mmu;
mod ppu;
mod tests;
mod timer;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}
pub(crate) use log;
