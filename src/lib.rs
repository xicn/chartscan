use std::{env::args_os, fs::File};

// use spotify::SpotifyEntry;
mod spotify;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    match args_os().nth(1) {
        Some(path) => {
            let f = File::open(path)?;
            println!("{:?}", f);
        }
        None => return Err(From::from("Missing required path to the file")),
    }

    Ok(())
}
