use egui::{RichText, FontId, Color32};

pub struct GMBApp {
    gmb: super::gmb::GMB,
    running: bool,
    initialized: bool,
    last_refresh: std::time::Instant,
    refresh_per_second: u32,
    max: u32,
}

impl Default for GMBApp {
    fn default() -> Self {
        Self {
            gmb: super::gmb::GMB::new(),
            running: false,
            last_refresh: std::time::Instant::now(),
            refresh_per_second: 1,
            max: 100,
            initialized: false,
        }
    }
}

impl GMBApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for GMBApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("cpu_debug").show(ctx, |ui| {
            ui.heading("CPU debug");
            ui.label(
                RichText::new(self.gmb.cpu_debug())
                    .font(FontId::proportional(19.))
                    .color(Color32::WHITE),
            );
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Gameboy Emulator");
            if ui.button(if self.running {"Stop"} else {"Start"}).clicked() {
                self.running = !self.running;
            }
        });

        if !self.initialized {
            self.gmb.init("./resources/tetris.gb");
            self.initialized = true;
        }

        if self.last_refresh.elapsed().as_secs_f32() > 1.0 / self.refresh_per_second as f32 && self.running {
            self.last_refresh = std::time::Instant::now();
            
            if self.max <= 0 {
                self.running = false;
            }
            self.max -= 1;
            self.gmb.cycle();     
        }

        ctx.request_repaint();
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> egui::Rgba {
        egui::Color32::from_rgba_unmultiplied(12, 12, 12, 180).into()
    }

    fn max_size_points(&self) -> egui::Vec2 {
        egui::Vec2::new(400.0, 400.0)
    }

    fn post_rendering(&mut self, _window_size_px: [u32; 2], _frame: &eframe::Frame) {
        
    }
 }