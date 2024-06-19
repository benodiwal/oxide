use std::{sync::{Arc, Mutex}, thread};

mod ui;
mod env;
mod shell;
mod constants;

fn main() -> Result<(), eframe::Error> {
    let default_shell = env::read_env(constants::SHELL);
    let stdout_fd = shell::spawn_pty_with_shell(default_shell);
    let shared_buffer: Arc<Mutex<Vec<u8>>> = Arc::new(Mutex::new(Vec::new()));
    let shared_buffer_clone = shared_buffer.clone();

    thread::spawn(move || {
        while let Some(read_bytes) = shell::read_from_fd(&stdout_fd) {
            let mut buffer = shared_buffer_clone.lock().unwrap();
            buffer.extend_from_slice(&read_bytes);
        }
    });

    ui::run(shared_buffer)?;
    Ok(())
}
