#![allow(non_snake_case)]
use eframe::egui;
pub mod database;
pub mod navigation;
pub mod widgets;

pub fn test(ui: &mut egui::Ui) {
    ui.label("testou");
}
