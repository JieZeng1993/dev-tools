use log::{info, warn};
use eframe::{egui};
use rfd::FileDialog;
use std::fs;



struct App {
    folder_path: String,
    files: Vec<String>,
}

impl App {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_custom_fonts(&cc.egui_ctx);
        Self {
            folder_path: String::new(),
            files: Vec::new(),
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
            if ui.button("Choose Folder").clicked() {
                if let Some(path) = FileDialog::new().pick_folder() {
                    self.folder_path = path.display().to_string();
                    self.files.clear();
                    if let Ok(entries) = fs::read_dir(&path) {
                        for entry in entries.flatten() {
                            if let Ok(metadata) = entry.metadata() {
                                if metadata.is_file() {
                                    if let Some(name) = entry.file_name().to_str() {
                                        self.files.push(name.to_owned());
                                    }
                                }
                            }
                        }
                    }
                }
            }

            if !self.folder_path.is_empty() {
                ui.label(format!("Selected folder: {}", self.folder_path));
                ui.separator();
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for file in &self.files {
                        ui.label(file);
                    }
                });
            }
        });
    }
}

fn main() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    info!("log4rs init finish");

    let mut options = eframe::NativeOptions::default();
    options.centered = true;
    eframe::run_native("File Explorer", options, Box::new(|_cc| Ok(Box::new(App::new(_cc)))));
}
