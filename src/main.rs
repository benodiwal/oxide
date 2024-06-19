use nix::pty::ForkptyResult;

mod ui;
mod env;
mod shell;

fn main() -> Result<(), eframe::Error> {
    let shell = env::read_env(shell::SHELL);
    println!("{}", shell);

    unsafe {
        let res = nix::pty::forkpty(None, None).unwrap();
        match res {
            ForkptyResult::Parent { child, master } => {
                println!("Child {:?}", child);
                println!("Master {:?}", master);
            },
            ForkptyResult::Child => {
                println!("Child Process");
            },
        }
    }
    ui::run()
}
