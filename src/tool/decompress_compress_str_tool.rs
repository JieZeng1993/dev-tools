use std::io::{Read, Write};
use std::time::Duration;

use base64::Engine;
use base64::engine::general_purpose;
use eframe::emath::Align;
use egui::{Button, Context, Layout, TextEdit, Ui};
use flate2::{Compression, write::GzEncoder};
use log::{debug, info};

use crate::tool::get_seconds_since_epoch;

#[derive(PartialEq, Clone)]
pub struct DecompressCompressStrTool {
    pub source_text: String,
    pub result_text: String,
    pub error_msg: String,
    pub error_start: u64,
    pub show_count: u64,
}

impl DecompressCompressStrTool {
    pub fn new() -> Self {
        Self {
            source_text: "".to_string(),
            result_text: "".to_string(),
            error_msg: "".to_string(),
            error_start: 1,
            show_count: 0,
        }
    }

    pub fn show(&mut self, ctx: &egui::Context, tool_main_ui: &mut Ui) {
        self.show_count = self.show_count + 1;
        debug!("DecompressCompressStrTool show:{}", self.show_count);
        tool_main_ui.horizontal_top(|tool_main_ui| {
            let available_width = tool_main_ui.available_width();
            // 使用相同的尺寸配置两个 TextEdit 控件
            let half_width = (available_width - 40.0) * 0.5;
            let text_edit_size = egui::vec2(half_width, tool_main_ui.available_height());


            egui::ScrollArea::vertical().id_source("DecompressCompressStrTool compressed ScrollArea").max_height(tool_main_ui.available_height()).show(tool_main_ui, |tool_main_ui| {
                tool_main_ui.add_sized(text_edit_size, TextEdit::multiline(&mut self.source_text).hint_text("输入原始字符串"));
            });

            //展示错误消息
            self.show_error_msg(ctx);
            //显示转换按钮
            self.show_transfer_button(ctx, tool_main_ui);

            egui::ScrollArea::vertical().id_source("DecompressCompressStrTool origin_info ScrollArea").max_height(tool_main_ui.available_height()).show(tool_main_ui, |tool_main_ui| {
                tool_main_ui.add_sized(text_edit_size, TextEdit::multiline(&mut self.result_text).hint_text("显示解压缩后的字符串"));
            });
        });
    }

    /// 显示转换按钮
    fn show_transfer_button(&mut self, ctx: &Context, tool_main_ui: &mut Ui) {
        tool_main_ui.vertical(|ui| {
            if ui.add(Button::new(">>")).clicked() {
                let decompress_result = self.decompress(self.source_text.clone());
                if decompress_result.is_err() {
                    self.error_msg = format!("解压失败:{}", decompress_result.unwrap_err());
                    self.error_start = get_seconds_since_epoch();
                    ctx.request_repaint_after(Duration::from_secs(1));
                } else {
                    self.result_text = decompress_result.unwrap();
                }
            }
            ui.add_space(10.0);
            if ui.add(Button::new("<<")).clicked() {
                let compress_result = self.compress(self.result_text.clone());
                if compress_result.is_err() {
                    self.error_msg = format!("压缩失败:{}", compress_result.unwrap_err());
                    self.error_start = get_seconds_since_epoch();
                    ctx.request_repaint_after(Duration::from_secs(1));
                } else {
                    self.source_text = compress_result.unwrap();
                }
            }
        });
    }

    /// 暂时错误信息
    /// TODO 消息弹窗需要实现全局
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


    pub fn decompress(&mut self, str: String) -> Result<String, Box<dyn std::error::Error>> {
        let bytes = general_purpose::STANDARD
            .decode(str)?;
        let mut decompress_decoder = flate2::read::GzDecoder::new(bytes.as_slice());
        let mut decompress_str = String::new();
        decompress_decoder.read_to_string(&mut decompress_str)?;
        return Ok(decompress_str);
    }

    pub fn compress(&mut self, str: String) -> Result<String, Box<dyn std::error::Error>> {
        // 创建一个 GzEncoder，用于压缩数据
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(str.as_bytes())?;
        let compressed_bytes = encoder.finish()?;

        // 将压缩后的数据进行 Base64 编码
        return Ok(general_purpose::STANDARD.encode(compressed_bytes));
    }
}