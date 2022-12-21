use eframe::egui;

const WIDTH: usize = 160;
const HEIGHT: usize = 144;

pub struct GameboyScreen {
    visible: bool,
    size: [usize; 2],
    scaled_size: [f32; 2],
}

impl Default for GameboyScreen {
    fn default() -> Self {
        Self {
            visible: true,
            size: [WIDTH, HEIGHT],
            scaled_size: [WIDTH as f32, HEIGHT as f32],
        }
    }
}

impl GameboyScreen {
    pub fn new(scale: f32) -> Self {
        Self {
            scaled_size: [WIDTH as f32 * scale, HEIGHT as f32 * scale],
            ..Default::default()
        }
    }

    pub fn show(&self, ctx: &egui::Context, pixels: &[u8]) {
        if !self.visible {
            return
        }

        let image = egui::ColorImage::from_rgba_unmultiplied(
            self.size,
            pixels,
        );
        let gameboy_screen_texture = ctx.load_texture("gameboy_screen_texture", image, egui::TextureOptions::NEAREST);
    
        let gameboy_screen_frame = egui::containers::Frame {
            inner_margin: egui::style::Margin::same(5.),
            fill: egui::Color32::from_rgb(0x1E, 0x1E, 0x1E),
            stroke: egui::Stroke {width: 0.4, color: egui::Color32::from_rgb(0x39, 0x39, 0x39)},
            ..Default::default()
        };
    
        egui::SidePanel::left("gameboy_screen")
            .resizable(false)
            .frame(gameboy_screen_frame)
            .show(ctx, |ui| {
                ui.image(&gameboy_screen_texture, self.scaled_size);
        }); 
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    pub fn update_visibility(&mut self) {
        self.visible = !self.visible
    }

    pub fn get_size(&self) -> egui::Vec2 {
        if !self.visible {
            return egui::Vec2::new(0., 0.)
        }

        egui::Vec2::from(self.scaled_size)
    }
}