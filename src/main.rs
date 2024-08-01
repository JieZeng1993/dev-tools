#![windows_subsystem = "windows"]
use std::fs;

use eframe::egui;
use egui::Ui;
use log::{info, warn};
use rfd::FileDialog;

use dev_tool_lib::{FolderTree, ToolType};


struct App {
    tool_type: ToolType,
}

impl App {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_custom_fonts(&cc.egui_ctx);
        Self {
            tool_type: ToolType::Unselected,
        }
    }
}

fn setup_custom_fonts(ctx: &egui::Context) {
    info!("font init start");
    // Start with the default fonts (we will be adding to them rather than replacing them).
    let mut fonts = egui::FontDefinitions::default();

    // Install my own font (maybe supporting non-latin characters).
    // .ttf and .otf files supported.
    fonts.font_data.insert(
        "msyh".to_owned(),
        egui::FontData::from_static(include_bytes!(
            "C:\\Windows\\Fonts\\msyh.ttc"
        )),
    );

    // Put my font first (highest priority) for proportional text:
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "msyh".to_owned());

    // Put my font as last fallback for monospace:
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("msyh".to_owned());

    // Tell egui to use these fonts:
    ctx.set_fonts(fonts);
    info!("font init finish");
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.tool_type == ToolType::Unselected {
                //未选择工具时，选择工具
                ui.heading("请选择要使用的工具");
                egui::ComboBox::from_label("Colors")
                    .show_ui(ui, |ui| {
                        for color in ToolType::iter() {
                            ui.selectable_value(&mut self.tool_type, color.clone(), color.as_label());
                        }
                    });

                // // 显示当前选择的颜色
                // ui.label(format!("Selected color: {}", self.selected_color.as_str()));

                return;
            }

            let tool_type =  &mut self.tool_type;
            match tool_type {
                ToolType::FolderTree(folder_tree) => {
                    folder_tree.add_choose_folder_button(ui);
                    folder_tree.show_sub_file_info(ui);
                }
                ToolType::DecompressStr => {}
                ToolType::Unselected => {}
            }
        },
        );
    }
}

fn main() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    info!("log4rs init finish");

    let mut options = eframe::NativeOptions::default();
    options.centered = true;
    eframe::run_native("File Explorer", options, Box::new(|_cc| Ok(Box::new(App::new(_cc)))));
}
