use std::os::fd::{AsFd, AsRawFd, OwnedFd};
use eframe::{egui, Result};
use crate::{constants, shell};
use nix::fcntl::{fcntl, FcntlArg, OFlag};

mod util;

pub fn run(fd: OwnedFd) -> Result<()> {
    eframe::run_native(constants::OXIDE, util::native_options(), Box::new(|cc| Box::new(Oxide::new(cc, fd))))
}

struct Oxide {
    buffer: Vec<u8>,
    fd: OwnedFd,
}

impl Oxide {
    fn new(_: &eframe::CreationContext<'_>, fd: OwnedFd) -> Self {
        let flags = fcntl(fd.as_raw_fd(), FcntlArg::F_GETFL).unwrap();
        let mut flags = OFlag::from_bits(flags & OFlag::O_ACCMODE.bits()).unwrap();
        flags.set(OFlag::O_NONBLOCK, true);
        fcntl(fd.as_raw_fd(), FcntlArg::F_SETFL(flags)).unwrap();

        Oxide {
            buffer: Vec::new(),
            fd
        }
    }
}

impl eframe::App for Oxide {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {

        if let Some(buf) = shell::read_from_fd(&self.fd) {
            self.buffer.extend_from_slice(&buf);
        }
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.input(|input_state| {
                for event in &input_state.events {

                    let text = match event {
                        egui::Event::Text(text) => text,
                        egui::Event::Key { key: egui::Key::Enter, pressed: true, ..} => "\n",
                        _ => ""
                    };

                    let bytes = text.as_bytes();
                    let mut to_write: &[u8] = bytes;
                    
                    while !to_write.is_empty() {
                        let written = nix::unistd::write(self.fd.as_fd(), to_write).unwrap();
                        to_write = &to_write[written..];
                    }
                }
            });
            
            unsafe{
                ui.label(std::str::from_utf8_unchecked(&self.buffer));
            }
        });
    }
}
