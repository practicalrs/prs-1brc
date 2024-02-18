use std::{collections::BTreeMap, env, fs::File, io::BufReader, io::prelude::*};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub enum Error {
    CommandLine,
    Parsing,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error!")
    }
}

struct Db {
    data: BTreeMap<String, Vec<f32>>,
}

impl Db {
    fn new() -> Self {
        Self {
            data: BTreeMap::new(),
        }
    }
}

pub fn run() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let file_name = args.last().ok_or(Error::CommandLine)?;

    let file = File::open(file_name)?;
    let mut reader = BufReader::with_capacity(1073741824, file);

    let mut line = String::new();

    let mut i = 1;

    let mut db = Db::new();

    loop {
        let len = reader.read_line(&mut line)?;

        println!("{line}");
        let values = line.strip_suffix('\n').ok_or(Error::Parsing)?.split(';').collect::<Vec<&str>>();
        println!("values = {:?}", values);
        let name = values.first().ok_or(Error::Parsing)?;
        let value = values.last().ok_or(Error::Parsing)?.parse::<f32>()?;
        println!("name = {name}, value = {value}");
        println!("i = {i} {len}");
        i += 1;

        if len == 0 {
            break;
        }
    }

    Ok(())
}
