use std::ffi::CStr;
use std::os::fd::AsRawFd;
use std::os::fd::OwnedFd;
use std::process::exit;
use nix::pty::{forkpty, ForkptyResult};
use nix::unistd::execvp;
use nix::unistd::read;

pub fn read_from_fd(fd: &OwnedFd) -> Option<Vec<u8>> {
    let mut read_buffer = [0; 65536];
    let read_result = read(fd.as_raw_fd(), &mut read_buffer);
    match read_result {
        Ok(bytes_read) => Some(read_buffer[..bytes_read].to_vec()),
        Err(_) => None, 
    }
}

pub fn spawn_pty_with_shell() -> Result<OwnedFd, String> {
    unsafe{
        let res = forkpty(None, None);
        match res {
            Ok(fork_pty_res) => {
                if let ForkptyResult::Parent { child, master } = fork_pty_res {
                    println!("Child Process: {:?}", child);
                    Ok(master)
                } else {
                    let shell_name = CStr::from_bytes_until_nul(b"bash\0")
                    .map_err(|err| format!("Failed to create a shell name CStr: {:?}", err))?;

                    let args: &[&[u8]] = &[b"--bash\0", b"--noprofile\0", b"--norc\0"];

                    let args = args.iter()
                    .map(|v| CStr::from_bytes_with_nul(v).map_err(|e| format!("Failed to create args CStr: {:?}", e)))
                    .collect::<Result<Vec<_>, _>>()?;
                     
                     std::env::remove_var("PROMPT_COMMAND");
                     std::env::set_var("PS1", "\\$ ");
                     if let Err(err) = execvp(shell_name, &args) {
                        eprintln!("Failed to execvp: {:?}", err);
                        exit(1);
                     }
                     unreachable!("execvp should not return");
                }
            },
            Err(err) => {
                panic!("failed to fork {:?}", err);
            },
        }
    }
}
