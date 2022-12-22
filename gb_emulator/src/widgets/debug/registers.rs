use eframe::egui;

pub struct Register {
    visible: bool
}

impl Default for Register {
    fn default() -> Self {
        Self { 
            visible: false, 
        }
    }
}

impl Register {
    pub fn show(&mut self, ctx: &egui::Context, cpu: &gameboy::cpu::cpu::Cpu) {
        if !self.visible {
            return
        }

        egui::Window::new("Registers")
            .resizable(false)
            .collapsible(false)
            .open(&mut self.visible)
            .show(ctx, |ui| {
                egui::Grid::new("registers_table").show(ui, |ui| {
                    ui.label("AF");
                    ui.label(format!("0x{:02X}{:02X}", cpu.a, cpu.f));
                    ui.end_row();

                    ui.label("BC");
                    ui.label(format!("0x{:02X}{:02X}", cpu.b, cpu.c));
                    ui.end_row();

                    ui.label("DE");
                    ui.label(format!("0x{:02X}{:02X}", cpu.d, cpu.e));
                    ui.end_row();

                    ui.label("HL");
                    ui.label(format!("0x{:02X}{:02X}", cpu.h, cpu.l));
                    ui.end_row();

                    ui.label("SP");
                    ui.label(format!("0x{:04X}", cpu.sp));
                    ui.end_row();
                    
                    ui.label("PC");
                    ui.label(format!("0x{:04X}", cpu.pc));
                    ui.end_row();
                });
            });
    }

    pub fn update_visibility(&mut self) {
        self.visible = !self.visible
    }
}