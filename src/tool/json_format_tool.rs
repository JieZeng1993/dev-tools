use std::time::Duration;

use eframe::emath::Align;
use egui::{Context, Layout, TextEdit, Ui};
use log::{debug, info};
use serde_json::Value;

use crate::tool::get_seconds_since_epoch;

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

    pub fn show(&mut self, ctx: &egui::Context, tool_main_ui: &mut Ui, _frame: &mut eframe::Frame) {
        self.show_count = self.show_count + 1;
        debug!("DecompressCompressStrTool show:{}", self.show_count);
        tool_main_ui.horizontal_top(|tool_main_ui| {
            let available_width = tool_main_ui.available_width();
            // 使用相同的尺寸配置两个 TextEdit 控件
            let half_width = (available_width - 40.0) * 0.5;
            let text_edit_size = egui::vec2(half_width, tool_main_ui.available_height());
            //展示错误消息
            self.show_error_msg(ctx);

            egui::ScrollArea::vertical().id_source("JsonFormatTool json_str ScrollArea").max_height(tool_main_ui.available_height()).show(tool_main_ui, |tool_main_ui| {
                tool_main_ui.add_sized(text_edit_size, TextEdit::multiline(&mut self.json_str).hint_text("输入json字符串"));
            });

            tool_main_ui.separator();

            egui::ScrollArea::vertical().id_source("JsonFormatTool formated_json_str ScrollArea").max_height(tool_main_ui.available_height()).show(tool_main_ui, |tool_main_ui| {
                let json_data: Result<serde_json::Value, serde_json::Error> = serde_json::from_str(&self.json_str);
                if json_data.is_err() {
                    //有错误时
                    let error = json_data.unwrap_err();
                    tool_main_ui.add_sized(text_edit_size, TextEdit::multiline(&mut format!("{}", error)).hint_text("输入json字符串"));
                    return;
                }

                let json_data = json_data.unwrap();
                tool_main_ui.vertical(|tool_main_ui| {
                    tool_main_ui.horizontal_top(|tool_main_ui| {
                        self.formated_copy(ctx, &json_data, tool_main_ui);
                        self.compress_copy(ctx, &json_data, tool_main_ui);
                    });
                    self.show_json(tool_main_ui, &json_data, "root".to_string());
                });
            });
        });
    }

    fn compress_copy(&mut self, ctx: &Context, json_data: &Value, tool_main_ui: &mut Ui) {
        if !tool_main_ui.button("压缩复制").clicked() {
            return;
        }
        let json_data = serde_json::to_string(&json_data);
        if json_data.is_err() {
            self.error_msg = format!("复制字符串失败:{}", json_data.unwrap_err());
            self.error_start = get_seconds_since_epoch();
            return;
        }
        ctx.copy_text(json_data.unwrap());
    }

    fn formated_copy(&mut self, ctx: &Context, json_data: &Value, tool_main_ui: &mut Ui) {
        if !tool_main_ui.button("格式化复制").clicked() {
            return;
        }
        let json_data = serde_json::to_string_pretty(&json_data);
        if json_data.is_err() {
            self.error_msg = format!("复制字符串失败:{}", json_data.unwrap_err());
            self.error_start = get_seconds_since_epoch();
            return;
        }
        ctx.copy_text(json_data.unwrap());
    }

    fn show_json(&mut self, ui: &mut egui::Ui, data: &serde_json::Value, parent: String) {
        match data {
            serde_json::Value::Object(map) => {
                for (key, value) in map {
                    let id = format!("{} {}", parent, key);
                    ui.push_id(id.clone(), |ui| {
                        match value {
                            serde_json::Value::Array(_) => {
                                if ui.collapsing(key, |ui| {
                                    self.show_json(ui, value, id);
                                }).header_response.clicked() {
                                    // Handle clicks on the header if needed
                                }
                            }
                            serde_json::Value::Object(_) => {
                                if ui.collapsing(key, |ui| {
                                    self.show_json(ui, value, id);
                                }).header_response.clicked() {
                                    // Handle clicks on the header if needed
                                }
                            }
                            _ => {
                                ui.label(format!("{}:{}", key, value));
                            }
                        }
                    },
                    );
                }
            }
            serde_json::Value::Array(arr) => {
                for (i, value) in arr.iter().enumerate() {
                    let id = format!("{} {}", parent, i);
                    ui.push_id(id.clone(), |ui| {
                        match value {
                            serde_json::Value::Array(_) => {
                                if ui.collapsing(i.to_string(), |ui| {
                                    self.show_json(ui, value, id.clone());
                                }).header_response.clicked() {
                                    // Handle clicks on the header if needed
                                }
                            }
                            serde_json::Value::Object(_) => {
                                if ui.collapsing(i.to_string(), |ui| {
                                    self.show_json(ui, value, id.clone());
                                }).header_response.clicked() {
                                    // Handle clicks on the header if needed
                                }
                            }
                            _ => {
                                ui.label(format!("{}", value));
                            }
                        }
                    },
                    );
                }
            }
            _ => {
                //文本已经在在前面处理了
            }
        }
    }

    /// 暂时错误信息
    fn show_error_msg(&mut self, ctx: &Context) {
        if get_seconds_since_epoch() - self.error_start > 5 && !self.error_msg.is_empty() {
            info!("关闭弹窗消息");
            self.error_msg = "".to_string();  // 设置关闭行为
        }

        if self.error_msg.is_empty() {
            return;
        }

        let screen_rect = ctx.available_rect(); // 获取可用屏幕区域
        let window_size = egui::vec2(300.0, 200.0); // 指定窗口大小

        // 计算居中位置
        let position = egui::pos2(
            screen_rect.center().x - window_size.x / 2.0,
            screen_rect.center().y - window_size.y / 2.0,
        );

        //等一秒再刷新一次
        ctx.request_repaint_after(Duration::from_secs(1));
        egui::Window::new("")
            .collapsible(false)
            .fixed_pos(position) // 设置窗口位置
            .fixed_size(window_size) // 设置窗口大小
            .title_bar(false)  // 关闭默认标题栏，以便自定义
            .show(ctx, |ui| {
                // 自定义标题栏，包含关闭按钮
                ui.horizontal(|ui| {
                    ui.with_layout(Layout::left_to_right(Align::Max), |ui| {
                        ui.add_space(10.0); // 左边距，调整以美观
                    });
                    ui.with_layout(Layout::centered_and_justified(egui::Direction::LeftToRight), |ui| {
                        ui.label("错误");  // 标题内容，居中显示
                    });
                    ui.with_layout(Layout::right_to_left(Align::Max), |ui| {
                        if ui.button("✖").clicked() {
                            self.error_msg = "".to_string();  // 设置关闭行为
                        }
                    });
                });

                // 窗口的主体内容
                ui.separator();
                ui.label(&self.error_msg);
            });
    }
}