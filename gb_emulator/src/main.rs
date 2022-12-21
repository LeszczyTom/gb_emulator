mod widgets;

use std::{collections::HashMap, ops::Add};

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
const MARGIN: f32 = 10.;
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
    pixels: [u8; WIDTH * HEIGHT * 4],
    gameboy: GameBoy,
    fps: u32,
    scaled_size: [f32; 2],
    visible_widgets: HashMap<Widgets, bool>,
    resize_requested: bool,
    debug_widget: widgets::debug_widget::DebugWidget,
    menu_bar_widget: widgets::menu_bar::MenuBar,
    gameboy_screen_widget: widgets::gameboy_screen::GameboyScreen,
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
            pixels: [0; HEIGHT * WIDTH * 4],
            gameboy: GameBoy::new(),
            fps,
            scaled_size: [scaled_height, scaled_width],
            visible_widgets,
            resize_requested: true,
            debug_widget: widgets::debug_widget::DebugWidget::default(),
            menu_bar_widget: widgets::menu_bar::MenuBar::default(),
            gameboy_screen_widget: widgets::gameboy_screen::GameboyScreen::new(scale),
        }
    }

    fn resize_window(&self, frame: &mut eframe::Frame, expected_window_size: egui::Vec2) {
        let current_winow_size = frame.info().window_info.size;
        if current_winow_size.ne(&expected_window_size) {
            frame.set_window_size(expected_window_size);
        }
    }

    fn get_corect_window_size(&self) -> egui::Vec2 {
        let mut size = egui::vec2(0., 0.);
        let mut visible_count: f32 = 0.;

        size = size.add(self.gameboy_screen_widget.get_size());

        if self.gameboy_screen_widget.is_visible() {
            visible_count += 1.;
        }
        
        if self.debug_widget.is_visible() {
            visible_count += 1.;

            if self.gameboy_screen_widget.is_visible() { size.x *= 2.; }
            else { size = size.add(self.debug_widget.get_size())}
        }

        let vertical_margin = (MARGIN * visible_count) - 1.;
        let mut horizontal_margin = MARGIN;
        
        if self.gameboy_screen_widget.is_visible() | self.debug_widget.is_visible() { 
            horizontal_margin += MARGIN;
            size.y += self.menu_bar_widget.get_size().y; 
        } else {
            size = size.add(self.menu_bar_widget.get_size());
        };

        size.add(egui::vec2(vertical_margin, horizontal_margin))
    }

    fn draw_widgets(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.menu_bar_widget.show(ctx, frame, &mut self.debug_widget, &mut self.resize_requested);

        self.gameboy_screen_widget.show(ctx, &self.pixels);

        self.debug_widget.show(ctx, self.scaled_size[0], &self.gameboy.mmu, &self.gameboy.cpu);   
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
        
        self.gameboy.cycle(&mut self.pixels, self.fps);
    
        self.draw_widgets(ctx, frame);

        while Utc::now().timestamp_micros() - time.timestamp_micros() <= (1_000_000 / self.fps) as i64 {}
        
        ctx.request_repaint();
    }
}