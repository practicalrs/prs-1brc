use std::{env, fs::File, io::BufReader, io::prelude::*};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub enum Error {
    CommandLine,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error!")
    }
}

pub fn run() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let file_name = args.last().ok_or(Error::CommandLine)?;

    let file = File::open(file_name)?;
    let mut reader = BufReader::with_capacity(1073741824, file);

    let mut line = String::new();

    let mut i = 1;
    loop {
        let len = reader.read_line(&mut line)?;

        println!("i = {i} {len}");
        i += 1;

        if len == 0 {
            break;
        }
    }

    Ok(())
}
