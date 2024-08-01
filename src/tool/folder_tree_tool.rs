use std::fs;

use egui::Ui;
use log::error;
use rfd::FileDialog;

use crate::AppInstance;

/// 文件夹处理
#[derive(PartialEq, Clone)]
pub struct FolderTreeTool {
    pub folder_path: String,
    pub files: Vec<String>,
}

impl FolderTreeTool {
    pub fn new() -> Self {
        Self {
            folder_path: "".to_string(),
            files: vec![],
        }
    }


    pub fn add_choose_folder_button(&mut self, ui: &mut Ui) {
        if !ui.button("Choose Folder").clicked() {
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
                continue;
            }
            let entry = entry.unwrap();
            let metadata = entry.metadata();
            if !metadata.unwrap().is_file() {
                return;
            }
            if let Some(name) = entry.file_name().to_str() {
                self.files.push(name.to_owned());
            }
        }
    }

    pub fn show_sub_file_info(&mut self, ui: &mut Ui) {
        if self.folder_path.is_empty() {
            return;
        }
        //存在文件再展示
        ui.label(format!("Selected folder: {}", self.folder_path));
        ui.separator();
        egui::ScrollArea::vertical().show(ui, |ui| {
            for file in &self.files {
                ui.label(file);
            }
        });
    }
}