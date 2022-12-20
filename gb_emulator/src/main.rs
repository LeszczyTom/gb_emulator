use std::collections::HashMap;

use chrono::Utc;
use eframe::egui;
use gameboy::GameBoy;


#[derive(PartialEq, Hash, Eq)]
enum Widgets {
    MenuBar,
    GameboyScreen,
    Registers
}

const WIDTH: usize = 144;
const HEIGHT: usize = 160;
const MENU_BAR_HEIGHT: f32 = 10.;
const SPACING_SIZE: f32 = 5.;
const BORDER_SIZE: f32 = 0.2;

fn main() {
    let native_options = eframe::NativeOptions {
        centered: true,
        resizable: false,
        decorated: false,
        ..Default::default()
    };

    eframe::run_native(
        "Gameboy emulator", 
        native_options,
        Box::new(|cc| 
            Box::new(GameboyEmulatorGUI::new(cc, 3., 60))
        )
    );
}

struct GameboyEmulatorGUI {
    size: [usize; 2],
    pixels: [u8; WIDTH * HEIGHT * 4],
    gameboy: GameBoy,
    fps: u32,
    scaled_size: [f32; 2],
    visible_widgets: HashMap<Widgets, bool>,
    resize_requested: bool
}

impl GameboyEmulatorGUI {
    fn new(_cc: &eframe::CreationContext<'_>, scale: f32, fps: u32) -> Self {
        let scaled_width = WIDTH as f32 * scale;
        let scaled_height = HEIGHT as f32 * scale;
        let visible_widgets = HashMap::from([
            (Widgets::MenuBar, true),
            (Widgets::GameboyScreen, true),
            (Widgets::Registers, true)
        ]);

        Self {
            size: [HEIGHT, WIDTH],
            pixels: [0; HEIGHT * WIDTH * 4],
            gameboy: GameBoy::new(),
            fps,
            scaled_size: [scaled_height, scaled_width],
            visible_widgets,
            resize_requested: true
        }
    }

    fn resize_window(&self, frame: &mut eframe::Frame, expected_window_size: egui::Vec2) {
        let current_winow_size = frame.info().window_info.size;
        if current_winow_size.ne(&expected_window_size) {
            frame.set_window_size(expected_window_size);
        }
    }

    fn get_corect_window_size(&self) -> egui::Vec2 {
        // let mut width: f32 = 0.;
        // let mut height: f32 = 0.;
        
        // if *self.visible_widgets.get(&Widgets::MenuBar).unwrap() {
        //     if width < 200. { width = 200. };
        //     height += 10.;
        // }
        
        // if *self.visible_widgets.get(&Widgets::GameboyScreen).unwrap() {
        //     if width < self.scaled_size[0] { width = self.scaled_size[0] }
        //     height += self.scaled_size[1];
        // }
        
        // if *self.visible_widgets.get(&Widgets::Registers).unwrap() {
        //     if height < 200. { height = 200. }
        //     width += 200.
        // }

        // if width < 200. { width = 200.}
        // if height < 200. { height = 200. }

        let margin = 20.;
        let width = self.scaled_size[0] * 2. + 9.; // magic
        let height = self.scaled_size[1] + MENU_BAR_HEIGHT + margin + BORDER_SIZE * 2.;

        egui::Vec2::new(width, height)
    }

    fn draw_widgets(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let test_frame = egui::Frame {
            fill: egui::Color32::from_rgb(0x41, 0x41, 0x41),
            ..Default::default()
        };
        egui::CentralPanel::default().frame(test_frame).show(ctx, |_| {});

        if *self.visible_widgets.get(&Widgets::MenuBar).unwrap() {
            self.draw_menu_bar_widget(ctx, frame)
        }

        if *self.visible_widgets.get(&Widgets::GameboyScreen).unwrap() {
            self.draw_gameboy_screen_widget(ctx)
        }

        if *self.visible_widgets.get(&Widgets::Registers).unwrap() {
            self.draw_register_widget(ctx)
        }
    }

    fn draw_register_widget(&self, ctx: &egui::Context) {
        let register_widget_frame = egui::Frame {
            fill: egui::Color32::from_rgb(0x1E, 0x1E, 0x1E),
            outer_margin: egui::style::Margin {left: 0., top: 0., right: BORDER_SIZE, bottom: BORDER_SIZE},
            ..Default::default()
        };

        egui::SidePanel::left("register_panel")
            .exact_width(self.scaled_size[0] - BORDER_SIZE)
            .resizable(false)
            .frame(register_widget_frame)
            .show(ctx, |ui| {
                ui.painter().text(egui::Pos2::default(),
                    egui::Align2::CENTER_CENTER,
                    "Registers", 
                    egui::FontId::default(), 
                    egui::Color32::RED
                );

                ui.add(egui::Separator::default().vertical().spacing(SPACING_SIZE))
        });
    }

    fn draw_menu_bar_widget(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let menu_bar_widget_frame = egui::Frame {
            fill: egui::Color32::from_rgb(0x3C,0x3C,0x3C),
            inner_margin: egui::style::Margin::same(1.),
            outer_margin: egui::style::Margin { left: BORDER_SIZE, right: BORDER_SIZE, top: BORDER_SIZE, bottom: 0.},
            ..Default::default()
        };

        egui::TopBottomPanel::top("menu_bar")
            .frame(menu_bar_widget_frame)
            .exact_height(MENU_BAR_HEIGHT)
            .show(ctx, |ui| {
                let rect_menu_bar = {
                    let mut rect = ui.max_rect();
                    rect.max.y = rect.min.y + MENU_BAR_HEIGHT;
                    rect
                };

                let menu_bar_response = ui.interact(rect_menu_bar, egui::Id::new("menu_bar_dragable"), egui::Sense::click());
                if menu_bar_response.is_pointer_button_down_on() {
                    frame.drag_window();
                }

                egui::menu::bar(ui, |ui| {
                    ui.menu_button("File", |ui| {
                        if ui.button("Load Rom").clicked() {
                            // …
                        }
                    });

                    ui.menu_button("View", |ui| {                       
                        if ui.button("Show Menu bar (Alt)").clicked() {
                            self.visible_widgets.entry(Widgets::MenuBar).and_modify(|value| *value = !*value);
                            // self.resize_requested = true;
                        }
                        
                        if ui.button("Show Gameboy screen").clicked() {
                            self.visible_widgets.entry(Widgets::GameboyScreen).and_modify(|value| *value = !*value);
                            // self.resize_requested = true;
                        }

                        if ui.button("Show Registers").clicked() {
                            self.visible_widgets.entry(Widgets::Registers).and_modify(|value| *value = !*value);
                            // self.resize_requested = true;
                        }
                    });

                    let min_size_close_button = egui::Vec2::new(30., 10.);
                    let available_width = ui.available_width();
                    ui.add_space(available_width - min_size_close_button[0]);

                    let button = egui::Button::new(egui::RichText::new("❌")).min_size(min_size_close_button);
                    if ui.add(button).clicked() {
                        frame.close();
                    };
                });

                
        });
    }

    fn draw_gameboy_screen_widget(&self, ctx: &egui::Context) {
        let image = egui::ColorImage::from_rgba_unmultiplied(
            self.size,
            self.pixels.as_slice(),
        );
        let gameboy_screen_texture = ctx.load_texture("gameboy_screen_texture", image, egui::TextureOptions::NEAREST);

        let gameboy_screen_frame = egui::containers::Frame {
            inner_margin: egui::style::Margin {left: 5., right: 5., top: 4.8, bottom: 4.8},
            outer_margin: egui::style::Margin {left: BORDER_SIZE, right: 0., top: 0., bottom: BORDER_SIZE},
            fill: egui::Color32::from_rgb(0x1E, 0x1E, 0x1E),
            ..Default::default()
        };

        egui::SidePanel::left("gameboy_screen")
            .resizable(false)
            .frame(gameboy_screen_frame)
            .show(ctx, |ui| {
                ui.image(&gameboy_screen_texture, self.scaled_size);
        }); 
    }
}

impl eframe::App for GameboyEmulatorGUI {
    fn clear_color(&self, _visuals: &egui::Visuals) -> egui::Rgba {
        //println!("{:?}", _visuals.window_shadow);
        egui::Rgba::TRANSPARENT
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if self.resize_requested {
            self.resize_window(frame, self.get_corect_window_size());
            self.resize_requested = false;
        }

        if !ctx.input().keys_down.is_empty() {
            // TODO: Process inputs
            println!("Keys down: {:?}", ctx.input().keys_down);
        }

        let time = Utc::now();
        
        self.gameboy.cycle(&mut self.pixels, self.fps);
    
        self.draw_widgets(ctx, frame);

        while Utc::now().timestamp_micros() - time.timestamp_micros() <= (1_000_000 / self.fps) as i64 {}
        
        ctx.request_repaint();
    }
}