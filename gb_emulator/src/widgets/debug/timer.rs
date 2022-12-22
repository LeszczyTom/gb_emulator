use eframe::egui;

const DIVIDER_REGISTER_ADDRESS: u16 = 0xFF04;
const TIMER_COUNTER_ADDRESS: u16 = 0xFF05;
const TIMER_MODULO_ADDRESS: u16 = 0xFF06;
const TIMER_CONTROL_ADDRESS: u16 = 0xFF07;

pub struct Timer {
    visible: bool,
}

impl Default for Timer {
    fn default() -> Self {
        Self { visible: false }
    }
}

impl Timer {
    pub fn show(&mut self, ctx: &egui::Context, mmu: &gameboy::memory::mmu::Mmu) {
        if !self.visible {
            return;
        }

        egui::Window::new("Timer")
            .open(&mut self.visible)
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                get_label_div(ui, mmu);
                get_label_tima(ui, mmu);
                get_label_tma(ui, mmu);
                get_label_tac(ui, mmu);
            });
    }

    pub fn update_visibility(&mut self) {
        self.visible = !self.visible
    }
}

fn get_label_div(ui: &mut egui::Ui, mmu: &gameboy::memory::mmu::Mmu) {
    let div_value = mmu.read_byte(DIVIDER_REGISTER_ADDRESS);
    ui.label(format!("DIV: {:02X}", div_value));
}

fn get_label_tima(ui: &mut egui::Ui, mmu: &gameboy::memory::mmu::Mmu) {
    let tima_value = mmu.read_byte(TIMER_COUNTER_ADDRESS);
    ui.label(format!("TIMA: {:02X}", tima_value));
}

fn get_label_tma(ui: &mut egui::Ui, mmu: &gameboy::memory::mmu::Mmu) {
    let tma_value = mmu.read_byte(TIMER_MODULO_ADDRESS);
    ui.label(format!("TMA: {:02X}", tma_value));
}

fn get_label_tac(ui: &mut egui::Ui, mmu: &gameboy::memory::mmu::Mmu) {
    let tac_value = mmu.read_byte(TIMER_CONTROL_ADDRESS);
    ui.label(format!("TAC: {:02X}", tac_value));

    let timer_enable = if tac_value >> 2 == 0 { false } else { true };
    ui.label(format!("Timer enabled: {}", timer_enable));

    let input_clock_select = tac_value & 0b11;
    let clock_frequency = match input_clock_select {
        0b00 => 4_096,
        0b01 => 262_144,
        0b10 => 65_536,
        0b11 => 16_384,
        _ => unreachable!("input clock select > 0b11"),
    };

    ui.label(format!("Clock frequency: {}Hz", clock_frequency));
}
