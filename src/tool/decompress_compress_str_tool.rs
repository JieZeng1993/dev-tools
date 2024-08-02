use std::io::{Read, Write};

use base64::Engine;
use base64::engine::general_purpose;
use egui::{Button, Id, popup_below_widget, PopupCloseBehavior, TextEdit, Ui};
use flate2::{Compression, write::GzEncoder};
use log::{error, info};

#[derive(PartialEq, Clone)]
pub struct DecompressCompressStrTool {
    pub source_text: String,
    pub result_text: String,
}

impl DecompressCompressStrTool {
    pub fn new() -> Self {
        Self {
            source_text: "".to_string(),
            result_text: "".to_string(),
        }
    }

    pub fn show(&mut self, ctx: &egui::Context, tool_main_ui: &mut Ui) {
      let response =   tool_main_ui.separator();


        tool_main_ui.horizontal_top(|tool_main_ui| {
            let available_width = tool_main_ui.available_width();
            let height = tool_main_ui.spacing().interact_size.y;

            // 使用相同的尺寸配置两个 TextEdit 控件
            let half_width = available_width * 0.49;
            let text_edit_size = egui::vec2(half_width, height);

          tool_main_ui.add_sized(text_edit_size, TextEdit::multiline(&mut self.source_text).hint_text("输入原始字符串"));

            let screen_rect = ctx.available_rect(); // 获取可用屏幕区域
            let window_size = egui::vec2(300.0, 200.0); // 指定窗口大小

            // 计算居中位置
            let position = egui::pos2(
                screen_rect.center().x - window_size.x / 2.0,
                screen_rect.center().y - window_size.y / 2.0,
            );

            egui::Window::new("错误")
                .collapsible(false)
                .fixed_pos(position) // 设置窗口位置
                .fixed_size(window_size) // 设置窗口大小
                .show(ctx, |ui| {
                    ui.label("我是错误消息");
                });

            tool_main_ui.vertical(|ui| {
                if ui.add(Button::new(">>")).clicked() {
                    let decompress_result = self.decompress(self.source_text.clone());
                    if decompress_result.is_err() {
                        error!("解密失败");

                    } else {
                        self.result_text = decompress_result.unwrap();
                    }
                }
                ui.add_space(10.0);
                if ui.add(Button::new("<<")).clicked() {
                    let compress_result = self.compress(self.result_text.clone());
                    if compress_result.is_err() {} else {
                        self.source_text = compress_result.unwrap();
                    }
                }
            });
            tool_main_ui.add_sized(text_edit_size, TextEdit::multiline(&mut self.result_text).hint_text("显示解压缩后的字符串"));
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