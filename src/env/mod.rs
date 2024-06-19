use std::env::{self, VarError};

pub fn read_env(key: &str) -> String {
    let res = env::var(key);
    match res {
        Ok(value) => value,
        Err(err) => match err {
            VarError::NotPresent => panic!("Environment variable {} is not set", key),
            VarError::NotUnicode(_) => panic!("Environment variable {} is not valid Unicode", key),
        }
    }
}
