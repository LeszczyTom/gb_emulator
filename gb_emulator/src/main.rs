use std::collections::HashMap;

use chrono::Utc;
use eframe::egui;
use gameboy::GameBoy;


#[derive(PartialEq, Hash, Eq)]
enum Widgets {
    MenuBar,
    GameboyScreen,
    Debug
}

const WIDTH: usize = 144;
const HEIGHT: usize = 160;
const MENU_BAR_HEIGHT: f32 = 10.;
const BORDER_SIZE: f32 = 0.5;

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
    resize_requested: bool,
    debug_values: gameboy::io::debug::Debug,
}

impl GameboyEmulatorGUI {
    fn new(_cc: &eframe::CreationContext<'_>, scale: f32, fps: u32) -> Self {
        let scaled_width = WIDTH as f32 * scale;
        let scaled_height = HEIGHT as f32 * scale;
        let visible_widgets = HashMap::from([
            (Widgets::MenuBar, true),
            (Widgets::GameboyScreen, true),
            (Widgets::Debug, true)
        ]);

        Self {
            size: [HEIGHT, WIDTH],
            pixels: [0; HEIGHT * WIDTH * 4],
            gameboy: GameBoy::new(),
            fps,
            scaled_size: [scaled_height, scaled_width],
            visible_widgets,
            resize_requested: true,
            debug_values: Default::default()
        }
    }

    fn resize_window(&self, frame: &mut eframe::Frame, expected_window_size: egui::Vec2) {
        let current_winow_size = frame.info().window_info.size;
        if current_winow_size.ne(&expected_window_size) {
            frame.set_window_size(expected_window_size);
        }
    }

    fn get_corect_window_size(&self) -> egui::Vec2 {
        let mut nb_panels = 0.;
        if *self.visible_widgets.get(&Widgets::GameboyScreen).unwrap() { nb_panels += 1.}
        if *self.visible_widgets.get(&Widgets::Debug).unwrap() { nb_panels += 1.}
        if nb_panels == 0. {nb_panels = 1.}

        let margin = 20.;
        let width = self.scaled_size[0] * nb_panels + 19.; // magic
        let height = self.scaled_size[1] + MENU_BAR_HEIGHT + margin + BORDER_SIZE * 2.;

        egui::Vec2::new(width, height)
    }

    fn draw_widgets(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if *self.visible_widgets.get(&Widgets::MenuBar).unwrap() {
            self.draw_menu_bar_widget(ctx, frame)
        }

        if *self.visible_widgets.get(&Widgets::GameboyScreen).unwrap() {
            self.draw_gameboy_screen_widget(ctx)
        }

        if *self.visible_widgets.get(&Widgets::Debug).unwrap() {
            self.draw_debug_widget(ctx)
        }
    }

    fn draw_debug_widget(&self, ctx: &egui::Context) {
        let register_widget_frame = egui::Frame {
            fill: egui::Color32::from_rgb(0x1E, 0x1E, 0x1E),
            inner_margin: egui::style::Margin::same(5.),
            stroke: egui::Stroke {width: 0.2, color: egui::Color32::from_rgb(0x39, 0x39, 0x39)},
            ..Default::default()
        };
        
        egui::SidePanel::left("debug_panel")
            .exact_width(self.scaled_size[0] - BORDER_SIZE)
            .resizable(false)
            .frame(register_widget_frame)
            .show(ctx, |_| {
                show_register_window(ctx, &self.debug_values);
            });
    }

    fn draw_menu_bar_widget(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let menu_bar_widget_frame = egui::Frame {
            fill: egui::Color32::from_rgb(0x3C,0x3C,0x3C),
            inner_margin: egui::style::Margin::same(1.),
            stroke: egui::Stroke {width: 0.2, color: egui::Color32::from_rgb(0x39, 0x39, 0x39)},
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
                            // TODO
                        }
                    });

                    ui.menu_button("View", |ui| {                       
                        if ui.button("Show Menu bar (Alt)").clicked() {
                            self.visible_widgets.entry(Widgets::MenuBar).and_modify(|value| *value = !*value);
                            self.resize_requested = true;
                        }
                        
                        if ui.button("Show Gameboy screen").clicked() {
                            self.visible_widgets.entry(Widgets::GameboyScreen).and_modify(|value| *value = !*value);
                            self.resize_requested = true;
                        }

                        if ui.button("Show Debugger").clicked() {
                            self.visible_widgets.entry(Widgets::Debug).and_modify(|value| *value = !*value);
                            self.resize_requested = true;
                        }
                    });

                    let text_width = 252.; // magic
                    ui.add_space(frame.info().window_info.size[0] / 2. - text_width / 2.);
                    ui.label(egui::RichText::new("Gameboy emulator").color(egui::Color32::WHITE));
                    
                    let min_size_close_button = egui::Vec2::new(30., 10.);
                    let available_width = ui.available_width();
                    ui.add_space(available_width - min_size_close_button[0]);

                    ui.visuals_mut().widgets = egui::style::Widgets { // Red close button on hovering
                        hovered: egui::style::WidgetVisuals {
                            bg_fill: egui::Color32::from_rgb(0xD7, 0x15, 0x26),
                            bg_stroke: egui::Stroke::default(), 
                            rounding: egui::Rounding::default(), 
                            fg_stroke: egui::Stroke {
                                color: egui::Color32::WHITE,
                                ..Default::default()
                            }, 
                            expansion: 0.
                        },
                        ..Default::default()
                    };

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
}

impl eframe::App for GameboyEmulatorGUI {
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
        
        self.gameboy.cycle(&mut self.pixels, self.fps, &mut self.debug_values);
    
        self.draw_widgets(ctx, frame);

        while Utc::now().timestamp_micros() - time.timestamp_micros() <= (1_000_000 / self.fps) as i64 {}
        
        ctx.request_repaint();
    }
}

fn show_register_window(ctx: &egui::Context, debug_values: &gameboy::io::debug::Debug) {
    egui::Window::new("Registers")
        .title_bar(false)
        .resizable(false)
        .show(ctx, |ui| {
            ui.label(format!("Cycles:\t{:}", debug_values.cycles));
            ui.label(format!("AF:\t0x{:02X}{:02X}", debug_values.a, debug_values.f));
            ui.label(format!("BC:\t0x{:02X}{:02X}", debug_values.b, debug_values.c));
            ui.label(format!("DE:\t0x{:02X}{:02X}", debug_values.d, debug_values.e));
            ui.label(format!("HL:\t0x{:02X}{:02X}", debug_values.h, debug_values.l));
            ui.label(format!("SP:\t0x{:04X}", debug_values.sp));
            ui.label(format!("PC:\t0x{:04X}", debug_values.pc));
    });
}