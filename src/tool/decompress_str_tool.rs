use egui::{TextEdit, Ui};

#[derive(PartialEq, Clone)]
pub struct DecompressStrTool {
    pub source_text: String,
    pub result_text: String,
}

impl DecompressStrTool {
    pub fn new() -> Self {
        Self {
            source_text: "".to_string(),
            result_text: "".to_string(),
        }
    }

    pub fn show(&mut self, ctx: &egui::Context, ui: &mut Ui) {
        ui.horizontal(|ui| {
            let source_text = ui.add(TextEdit::multiline(&mut self.source_text).hint_text("输入原始字符串"));
            if source_text.lost_focus() {
                self.result_text = self.source_text.clone();
            }
            ui.add(TextEdit::multiline(&mut self.result_text).hint_text("显示解压缩后的字符串"));
        });
    }
}