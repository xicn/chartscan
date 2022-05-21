use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct SpotifyEntry {
    rank: u8,
    title: String,
    artist: String,
    streams: String,
}
