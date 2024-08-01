#![windows_subsystem = "windows"]
use std::fs;

use eframe::egui;
use egui::Ui;
use log::{info, warn};
use rfd::FileDialog;



fn main() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    info!("log4rs init finish");

    let mut options = eframe::NativeOptions::default();
    options.centered = true;
    eframe::run_native("File Explorer", options, Box::new(|_cc| Ok(Box::new(folder_tree::AppInstance::new(_cc)))));
}
