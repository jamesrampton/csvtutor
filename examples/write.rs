use std::{error::Error, process};

fn run() -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_writer(std::io::stdout());

    wtr.write_record(&["City", "State", "Population", "Latitude", "Longitude"])?;
    wtr.write_record(&["Davidsons Landing", "AK", "", "65.2419444", "-165.2716667"])?;
    wtr.write_record(vec!["Kenai", "AK", "7610", "60.5544444", "-151.2583333"])?;
    wtr.write_record(&csv::StringRecord::from(vec![
        "Oakman",
        "AL",
        "",
        "33.7133333",
        "-87.3886111",
    ]))?;
    wtr.flush()?;

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!("error reading csv from <stdin>: {}", err);
        process::exit(1);
    }
}
