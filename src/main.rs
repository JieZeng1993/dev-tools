// #![windows_subsystem = "windows"]

use log::info;

fn main() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    info!("log4rs init finish");

    let mut options = eframe::NativeOptions::default();
    options.centered = true;
    eframe::run_native("开发工具箱", options, Box::new(|_cc| Ok(Box::new(dev_tool::AppInstance::new(_cc))))).expect("运行异常");
}
