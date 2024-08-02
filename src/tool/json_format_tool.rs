use std::io::{Read, Write};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use base64::Engine;
use base64::engine::general_purpose;
use eframe::emath::Align;
use eframe::epaint::Margin;
use egui::{Button, Context, Frame, Id, Layout, popup_below_widget, PopupCloseBehavior, TextEdit, Ui};
use flate2::{Compression, write::GzEncoder};
use log::{error, info};
use serde_json::Value;
use uuid::Uuid;

#[derive(PartialEq, Clone)]
pub struct JsonFormatTool {
    pub json_str: String,
    pub formated_json_str: String,
    pub error_msg: String,
    pub error_start: u64,
    pub show_count: u64,
}

impl JsonFormatTool {
    pub fn new() -> Self {
        Self {
            json_str: "".to_string(),
            formated_json_str: "".to_string(),
            error_msg: "".to_string(),
            error_start: 1,
            show_count: 0,
        }
    }

    pub fn show(&mut self, ctx: &egui::Context, tool_main_ui: &mut Ui) {
        self.show_count = self.show_count + 1;
        info!("DecompressCompressStrTool show:{}", self.show_count);
        tool_main_ui.horizontal_top(|tool_main_ui| {
            let available_width = tool_main_ui.available_width();
            // 使用相同的尺寸配置两个 TextEdit 控件
            let half_width = (available_width - 40.0) * 0.5;
            let text_edit_size = egui::vec2(half_width, tool_main_ui.available_height());


            egui::ScrollArea::vertical().id_source("JsonFormatTool json_str ScrollArea").max_height(tool_main_ui.available_height()).show(tool_main_ui, |tool_main_ui| {
                tool_main_ui.add_sized(text_edit_size, TextEdit::multiline(&mut self.json_str).hint_text("输入json字符串"));
            });

            tool_main_ui.separator();

            egui::ScrollArea::vertical().id_source("JsonFormatTool formated_json_str ScrollArea").max_height(tool_main_ui.available_height()).show(tool_main_ui, |tool_main_ui| {
                let json_data: Result<serde_json::Value, serde_json::Error> = serde_json::from_str(&self.json_str);
                if json_data.is_ok() {
                    tool_main_ui.vertical(|tool_main_ui| {
                        self.show_json(tool_main_ui, &json_data.unwrap(), "root".to_string());
                    });

                    return;
                }
                let error = json_data.unwrap_err();
                tool_main_ui.add_sized(text_edit_size, TextEdit::multiline(&mut format!("{}", error)).hint_text("输入json字符串"));
            });
        });
    }


    fn show_json(&mut self, ui: &mut egui::Ui, data: &serde_json::Value, parent: String) {
        match data {
            serde_json::Value::Object(map) => {
                for (key, value) in map {
                    let id = format!("{} {}", parent, key);
                    ui.push_id(
                        id.clone(),
                        |ui| {
                            match value {
                                Value::Array(_) => {
                                    if ui.collapsing(key, |ui| {
                                        self.show_json(ui, value, id);
                                    }).header_response.clicked() {
                                        // Handle clicks on the header if needed
                                    }
                                }
                                Value::Object(_) => {
                                    if ui.collapsing(key, |ui| {
                                        self.show_json(ui, value, id);
                                    }).header_response.clicked() {
                                        // Handle clicks on the header if needed
                                    }
                                }
                                _ => {
                                    ui.label(format!("{}:{}",key,value));
                                }
                            }

                        }
                    );
                }
            }
            serde_json::Value::Array(arr) => {
                for (i, value) in arr.iter().enumerate() {
                    let id = format!("{} {}", parent, i);
                    ui.push_id(
                        id.clone(),
                        |ui| {
                            match value {
                                Value::Array(_) => {
                                    if ui.collapsing(i.to_string(), |ui| {
                                        self.show_json(ui, value,  id.clone());
                                    }).header_response.clicked() {
                                        // Handle clicks on the header if needed
                                    }
                                }
                                Value::Object(_) => {
                                    if ui.collapsing(i.to_string(), |ui| {
                                        self.show_json(ui, value,  id.clone());
                                    }).header_response.clicked() {
                                        // Handle clicks on the header if needed
                                    }
                                }
                                _ => {
                                    ui.label(format!("{}",value));
                                }
                            }

                        }
                    );
                }
            }
            _ => {
                // ui.push_id(format!("{} {}", parent, data.to_string()), |ui| { ui.label(data.to_string()); });
            }
        }
    }
}