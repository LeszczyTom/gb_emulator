use std::ops::Range;

use eframe::egui;

const WIDTH: usize = 256;
const HEIGHT: usize = 256;

#[derive(PartialEq)]
enum TileMapArea {
    First,
    Second,
}

pub struct BackgroundMap {
    visible: bool,
    tile_map_area: TileMapArea,
    viewport_visible: bool,
}

impl Default for BackgroundMap {
    fn default() -> Self {
        Self {
            visible: false,
            tile_map_area: TileMapArea::First,
            viewport_visible: true,
        }
    }
}
// button window
impl BackgroundMap {
    pub fn show(&mut self, ctx: &egui::Context, mmu: &gameboy::memory::mmu::Mmu) {
        if !self.visible {
            return;
        }

        egui::Window::new("Background Map")
            .open(&mut self.visible)
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Tile map area:");
                    if ui
                        .selectable_label(self.tile_map_area == TileMapArea::First, "9800-9BFF")
                        .clicked()
                    {
                        self.tile_map_area = TileMapArea::First;
                    }

                    if ui
                        .selectable_label(self.tile_map_area == TileMapArea::Second, "9C00-9FFF")
                        .clicked()
                    {
                        self.tile_map_area = TileMapArea::Second;
                    }
                });
                ui.checkbox(&mut self.viewport_visible, "Show Viewport");
                show_tile_map_image(&self.tile_map_area, self.viewport_visible, ui, ctx, mmu);
            });
    }

    pub fn update_visibility(&mut self) {
        self.visible = !self.visible
    }
}

fn show_tile_map_image(
    tile_map_area: &TileMapArea,
    viewport_visible: bool,
    ui: &mut egui::Ui,
    ctx: &egui::Context,
    mmu: &gameboy::memory::mmu::Mmu,
) {
    let mut tile_map_rgba_array = get_tile_map_rgba_array(tile_map_area, mmu);
    if viewport_visible {
        add_viewport_border_to_array(&mut tile_map_rgba_array, mmu);
    }

    let image = egui::ColorImage::from_rgba_unmultiplied([HEIGHT, WIDTH], &tile_map_rgba_array);

    let gameboy_screen_texture =
        ctx.load_texture("tile_map_texture", image, egui::TextureOptions::NEAREST);

    ui.image(
        &gameboy_screen_texture,
        egui::Vec2::from([HEIGHT as f32, WIDTH as f32]),
    );
}

fn get_tile_map_rgba_array(
    tile_map_area: &TileMapArea,
    mmu: &gameboy::memory::mmu::Mmu,
) -> [u8; WIDTH * HEIGHT * 4] {
    let mut res = [0; WIDTH * HEIGHT * 4];
    let tile_map_area: Range<usize> = match tile_map_area {
        TileMapArea::First => 0x9800..0x9C00,
        TileMapArea::Second => 0x9C00..0xA000,
    };

    mmu.get_data()
        .get(tile_map_area)
        .unwrap()
        .chunks(32)
        .enumerate()
        .for_each(|(x, row)| {
            row.iter().enumerate().for_each(|(y, tile_id)| {
                add_tile_to_array(
                    x,
                    y,
                    mmu.get_data()
                        .get(0x8000..0x9800)
                        .unwrap()
                        .chunks(16)
                        .nth(*tile_id as usize)
                        .unwrap(), // Get tile data for tile_id in 8000-97FF
                    &mut res,
                );
            });
        });

    res
}

fn add_viewport_border_to_array(
    array: &mut [u8; WIDTH * HEIGHT * 4],
    mmu: &gameboy::memory::mmu::Mmu,
) {
    let scy = mmu.read_byte(0xFF42) as usize;
    let scx = mmu.read_byte(0xFF43) as usize;

    for x in 0..145_usize {
        for y in 0..161_usize {
            if x == 0 || y == 0 || x == 144 || y == 160 {
                let x = x.wrapping_add(scy);
                let y = y.wrapping_add(scx);

                let index = y * 4 + (x * 4) * 256;

                // Paint border Red
                array[index] = 255;
                array[index + 1] = 0;
                array[index + 2] = 0;
            }
        }
    }
}

fn add_tile_to_array(x: usize, y: usize, tile_data: &[u8], array: &mut [u8; WIDTH * HEIGHT * 4]) {
    let x = x * 8 * 4;
    let y = y * 8 * 4;

    tile_data
        .chunks(2) // 2 bytes to build a row
        .map(|byte_pair| {
            // Build each row from the 2 bytes chunks, return a Vec<u8> which represent a row
            let mut row = Vec::<u8>::new();

            for i in 0..8 {
                let mut data = [0; 2];
                data[0] = byte_pair[0] >> (7 - i) & 1;
                data[1] = byte_pair[1] >> (7 - i) & 1;
                row.push((data[1] << 1) | data[0]);
            }

            row
        })
        .collect::<Vec<Vec<u8>>>() // Collect each row in a Vec. Build a matrix of color_id.
        .iter()
        .enumerate()
        .for_each(|(i, row)| {
            // Read the matrix to add the correct color in the correct index of the array given in parameter.
            row.iter().enumerate().for_each(|(j, value)| {
                let index = y + (j * 4) + (x + (i * 4)) * 256;
                let color_id = match value {
                    // TODO: is there a better way ?
                    0 => 0,
                    1 => 1,
                    2 => 2,
                    3 => 3,
                    _ => unreachable!(),
                };
                let color = COLORS[color_id];
                for i in 0..4 {
                    // RGBA
                    array[index + i] = color[i];
                }
            })
        });
}

const COLORS: [[u8; 4]; 4] = [
    [0xFF, 0xFF, 0xFF, 0xFF],
    [0x7E, 0x7E, 0x7E, 0xFF],
    [0x3F, 0x3F, 0x3F, 0xFF],
    [0x00, 0x00, 0x00, 0xFF],
];
