use std::{collections::BTreeMap, env, fs::File, io::BufReader, io::prelude::*};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub enum Error {
    CommandLine,
    Database,
    Parsing,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error!")
    }
}

#[derive(Debug)]
struct Db {
    data: BTreeMap<String, Vec<f32>>,
}

impl Db {
    fn new() -> Self {
        Self {
            data: BTreeMap::new(),
        }
    }

    fn insert(&mut self, key: String, value: f32) -> Result<()> {
        if self.data.contains_key(&key) {
            let mut new_values = vec![value];
            let values = self.data.get(&key).ok_or(Error::Database)?;
            new_values.append(&mut values.clone());
            self.data.insert(key, new_values);
        } else {
            self.data.insert(key, vec![value]);
        }

        Ok(())
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

//        println!("{line}");
        let values = line.strip_suffix('\n').ok_or(Error::Parsing)?.split(';').collect::<Vec<&str>>();
//        println!("values = {:?}", values);
        let key = values.first().ok_or(Error::Parsing)?.to_string();
        let value = values.last().ok_or(Error::Parsing)?.parse::<f32>()?;
//        println!("name = {name}, value = {value}");
        println!("i = {i} {len}");
        i += 1;

        db.insert(key, value)?;

        if len == 0 {
            break;
        }
    }

    println!("DB = {:?}", db);

    Ok(())
}
