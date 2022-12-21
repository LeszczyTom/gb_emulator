use eframe::egui;

pub struct MemoryDump {
    visible: bool
}

impl Default for MemoryDump {
    fn default() -> Self {
        Self {
            visible: true
        }
    }
}

impl MemoryDump {
    pub fn show(&self, ctx: &egui::Context, mmu: &gameboy::memory::mmu::Mmu) {
        if !self.visible {
            return
        }

        egui::Window::new("memory_dump")
        .title_bar(false)
        .resizable(false)
        .show(ctx, |ui| {
            ui.label(get_hex_dump_at(&mmu.get_slice_data(0xFFF0)))
        });
    }

    fn _update_visibility(&mut self) {
        self.visible = !self.visible;
    }    
}

fn get_hex_dump_at(mmu: &[u8]) -> String {    

    let mut res: String = "".to_string();
    mmu.chunks(16).for_each(|chunk| {
        res = "".to_string();
        chunk.iter().for_each(|value| {
            res.push_str(&format!("{:02X} ", value))
        })
    });

    res
}