mod ui;
mod env;
mod shell;
mod constants;

fn main() -> Result<(), eframe::Error> {
    let default_shell = env::read_env(constants::SHELL);
    let stdout_fd = shell::spawn_pty_with_shell(default_shell);
    let mut read_buffer = vec![];

    loop {
        match shell::read_from_fd(&stdout_fd) {
            Some(mut read_bytes) => {
                read_buffer.append(&mut read_bytes);
            },
            None => {
                println!("{:?}", String::from_utf8(read_buffer).unwrap());
                std::process::exit(1);
            }
        }
    }
}
