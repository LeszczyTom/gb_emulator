mod bg_fifo;
mod oam_fifo;
mod object;
mod pixel;
pub mod ppu;

pub enum Palette {
    OBP0,
    OBP1,
}

#[derive(PartialEq, Debug)]
pub enum FetcherState {
    GetTile,
    GetTileDataLow,
    GetTileDataHigh,
    Sleep,
    Push,
}
