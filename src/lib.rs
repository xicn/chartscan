use std::{env::args_os, fs::File};
mod spotify;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    match args_os().nth(1) {
        Some(path) => {
            let f = File::open(path)?;
            let mut csv_rdr = csv::ReaderBuilder::new()
                .has_headers(false)
                .delimiter(b'*')
                .from_reader(f);

            for rec in csv_rdr.deserialize() {
                let r: spotify::SpotifyEntry = rec?;
                println!("{:?}", r);
            }
        }
        None => return Err(From::from("Missing required path to the file")),
    }

    Ok(())
}
