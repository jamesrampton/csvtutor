use std::{env, error::Error, ffi::OsString, process};

fn run() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b';')
        .double_quote(false)
        .escape(Some(b'\\'))
        .flexible(true)
        .comment(Some(b'#'))
        .from_path(file_path)?;
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
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
