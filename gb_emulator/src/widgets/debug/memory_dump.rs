use eframe::egui;
pub struct MemoryDump {
    visible: bool,
    starting_address: u16,
    max_addresses: u8,
}

impl Default for MemoryDump {
    fn default() -> Self {
        Self {
            visible: true,
            starting_address: 0x00,
            max_addresses: 5,
        }
    }
}

impl MemoryDump {
    pub fn show(&mut self, ctx: &egui::Context, mmu: &gameboy::memory::mmu::Mmu) {
        if !self.visible {
            return
        }

        egui::Window::new("Memory dump")
            .resizable(false)
            .collapsible(false)
            .open(&mut self.visible)
            .show(ctx, |ui| {                
                ui.horizontal(|ui| {
                    ui.add(egui::Slider::new(&mut self.starting_address, 0..=(0xFFFF - 16 * self.max_addresses as u16))
                    .step_by(16.)
                    .prefix("0x")
                    .hexadecimal(4, false, true));
                
                    ui.add(egui::Slider::new(&mut self.max_addresses, 0..=16).step_by(1.).suffix(" rows"));
                });
                
                egui::Grid::new("grid_memory_dump")
                    .max_col_width(40.)
                    .min_col_width(0.)
                    .striped(true)
                    .show(ui, |ui| {
                        for i in 0..self.max_addresses {
                            let addr = self.starting_address + i as u16 * 16;
                            get_row(ui, &mmu.get_slice_data(addr as usize),  addr);
                        }
                    });
        });
    }

    pub fn update_visibility(&mut self) {
        self.visible = !self.visible;
        self.max_addresses = 5;
    }    
}

fn get_row(ui: &mut egui::Ui, slice: &[u8], addr: u16) {
    ui.label(format!("{:04X} ", addr));
    slice.iter().for_each(|value| {
        ui.label(format!("{:02X}", value));
    });
    ui.end_row();
}