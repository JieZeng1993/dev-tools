use egui::Ui;
use log::info;

mod tool;
mod util;

#[derive(PartialEq, Clone)]
pub enum ToolType {
    Unselected,
    FolderInfo(tool::FolderInfoTool),
    DecompressCompressStr(tool::DecompressCompressStrTool),
    JsonFormat(tool::JsonFormatTool),
}

impl ToolType {
    // 获得枚举的所有值
    pub fn iter() -> Vec<ToolType> {
        vec![ToolType::Unselected, ToolType::FolderInfo(tool::FolderInfoTool::new()), ToolType::DecompressCompressStr(tool::DecompressCompressStrTool::new())
             , ToolType::JsonFormat(tool::JsonFormatTool::new())]
    }

    // 将枚举转换为字符串表示，用于显示
    pub fn as_label(&self) -> &str {
        match self {
            ToolType::FolderInfo(_) => "文件夹信息",
            ToolType::DecompressCompressStr(_) => "解压、压缩字符串",
            ToolType::Unselected => { "请选择" }
            ToolType::JsonFormat(_) => { "json格式化" }
        }
    }
}

pub struct AppInstance {
    tool_type: ToolType,
}

impl AppInstance {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_custom_fonts(&cc.egui_ctx);
        Self {
            tool_type: ToolType::Unselected,
        }
    }
}

fn setup_custom_fonts(ctx: &egui::Context) {
    info!("字体初始化开始");
    // Start with the default fonts (we will be adding to them rather than replacing them).
    let mut fonts = egui::FontDefinitions::default();

    // Install my own font (maybe supporting non-latin characters).
    // .ttf and .otf files supported.
    fonts.font_data.insert(
        "msyh".to_owned(),
        egui::FontData::from_static(include_bytes!("..\\static\\font\\msyh.ttc")),
    );

    // Put my font first (highest priority) for proportional text:
    fonts.families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "msyh".to_owned());

    // Put my font as last fallback for monospace:
    fonts.families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("msyh".to_owned());

    // Tell egui to use these fonts:
    ctx.set_fonts(fonts);
    info!("字体初始化完成");
}


impl AppInstance {
    /// 设置工具类型
    pub fn change_tool_type(&mut self, tool_type: ToolType) {
        self.tool_type = tool_type;
    }

    /// 重新选择工具
    pub fn reselect_tool(&mut self, ui: &mut Ui) {
        if !ui.button("重新选择工具").clicked() {
            //未点击
            return;
        }
        self.tool_type = ToolType::Unselected;
    }
}

impl eframe::App for AppInstance {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // 顶部面板
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            if self.tool_type == ToolType::Unselected {
                //未选择工具时，选择工具
                ui.heading("请选择要使用的工具");
                egui::ComboBox::from_id_source("tool_type_check")
                    .selected_text(self.tool_type.as_label())
                    .show_ui(ui, |ui| {
                        for color in ToolType::iter() {
                            ui.selectable_value(&mut self.tool_type, color.clone(), color.as_label());
                        }
                    });
                return;
            }
            //设置全局的退出
            self.reselect_tool(ui);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let tool_type = &mut self.tool_type;
            match tool_type {
                ToolType::FolderInfo(folder_info) => {
                    ui.horizontal(|ui| {
                        folder_info.add_choose_folder_button(ui);
                        folder_info.show_select_file_info(ui);
                    });
                    folder_info.show_sub_file_info(ui);
                }
                ToolType::DecompressCompressStr(decompress_str) => {
                    decompress_str.show(ctx, ui);
                }
                ToolType::JsonFormat(json_format) => {
                    json_format.show(ctx, ui, frame);
                }
                ToolType::Unselected => {}
            }
        },
        );
    }
}