#![windows_subsystem = "windows"]

use log::info;

fn main() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    info!("log4rs init finish");

    let mut options = eframe::NativeOptions::default();
    options.centered = true;
    eframe::run_native("File Explorer", options, Box::new(|_cc| Ok(Box::new(folder_tree::AppInstance::new(_cc))))).expect("运行异常");
}
