use std::{io, process};

fn main() {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.records() {
        match result {
            Ok(record) => println!("{:?}", record),
            Err(err) => {
                println!("error reading csv from <stdin>: {}", err);
                process::exit(1);
            }
        }
    }
}
