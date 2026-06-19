// #![allow(non_snake_case)]
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// mod app;
// mod core;
// mod home;
// mod todo;
// mod watchList;

// /// Point d'entrée partagé (desktop + réutilisable)
// pub fn run() -> eframe::Result<()> {
//     eframe::run_native(
//         env!("CARGO_PKG_NAME"),
//         eframe::NativeOptions::default(),
//         Box::new(|cc| Ok(Box::new(app::App::new(cc)))),
//     )

//     // let options = eframe::NativeOptions {
//     //     viewport: egui::ViewportBuilder::default().with_maximized(true), // ← remplit la work area automatiquement
//     //     ..Default::default()
//     // };

//     // eframe::run_native(
//     //     "MyRustLife",
//     //     options,
//     //     Box::new(|cc| Ok(Box::new(app::App::new(cc)))),
//     // )
// }

// // Android entry point
// #[cfg(target_os = "android")]
// use android_activity::AndroidApp;
// #[allow(unused_imports)]
// use eframe::egui;

// #[cfg(target_os = "android")]
// #[unsafe(no_mangle)]
// fn android_main(app: AndroidApp) {
//     let options = eframe::NativeOptions {
//         android_app: Some(app),
//         viewport: egui::ViewportBuilder::default().with_fullscreen(true), // ← remplit la work area automatiquement
//         ..Default::default()
//     };

//     let _ = eframe::run_native(
//         env!("CARGO_PKG_NAME"),
//         options,
//         Box::new(|cc| Ok(Box::new(app::App::new(cc)))),
//     );
// }

#![allow(non_snake_case)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod core;
mod database;
mod home;
mod todo;
mod watchList;

// #[allow(unused_imports)]
use core::consts::*;
use eframe::egui;
use std::sync::Arc;

/// Shared entry point (desktop + reusable)
pub fn run() -> eframe::Result<()> {
    // Load and decode the PNG icon embedded at compile time
    let icon: egui::IconData =
        eframe::icon_data::from_png_bytes(include_bytes!("../assets/icon/icon.png"))
            .expect("Failed to decode icon.png — make sure it is a valid RGBA PNG");

    let options: eframe::NativeOptions = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_icon(Arc::new(icon)),
        ..Default::default()
    };

    eframe::run_native(
        APP_NAME,
        options,
        Box::new(|cc: &eframe::CreationContext<'_>| Ok(Box::new(app::App::new(cc)))),
    )
}

// Android entry point
#[cfg(target_os = "android")]
use android_activity::AndroidApp;

#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
fn android_main(app: AndroidApp) {
    // Android uses fullscreen — no icon needed at the window level
    let options = eframe::NativeOptions {
        android_app: Some(app),
        viewport: egui::ViewportBuilder::default().with_fullscreen(true),
        ..Default::default()
    };

    let _ = eframe::run_native(
        APP_NAME,
        options,
        Box::new(|cc: &eframe::CreationContext<'_>| Ok(Box::new(app::App::new(cc)))),
    );
}
