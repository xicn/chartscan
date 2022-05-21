use std::fs::File;

use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct SpotifyEntry {
    rank: u8,
    title: String,
    artist: String,
    streams: String,
}

pub fn from_reader(f: File) -> Result<Vec<SpotifyEntry>, Box<dyn std::error::Error>> {
    let mut res = Vec::new();
    let mut csv_rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'*')
        .from_reader(f);

    for rec in csv_rdr.deserialize() {
        let entry: SpotifyEntry = rec?;
        res.push(entry);
    }

    Ok(res)
}
