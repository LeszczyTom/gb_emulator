use eframe::egui;

const INTERRUPT_ENABLE_ADDRESS: u16 = 0xFFFF;
const INTERRUPT_FLAG_ADDRESS: u16 = 0xFF0F;

pub struct Interrupts {
    visible: bool,
}

impl Default for Interrupts {
    fn default() -> Self {
        Self { visible: false }
    }
}

impl Interrupts {
    pub fn show(&mut self, ctx: &egui::Context, mmu: &gameboy::memory::mmu::Mmu) {
        if !self.visible {
            return;
        }

        egui::Window::new("Interrupts")
            .open(&mut self.visible)
            .resizable(false)
            .collapsible(false)
            .show(ctx, |ui| {
                get_ie(ui, mmu);
                get_if(ui, mmu);
            });
    }

    pub fn update_visibility(&mut self) {
        self.visible = !self.visible
    }
}

fn get_ie(ui: &mut egui::Ui, mmu: &gameboy::memory::mmu::Mmu) {
    let interrupt_enable = mmu.read_byte(INTERRUPT_ENABLE_ADDRESS);
    ui.label(format!("Interrupt enable: {:04b}", interrupt_enable));
}

fn get_if(ui: &mut egui::Ui, mmu: &gameboy::memory::mmu::Mmu) {
    let interrupt_flag = mmu.read_byte(INTERRUPT_FLAG_ADDRESS);
    ui.label(format!("Interrupt flag: {:04b}", interrupt_flag));
}
