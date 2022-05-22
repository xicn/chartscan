#![allow(dead_code)]

use serde::Deserialize;
use std::{error::Error, fs::File, num::NonZeroU8};

const FORMAT: u128 = lexical_core::NumberFormatBuilder::new()
    .digit_separator(NonZeroU8::new(b','))
    .required_digits(true)
    .no_positive_mantissa_sign(true)
    .no_special(true)
    .internal_digit_separator(true)
    .trailing_digit_separator(true)
    .consecutive_digit_separator(true)
    .build();

#[derive(Debug, Deserialize, PartialEq)]
pub struct SpotifyEntry {
    rank: u8,
    title: String,
    artist: String,
    streams: u64,
}

impl SpotifyEntry {
    fn new(rank: u8, title: String, artist: String, streams: u64) -> Self {
        SpotifyEntry {
            rank,
            title,
            artist,
            streams,
        }
    }
}

#[derive(Debug)]
pub struct SpotifyChart {
    region: String,
    code: String,
    date: String,
    chart: [SpotifyEntry; 200],
    count: u8,
}

type Record = (u8, String, String, String);

pub fn from_reader(f: File) -> Result<Vec<SpotifyEntry>, Box<dyn std::error::Error>> {
    let mut res = Vec::new();
    let mut csv_rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'*')
        .from_reader(f);

    for rec in csv_rdr.deserialize() {
        let (rank, title, artist, _streams): Record = rec?;

        res.push(SpotifyEntry::new(rank, title, artist, 0));
    }

    Ok(res)
}

fn parse_int(_num: &str) -> Result<u64, Box<dyn Error>> {
    let options = lexical_core::ParseFloatOptions::builder()
        .decimal_point(b'.')
        .build()
        .unwrap();
    let res = lexical_core::parse_with_options::<f64, FORMAT>(_num.as_bytes(), &options)?;

    Ok(res as u64)
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn lexical_1() {
        const FORMAT: u128 = lexical_core::NumberFormatBuilder::new()
            .digit_separator(NonZeroU8::new(b','))
            .required_digits(true)
            .no_positive_mantissa_sign(true)
            .no_special(true)
            .internal_digit_separator(true)
            .trailing_digit_separator(true)
            .consecutive_digit_separator(true)
            .build();

        let options = lexical_core::ParseFloatOptions::builder()
            .decimal_point(b'.')
            .build()
            .unwrap();
        assert_eq!(
            lexical_core::parse_with_options::<f32, FORMAT>("300,100".as_bytes(), &options),
            Ok(300_100.0 as f32)
        );
    }

    #[test]
    fn parse_int_1() -> Result<(), Box<dyn Error>> {
        let expected: u64 = 10;
        assert_eq!(parse_int("10")?, expected);
        Ok(())
    }

    #[test]
    fn parse_int_2() -> Result<(), Box<dyn Error>> {
        let expected: u64 = 10000;
        assert_eq!(parse_int("10,000")?, expected);
        Ok(())
    }

    #[test]
    fn parse_int_3() -> Result<(), Box<dyn Error>> {
        let expected: u64 = 1676272;
        assert_eq!(parse_int("1,676,272")?, expected);
        Ok(())
    }

    #[test]
    fn test_from_reader() -> Result<(), Box<dyn Error>> {
        let expected = SpotifyEntry {
            rank: 1,
            title: "N95".to_string(),
            artist: "Kendrick Lamar".to_string(),
            streams: 1,
        };
        let f = File::open("/home/ubuntu/project/chartscan/SpotifyData/US/2022-05-18.csv")?;
        let vec = from_reader(f)?;
        let actual = vec.get(0).unwrap();
        assert_eq!(*actual, expected);
        Ok(())
    }
}
