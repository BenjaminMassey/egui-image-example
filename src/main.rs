use eframe::egui;
use std::fs;

const PATH: &str = "./image.png";

fn main() {
    let _ = eframe::run_native(
        "Window",
        eframe::NativeOptions {
            ..Default::default()
        },
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::new(EguiApp::new(cc))
        }),
    );
}

#[derive(Default)]
struct EguiApp {}

impl EguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.include_bytes(
            "bytes://".to_owned() + PATH,
            egui::load::Bytes::from(fs::read(PATH).unwrap()),
        );
        Self {}
    }
}

impl eframe::App for EguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_sized(
                egui::vec2(250.0, 250.0),
                egui::Image::new("bytes://".to_owned() + PATH),
            );
        });
    }
}
