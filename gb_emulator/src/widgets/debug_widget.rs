use crate::widgets::debug;
use eframe::egui;

pub struct DebugWidget {
    visible: bool,
    frame: egui::Frame,
    pub register_window: debug::registers::Register,
    pub memory_dump_window: debug::memory_dump::MemoryDump,
    pub controls_window: debug::controls::Controls,
    pub timer_widget: debug::timer::Timer,
    pub interrupts_widget: debug::interrupts::Interrupts,
    pub background_map_widget: debug::background_map::BackgroundMap,
}

impl Default for DebugWidget {
    fn default() -> Self {
        let register_widget_frame = egui::Frame {
            fill: egui::Color32::from_rgb(0x1E, 0x1E, 0x1E),
            inner_margin: egui::style::Margin::same(5.),
            stroke: egui::Stroke {
                width: 0.2,
                color: egui::Color32::from_rgb(0x39, 0x39, 0x39),
            },
            ..Default::default()
        };

        Self {
            visible: true,
            frame: register_widget_frame,
            register_window: debug::registers::Register::default(),
            memory_dump_window: debug::memory_dump::MemoryDump::default(),
            controls_window: debug::controls::Controls::default(),
            timer_widget: debug::timer::Timer::default(),
            interrupts_widget: debug::interrupts::Interrupts::default(),
            background_map_widget: debug::background_map::BackgroundMap::default(),
        }
    }
}

impl DebugWidget {
    pub fn show(&mut self, ctx: &egui::Context, width: f32, gameboy: &mut gameboy::GameBoy) {
        if !self.visible {
            return;
        }

        egui::SidePanel::left("debug_panel")
            .exact_width(width)
            .resizable(false)
            .frame(self.frame)
            .show(ctx, |_| {
                self.register_window.show(ctx, &gameboy.cpu);
                self.controls_window.show(ctx, gameboy);
                self.memory_dump_window.show(ctx, &gameboy.mmu);
                self.timer_widget.show(ctx, &gameboy.mmu);
                self.interrupts_widget.show(ctx, &gameboy.mmu);
                self.background_map_widget.show(ctx, &gameboy.mmu);
            });
    }

    pub fn get_size(&self) -> egui::Vec2 {
        if !self.visible {
            return egui::Vec2::new(0., 0.);
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
