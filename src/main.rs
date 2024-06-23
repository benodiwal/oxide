mod ui;
mod env;
mod shell;
mod constants;

fn main() -> Result<(), eframe::Error> {
    if let Ok(stdout_fd) = shell::spawn_pty_with_shell() {
        ui::run(stdout_fd)?;
    };

    Ok(())
}
