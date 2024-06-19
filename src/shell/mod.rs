use std::os::fd::AsRawFd;
use std::{os::fd::OwnedFd, process::Command};
use nix::pty::{forkpty, ForkptyResult};
use nix::unistd::read;

pub fn read_from_fd(fd: &OwnedFd) -> Option<Vec<u8>> {
    let mut read_buffer = [0; 65536];
    let read_result = read(fd.as_raw_fd(), &mut read_buffer);
    match read_result {
        Ok(bytes_read) => Some(read_buffer[..bytes_read].to_vec()),
        Err(_) => None, 
    }
}

pub fn spawn_pty_with_shell(shell: String) -> OwnedFd {
    unsafe{
        let res = forkpty(None, None);
        match res {
            Ok(fork_pty_res) => {
                if let ForkptyResult::Parent { child, master } = fork_pty_res {
                    println!("Child Process: {:?}", child);
                    master
                } else {
                    Command::new(shell).spawn().expect("failed to spawn");
                    std::thread::sleep(std::time::Duration::from_millis(1000));
                    std::process::exit(0);
                }
            },
            Err(err) => {
                panic!("failed to fork {:?}", err);
            },
        }
    }
}
