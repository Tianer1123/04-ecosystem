use anyhow::Context;
use std::{fs, num::ParseIntError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("I/O error")]
    Io(#[from] std::io::Error),
    #[error("Parse error")]
    Parse(#[from] std::num::ParseIntError),
    #[error("Serialize error: {0}")]
    Serialize(#[from] serde_json::Error),

    #[error("Error: {0:?}")]
    BigError(Box<BigError>),

    #[error("Custom error: {0}")]
    Custom(String),
}

#[allow(unused)]
#[derive(Debug)]
pub struct BigError {
    a: String,
    b: Vec<String>,
    c: [u8; 64],
    d: u64,
}

fn main() -> Result<(), anyhow::Error> {
    println!("size of anyhow::Error is {}", size_of::<anyhow::Error>());
    println!("size of std::io::Error is {}", size_of::<std::io::Error>());
    println!(
        "size of std::num::ParseIntError is {}",
        size_of::<ParseIntError>()
    );
    println!(
        "size of serde_json::Error is {}",
        size_of::<serde_json::Error>()
    );
    println!("size of string is {}", size_of::<String>());
    println!("size of MyError is {}", size_of::<MyError>());

    let filename = "non-existent-file.txt";
    let _ = fs::File::open(filename).with_context(|| format!("can not find file: {}", filename))?;

    fail_with_error()?;

    Ok(())
}

fn fail_with_error() -> Result<(), MyError> {
    Err(MyError::Custom("this is a custom error".to_string()))
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
