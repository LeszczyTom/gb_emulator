use std::{path::PathBuf, str::FromStr};

use eframe::egui;

use crate::widgets;

pub struct MenuBar {
    visible: bool,
    frame: egui::Frame,
    height: f32,
}

impl Default for MenuBar {
    fn default() -> Self {
        let menu_bar_widget_frame = egui::Frame {
            fill: egui::Color32::from_rgb(0x3C, 0x3C, 0x3C),
            inner_margin: egui::style::Margin::same(1.),
            stroke: egui::Stroke {
                width: 0.2,
                color: egui::Color32::from_rgb(0x39, 0x39, 0x39),
            },
            ..Default::default()
        };

        Self {
            visible: true,
            frame: menu_bar_widget_frame,
            height: 10.,
        }
    }
}

impl MenuBar {
    pub fn show(
        &mut self,
        ctx: &egui::Context,
        frame: &mut eframe::Frame,
        debug_widget: &mut widgets::debug_widget::DebugWidget,
        gameboy_screen: &mut widgets::gameboy_screen::GameboyScreen,
        resize_requested: &mut bool,
        gameboy: &mut gameboy::GameBoy,
    ) {
        egui::TopBottomPanel::top("menu_bar")
            .frame(self.frame)
            .exact_height(self.height)
            .show(ctx, |ui| {
                let rect_menu_bar = {
                    let mut rect = ui.max_rect();
                    rect.max.y = rect.min.y + self.height;
                    rect
                };

                let menu_bar_response = ui.interact(
                    rect_menu_bar,
                    egui::Id::new("menu_bar_dragable"),
                    egui::Sense::click(),
                );
                if menu_bar_response.is_pointer_button_down_on() {
                    frame.drag_window();
                }

                egui::menu::bar(ui, |ui| {
                    ui.add_visible_ui(self.visible, |ui| {
                        self.show_file_button(ui, gameboy);
                        self.show_view_button(ui, resize_requested, gameboy_screen, debug_widget);
                    });

                    ui.add_space(frame.info().window_info.size[0] / 2. - 252. / 2.); // magic

                    ui.label(egui::RichText::new("Gameboy emulator").color(egui::Color32::WHITE));

                    let min_size_close_button = egui::Vec2::new(30., 10.);
                    let available_width = ui.available_width();
                    ui.add_space(available_width - min_size_close_button[0]);

                    ui.visuals_mut().widgets = egui::style::Widgets {
                        // Red close button on hovering
                        hovered: egui::style::WidgetVisuals {
                            bg_fill: egui::Color32::from_rgb(0xD7, 0x15, 0x26),
                            bg_stroke: egui::Stroke::default(),
                            rounding: egui::Rounding::default(),
                            fg_stroke: egui::Stroke {
                                color: egui::Color32::WHITE,
                                ..Default::default()
                            },
                            expansion: 0.,
                        },
                        ..Default::default()
                    };

                    let button = egui::Button::new(egui::RichText::new("❌"))
                        .min_size(min_size_close_button);
                    if ui.add(button).clicked() {
                        frame.close();
                    };
                });
            });
    }

    fn show_file_button(&self, ui: &mut egui::Ui, gameboy: &mut gameboy::GameBoy) {
        ui.menu_button("File", |ui| {
            if ui.button("Load Rom").clicked() {
                let mut path_to_resources_folder = std::env::current_dir().unwrap();
                path_to_resources_folder.push("resources");

                if !path_to_resources_folder.exists() {
                    path_to_resources_folder = PathBuf::from("~/Desktop");
                }

                let path = native_dialog::FileDialog::new()
                    .set_location(&path_to_resources_folder)
                    .add_filter("ROM files", &["gb"])
                    .show_open_single_file()
                    .unwrap();

                match path {
                    Some(path) => gameboy.load_roam(path),
                    None => println!("Invalid path"),
                };
                ui.close_menu()
            }
        });
    }

    fn show_view_button(
        &mut self,
        ui: &mut egui::Ui,
        resize_requested: &mut bool,
        gameboy_screen: &mut widgets::gameboy_screen::GameboyScreen,
        debug_widget: &mut widgets::debug_widget::DebugWidget,
    ) {
        ui.menu_button("View", |ui| {
            if ui.button("Show Menu bar (Alt)").clicked() {
                self.visible = !self.visible;
                *resize_requested = true;
            }

            if ui.button("Show Gameboy screen").clicked() {
                gameboy_screen.update_visibility();
                *resize_requested = true;
            }

            if ui.button("Show Debugger").clicked() {
                debug_widget.update_visibility();
                *resize_requested = true;
            }

            ui.menu_button("Debug widgets", |ui| {
                if ui.button("Show CPU registers").clicked() {
                    debug_widget.register_window.update_visibility()
                }

                if ui.button("Show Memory dump").clicked() {
                    debug_widget.memory_dump_window.update_visibility()
                }

                if ui.button("Show Controls").clicked() {
                    debug_widget.controls_window.update_visibility()
                }

                if ui.button("Show Timer").clicked() {
                    debug_widget.timer_widget.update_visibility()
                }

                if ui.button("Show Interrupts").clicked() {
                    debug_widget.interrupts_widget.update_visibility()
                }

                if ui.button("Show Background map").clicked() {
                    debug_widget.background_map_widget.update_visibility()
                }

                if ui.button("Show Tile data").clicked() {
                    debug_widget.tile_data_widget.update_visibility()
                }
            });
        });
    }

    pub fn get_size(&self) -> egui::Vec2 {
        egui::Vec2::new(300., self.height)
    }
}
