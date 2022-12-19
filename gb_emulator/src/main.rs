use chrono::Utc;
use eframe::egui;
use gameboy::GameBoy;

const WIDTH: usize = 144;
const HEIGHT: usize = 160;

const GAMEBOY_SCREEN_FRAME: egui::containers::Frame = egui::containers::Frame {
        inner_margin: egui::style::Margin { left: 0., right: 0., top: 0., bottom: 0. },
        outer_margin: egui::style::Margin { left: 0., right: 0., top: 0., bottom: 0. },
        shadow: eframe::epaint::Shadow{ extrusion: 0., color: egui::Color32::TRANSPARENT },
        rounding: egui::Rounding{nw: 0., ne: 0., sw: 0., se: 0.},
        fill: egui::Color32::TRANSPARENT,
        stroke: egui::Stroke{ width: 0., color: egui::Color32::RED }
    };

fn main() {
    let mut native_options = eframe::NativeOptions::default();
    native_options.centered = true;
    native_options.resizable = false;

    eframe::run_native(
        "Gameboy emulator", 
        native_options,
        Box::new(|cc| 
            Box::new(GbEmulatorGUI::new(cc, 2., 60))
        )
    );
}

struct GbEmulatorGUI {
    size: [usize; 2],
    pixels: [u8; WIDTH * HEIGHT * 4],
    gameboy: GameBoy,
    fps: u32,
    scaled_size: [f32; 2]
}

impl GbEmulatorGUI {
    fn new(_cc: &eframe::CreationContext<'_>, scale: f32, fps: u32) -> Self {
        let scaled_width = WIDTH as f32 * scale;
        let scaled_height = HEIGHT as f32 * scale;
        
        Self {
            size: [HEIGHT, WIDTH],
            pixels: [0; HEIGHT * WIDTH * 4],
            gameboy: GameBoy::new(),
            fps,
            scaled_size: [scaled_height, scaled_width]
        }
    }

    fn resize_window(&mut self, frame: &mut eframe::Frame, expected_window_size: egui::Vec2) {
        let current_winow_size = frame.info().window_info.size;
        if current_winow_size.ne(&expected_window_size) {
            frame.set_window_size(expected_window_size);
        }
    }
}

impl eframe::App for GbEmulatorGUI {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.resize_window(frame, egui::Vec2::from(self.scaled_size)); // TODO: call once 

        let time = Utc::now();
        
        self.gameboy.cycle(&mut self.pixels, self.fps);

        let image = egui::ColorImage::from_rgba_unmultiplied(
            self.size,
            self.pixels.as_slice(),
        );
        let gameboy_screen_texture = ctx.load_texture("gameboy_screen", image, egui::TextureOptions::NEAREST);

        egui::CentralPanel::default()
            .frame(GAMEBOY_SCREEN_FRAME)
            .show(ctx, |ui| {
                ui.image(&gameboy_screen_texture, self.scaled_size);
        });   
        
        while Utc::now().timestamp_micros() - time.timestamp_micros() <= (1_000_000 / self.fps) as i64 {}
        
        ctx.request_repaint();
    }
}