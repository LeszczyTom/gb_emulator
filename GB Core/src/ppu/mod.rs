mod bg_fifo;
mod oam_fifo;
mod object;
pub mod ppu;

#[derive(PartialEq, Debug)]
pub enum FetcherState {
    GetTile,
    GetTileDataLow,
    GetTileDataHigh,
    Sleep,
    Push,
}
