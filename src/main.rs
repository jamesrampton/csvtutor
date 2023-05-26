use std::{error::Error, io, process};

fn main() {
    if let Err(err) = run() {
        println!("error reading csv from <stdin>: {}", err);
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.records() {
        match result {
            Err(err) => return Err(From::from(err)),
            Ok(record) => println!("{:?}", record),
        }
    }
    Ok(())
}
