use std::{env, error::Error, ffi::OsString, process};

fn run() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;
    let mut wtr = csv::WriterBuilder::new().from_path(file_path)?;

    wtr.write_record(&["City", "State", "Population", "Latitude", "Longitude"])?;
    wtr.serialize((
        "Davidsons Landing",
        "AK",
        None::<u64>,
        65.2419444,
        -165.2716667,
    ))?;
    wtr.serialize(("Kenai", "AK", Some(7610), 60.5544444, -151.2583333))?;
    wtr.serialize(("Oakman", "AL", None::<u64>, 33.7133333, -87.3886111))?;
    wtr.flush()?;

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
        println!("{}", err);
        process::exit(1);
    }
}
