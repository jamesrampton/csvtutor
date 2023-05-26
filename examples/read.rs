use serde::Deserialize;
use std::{env, error::Error, ffi::OsString, process};

// type Record = (String, String, Option<u64>, f64, f64);
// type Record = HashMap<String, String>;
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Record {
    latitude: f64,
    longitude: f64,
    #[serde(deserialize_with = "csv::invalid_option")]
    population: Option<u64>,
    city: String,
    state: String,
}

fn run() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;
    let mut rdr = csv::ReaderBuilder::new().from_path(file_path)?;
    for result in rdr.deserialize() {
        let record: Record = result?;
        println!("{:#?}", record);
    }
    Ok(())
}

fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn main() {
    if let Err(err) = run() {
        println!("error reading csv from <stdin>: {}", err);
        process::exit(1);
    }
}
