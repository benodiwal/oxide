mod ui;
mod env;
mod shell;
mod constants;

fn main() -> Result<(), eframe::Error> {
    let default_shell = env::read_env(constants::SHELL);    
    let stdout_fd = shell::spawn_pty_with_shell(default_shell);

    ui::run(stdout_fd)?;
    Ok(())
}
