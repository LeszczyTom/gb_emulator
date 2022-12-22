use eframe::egui;

pub struct Controls {
    visible: bool,
}

impl Default for Controls {
    fn default() -> Self {
        Self {
            visible: true,
        }
    }
}

impl Controls {
    pub fn show(&mut self, ctx: &egui::Context, gmb: &mut gameboy::GameBoy) {
        if !self.visible {
            return
        }

        egui::Window::new("Controls")
            .collapsible(false)
            .resizable(false)
            .open(&mut self.visible)
            .show(ctx, |ui| {
                let play_label = if gmb.debug_paused { "Paused" } else { "Playing" };
                let play_button_label = if gmb.debug_paused { "Resume" } else { "Pause" };
                
                // play/pause
                ui.horizontal(|ui| {
                    if ui.button(format!("{}", play_button_label)).clicked() {
                        gmb.debug_paused = !gmb.debug_paused;
                    }
                    ui.label(format!("{}", play_label));
                });

                // Speed control
                ui.horizontal(|ui| {
                    ui.add(egui::Slider::new(&mut gmb.speed, 0..=10).prefix("x"));
                    ui.label(format!("Speed: {}%", gmb.speed * 100));
                });

                // Reset gameboy
                if ui.button("Reset").clicked() {
                    gmb.reset()
                }
        });
    }

    pub fn update_visibility(&mut self) {
        self.visible = !self.visible
    }
}