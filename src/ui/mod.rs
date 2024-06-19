use eframe::{egui, Result};
mod util;

pub fn run() -> Result<()> {
    eframe::run_native("Oxide", util::native_options(), Box::new(|cc| Box::new(Oxide::new(cc))))
}

#[derive(Default)]
struct Oxide {}

impl Oxide {
    fn new(_: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for Oxide {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
       egui::CentralPanel::default().show(ctx, |ui| {
           ui.heading("Hello World!");
       });
    }
}
