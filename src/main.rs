#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

pub mod gmb;
pub mod app;

use app::GMBApp;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Gameboy Emulator",
        native_options,
        Box::new(|cc| Box::new(GMBApp::new(cc))),
    );
}