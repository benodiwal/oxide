use std::sync::{Arc, Mutex};
use eframe::{egui, Result};
use crate::constants;

mod util;

pub fn run(shared_buffer: Arc<Mutex<Vec<u8>>>) -> Result<()> {
    eframe::run_native(constants::OXIDE, util::native_options(), Box::new(|cc| Box::new(Oxide::new(cc, shared_buffer))))
}

#[derive(Default)]
struct Oxide {
    shared_buffer: Arc<Mutex<Vec<u8>>>,
}

impl Oxide {
    fn new(_: &eframe::CreationContext<'_>, shared_buffer: Arc<Mutex<Vec<u8>>>) -> Self {
        Self { shared_buffer }
    }
}

impl eframe::App for Oxide {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        let buffer = self.shared_buffer.lock().unwrap();
        let output_text = String::from_utf8_lossy(&buffer);
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");
            ui.text_edit_multiline(&mut output_text.to_string());
        });
    }
}
