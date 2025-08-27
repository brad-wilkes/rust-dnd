mod character_sheet;
mod dnd_gui;
mod db;
mod models;
mod repositories;
mod dice;

use eframe::NativeOptions;
use dnd_gui::DndGui;

fn main() {
    let options = NativeOptions::default();
    eframe::run_native(
        Box::new(DndGui::default()),
        options,
    );
}