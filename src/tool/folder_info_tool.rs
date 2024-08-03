use std::fs;
use std::fs::DirEntry;
use chrono::{DateTime, Local};
use egui::{Separator, Ui};
use log::error;
use rfd::FileDialog;

use crate::{AppInstance, ToolType};

/// 文件夹处理
#[derive(PartialEq, Clone)]
pub struct FolderInfoTool {
    pub folder_path: String,
    pub files: Vec<FileInfo>,
    pub loading: bool,
}

#[derive(PartialEq, Clone)]
pub struct FileInfo {
    pub(crate) file_name: String,
    pub(crate) file_path: String,
    pub(crate) file_size: u64,
    pub(crate) update_time: String,
    pub(crate) is_file: bool,
}

impl FolderInfoTool {
    pub fn new() -> Self {
        Self {
            folder_path: "".to_string(),
            files: vec![],
            loading: false,
        }
    }

    ///重新选择工具
    pub fn reselect_tool(&mut self, ui: &mut Ui, app_instance: &mut AppInstance) {
        if !ui.button("重新选择工具").clicked() {
            //未点击
            return;
        }
        app_instance.tool_type = ToolType::Unselected;
    }

    pub fn add_choose_folder_button(&mut self, ui: &mut Ui) {
        if !ui.button("选择文件夹").clicked() {
            //未点击
            return;
        }
        let picked_folder = FileDialog::new().pick_folder();
        //未选择
        if picked_folder.is_none() {
            return;
        }
        let path = picked_folder.unwrap();
        self.folder_path = path.display().to_string();
        self.files.clear();

        //读取文件
        let entries = fs::read_dir(&path);
        if entries.is_err() {
            error!("${path:?} read error:{:?}", entries.unwrap_err());
            return;
        }

        //便利文件
        for entry in entries.into_iter().flatten() {
            if entry.is_err() {
                error!("读取文件错误：{:?}",entry.unwrap_err());
                continue;
            }
            let entry = entry.unwrap();
            let metadata = entry.metadata();
            if metadata.is_err() {
                error!("读取文件：{:?}元信息错误:{:?}",entry,metadata.unwrap_err());
                continue;
            }
            let metadata = metadata.unwrap();

            //构建信息实例
            let file_info = FileInfo {
                file_name: entry.file_name().into_string().unwrap(),
                file_path: entry.path().to_str().unwrap().to_string(),
                file_size: get_file_size(&entry),
                update_time: metadata.modified().map_or("--".to_string(), |update_time| {
                    // 将 SystemTime 转换为 DateTime<Local>
                    let datetime: DateTime<Local> = update_time.into();

                    // 格式化输出
                    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
                }),
                is_file: metadata.is_file(),
            };
            self.files.push(file_info);
        }
        self.files.sort_by(|a, b| {
            a.file_size.cmp(&b.file_size).reverse().then_with(|| a.file_name.cmp(&b.file_name))
        });
    }

    pub fn show_select_file_info(&mut self, ui: &mut Ui) {
        if self.folder_path.is_empty() {
            return;
        }
        //存在文件再展示
        ui.label(format!("已选择的文件夹: {}", self.folder_path));
    }

    pub fn show_sub_file_info(&mut self, ui: &mut Ui) {
        if self.folder_path.is_empty() {
            return;
        }
        ui.separator();

        let available_width = ui.available_width();

        ui.horizontal(|ui| {
            let file_name_column_header = egui::Label::new("文件名");
            ui.add_sized(egui::vec2(available_width * 0.2, ui.spacing().interact_size.y), file_name_column_header);
            let separator = Separator::default();
            let separator = separator.spacing(0.0);
            ui.add(separator);
            let file_path_column_header = egui::Label::new("文件路径");
            ui.add_sized(egui::vec2(available_width * 0.6, ui.spacing().interact_size.y), file_path_column_header);
            let separator = Separator::default();
            let separator = separator.spacing(0.0);
            ui.add(separator);
            let file_is_file_column_header = egui::Label::new("文件");
            ui.add_sized(egui::vec2(available_width * 0.1, ui.spacing().interact_size.y), file_is_file_column_header);
            let separator = Separator::default();
            let separator = separator.spacing(0.0);
            ui.add(separator);
            let file_size_column_header = egui::Label::new("大小");
            ui.add_sized(egui::vec2(available_width * 0.05, ui.spacing().interact_size.y), file_size_column_header);
        });
        let separator = Separator::default();
        let separator = separator.spacing(0.0);
        ui.add(separator);

        egui::ScrollArea::vertical().show(ui, |ui| {
            for file in &self.files {
                ui.horizontal(|ui| {
                    let file_name_column = egui::Label::new(&file.file_name).wrap();
                    ui.add_sized(egui::vec2(available_width * 0.2, ui.spacing().interact_size.y), file_name_column);
                    let separator = Separator::default();
                    let separator = separator.spacing(0.0);
                    ui.add(separator);
                    let file_path_column = egui::Label::new(&file.file_path).wrap();
                    ui.add_sized(egui::vec2(available_width * 0.6, ui.spacing().interact_size.y), file_path_column);
                    let separator = Separator::default();
                    let separator = separator.spacing(0.0);
                    ui.add(separator);
                    let file_is_file_column;
                    if file.is_file {
                        file_is_file_column = egui::Label::new("是");
                    } else {
                        file_is_file_column = egui::Label::new("否");
                    }
                    ui.add_sized(egui::vec2(available_width * 0.1, ui.spacing().interact_size.y), file_is_file_column);
                    let separator = Separator::default();
                    let separator = separator.spacing(0.0);
                    ui.add(separator);
                    let file_size_column = egui::Label::new(file.file_size.to_string());
                    ui.add_sized(egui::vec2(available_width * 0.05, ui.spacing().interact_size.y), file_size_column);
                });
                let separator = Separator::default();
                let separator = separator.spacing(0.0);
                ui.add(separator);
            }
        });
    }
}

/// 获取路径的大小，如果是目录，会统计子、孙文件大小，  如果是文件，字节返回文件大小
fn get_file_size(path: &DirEntry) -> u64 {
    if path.file_type().unwrap().is_file() {
        return path.metadata().unwrap().len();
    }
    let paths = fs::read_dir(path.path());
    if paths.is_err() {
        println!("读取文件失败 = {:?}", paths);
        return 0;
    }
    let paths = paths.unwrap();
    let mut size = 0;
    for path in paths {
        let path = path.unwrap();
        if path.file_type().unwrap().is_file() {
            size = size + path.metadata().unwrap().len();
        } else {
            size = size + get_file_size(&path);
        }
    }
    size
}