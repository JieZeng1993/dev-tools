#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;

use log::{error, info};

fn main() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    info!("log4rs 初始化完成");
    //加载配置
    let options = load_native_options();
    info!("加载原生配置 完成");
    //运行
    eframe::run_native("开发工具箱", options, Box::new(|_cc| Ok(Box::new(dev_tool::AppInstance::new(_cc))))).expect("运行异常");
}

///加载配置
fn load_native_options() -> eframe::NativeOptions {
    //图标路径
    let ico_path = "static/app.ico1";
    //加载图标数据
    let icon_image = load_image_from_path(ico_path).map_or(
        {
            error!("加载图标失败:{ico_path}");
            None
        }, |icon_data| { Some(Arc::new(icon_data)) });
    let viewport = egui::ViewportBuilder {
        icon: icon_image,
        ..Default::default()
    };
    let options = eframe::NativeOptions {
        viewport,
        centered: true,
        ..Default::default()
    };
    options
}

fn load_image_from_path(path: &str) -> Result<egui::IconData, image::ImageError> {
    let image = image::ImageReader::open(path)?.decode()?;
    let image_buffer = image.to_rgba8();
    let mut icon_data = egui::IconData::default();
    icon_data.height = image.height();
    icon_data.width = image.width();
    icon_data.rgba = Vec::from(image_buffer.iter().as_slice());
    return Ok(icon_data);
}