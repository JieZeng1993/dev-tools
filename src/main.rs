#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{sync::Arc, thread::current};

use log::{error, info};
const LOG4RS_CONFIG_PATH: &str = "config/log4rs.yaml";
fn main() {
    swtich_current_dir();
    let log4rs_init = log4rs::init_file(LOG4RS_CONFIG_PATH, Default::default());
    if log4rs_init.is_err() {
        let log4rs_init = log4rs_init.unwrap_err();
        panic!(
            "log4rs 初始化失败: {},文件路径：{}",
            log4rs_init, LOG4RS_CONFIG_PATH
        );
    }
    info!("log4rs 初始化完成");
    //加载配置
    let options = load_native_options();
    info!("加载原生配置 完成");
    //运行
    eframe::run_native(
        "开发工具箱",
        options,
        Box::new(|_cc| Ok(Box::new(dev_tools::AppInstance::new(_cc)))),
    )
    .expect("运行异常");
}

use std::env;

///切换当前目录
fn swtich_current_dir() {
    let current_dir = env::current_dir();
    if current_dir.is_err() {
        panic!("无法获取当前工作目录: {}", current_dir.unwrap_err());
    }
    let current_dir = current_dir.unwrap();

    println!("当前工作目录: {}", current_dir.display());
    //适配zed ide启动
    if current_dir.file_name().unwrap() != "src" {
        return;
    }
    println!("当前工作目录是源代码目录");
    let set_current_dir_result = std::env::set_current_dir(current_dir.parent().unwrap());
    if set_current_dir_result.is_err() {
        panic!("无法切换到父目录: {}", set_current_dir_result.unwrap_err());
    }
}

///加载配置
fn load_native_options() -> eframe::NativeOptions {
    //图标路径
    let ico_path = "static/app.ico";
    //加载图标数据
    let icon_image = load_image_from_path(ico_path).map_or(
        {
            error!("加载图标失败:{ico_path}");
            None
        },
        |icon_data| Some(Arc::new(icon_data)),
    );
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
