use eframe::egui;

const HEIGHT: usize = 192;
const WIDTH: usize = 128;

pub struct TileData {
    visible: bool,
}

impl Default for TileData {
    fn default() -> Self {
        Self { visible: false }
    }
}

impl TileData {
    pub fn show(&mut self, ctx: &egui::Context, mmu: &gameboy::memory::mmu::Mmu) {
        if !self.visible {
            return;
        }

        egui::Window::new("Tile Data")
            .open(&mut self.visible)
            .resizable(false)
            .collapsible(false)
            .show(ctx, |ui| {
                show_tile_data_image(ui, ctx, mmu);
            });
    }

    pub fn update_visibility(&mut self) {
        self.visible = !self.visible
    }
}

fn show_tile_data_image(ui: &mut egui::Ui, ctx: &egui::Context, mmu: &gameboy::memory::mmu::Mmu) {
    let tile_data_rgba_array = get_tile_data_rgba_array(mmu);

    let image = egui::ColorImage::from_rgba_unmultiplied([WIDTH, HEIGHT], &tile_data_rgba_array);

    let gameboy_screen_texture =
        ctx.load_texture("tile_data_texture", image, egui::TextureOptions::NEAREST);

    ui.image(
        &gameboy_screen_texture,
        egui::Vec2::from([WIDTH as f32, HEIGHT as f32]),
    );
}

fn get_tile_data_rgba_array(mmu: &gameboy::memory::mmu::Mmu) -> [u8; WIDTH * HEIGHT * 4] {
    // let mut res = [0; WIDTH * HEIGHT * 4];
    let mut res = [0; WIDTH * HEIGHT * 4];

    mmu.get_data()
        .get(0x8000..0x9800)
        .unwrap()
        .chunks(16)
        .enumerate() // 16 bytes per tile, 16 tiles per row
        .for_each(|(idx, tile)| {
            tile.chunks(2)
                .enumerate()
                .for_each(|(n_row_tile, byte_pair)| {
                    let mut color = Vec::new();
                    for i in 0..8 {
                        let mut data = [0; 2];
                        data[0] = byte_pair[0] >> (7 - i) & 1;
                        data[1] = byte_pair[1] >> (7 - i) & 1;
                        color.push((data[1] << 1) | data[0]);
                    }

                    color.iter().enumerate().for_each(|(n, pixel)| {
                        let x = (idx % 16) * 8 * 4;
                        let y = (idx / 16) * 8 * 4 + (n_row_tile * 4);
                        let i = (n % 8) * 4;
                        let j = (n / 8) * 4;
                        let index = (x + i) + (y + j) * 128;
                        let color_id = match pixel {
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
                            res[index + i] = color[i];
                        }
                    });
                });
        });

    res
}

const COLORS: [[u8; 4]; 4] = [
    [0xFF, 0xFF, 0xFF, 0xFF],
    [0x7E, 0x7E, 0x7E, 0xFF],
    [0x3F, 0x3F, 0x3F, 0xFF],
    [0x00, 0x00, 0x00, 0xFF],
];
