use eframe::egui;
use crate::widgets::debug;

pub struct DebugWidget {
    visible: bool,
    frame: egui::Frame,
    pub register_window: debug::registers::Register,
    pub memory_dump_window: debug::memory_dump::MemoryDump
}

impl Default for DebugWidget {
    fn default() -> Self {
        let register_widget_frame = egui::Frame {
            fill: egui::Color32::from_rgb(0x1E, 0x1E, 0x1E),
            inner_margin: egui::style::Margin::same(5.),
            stroke: egui::Stroke {width: 0.2, color: egui::Color32::from_rgb(0x39, 0x39, 0x39)},
            ..Default::default()
        };

        Self {
            visible: true,
            frame: register_widget_frame,
            register_window: debug::registers::Register::default(),
            memory_dump_window: debug::memory_dump::MemoryDump::default(),
        }
    }
}

impl DebugWidget {
    pub fn show(&mut self, 
        ctx: &egui::Context, 
        width: f32, 
        mmu: &gameboy::memory::mmu::Mmu, 
        cpu: &gameboy::cpu::cpu::Cpu) { 
            if !self.visible {
                return
            }

            egui::SidePanel::left("debug_panel")
                .exact_width(width)
                .resizable(false)
                .frame(self.frame)
                .show(ctx, |_| {
                    self.register_window.show(ctx, cpu);
                    self.memory_dump_window.show(ctx, mmu);
                });
    }

    pub fn get_size(&self) -> egui::Vec2 {
        if !self.visible {
            return egui::Vec2::new(0., 0.)
        }

        egui::Vec2::new(300., 300.)
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }
    
    pub fn update_visibility(&mut self) {
        self.visible = !self.visible
    }
}