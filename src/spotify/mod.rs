#![allow(dead_code)]

use regex::Regex;
use std::{error::Error, fs::File, num::NonZeroU8};
use time::Date;

#[derive(Debug, PartialEq)]
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
    date: Date,
    chart: [SpotifyEntry; 200],
    count: u8,
}

impl SpotifyChart {
    // fn new(region: String, code: String, date: String) -> Self {
    //     SpotifyChart {
    //         region: (),
    //         code: (),
    //         date: (),
    //         chart: (),
    //         count: (),
    //     }
    // }
}

fn match_date(date: &str) -> Option<(i32, u8, u8)> {
    let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
    let caps = re.captures(&date).unwrap();

    let year = caps[1].parse::<i32>().unwrap();
    let month = caps[2].parse::<u8>().unwrap();
    let day = caps[3].parse::<u8>().unwrap();

    // let date = Date::from_calendar_date(
    //     caps[1].parse::<i32>().unwrap(),
    //     caps[2].parse::<u8>().unwrap(),
    //     caps[3].parse::<u8>().unwrap(),
    // );

    Some((year, month, day))
}

type Record = (u8, String, String, String);

pub fn from_reader(f: File) -> Result<Vec<SpotifyEntry>, Box<dyn std::error::Error>> {
    let mut res = Vec::new();
    let mut csv_rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'*')
        .from_reader(f);

    for rec in csv_rdr.deserialize() {
        let (rank, title, artist, streams): Record = rec?;

        res.push(SpotifyEntry::new(rank, title, artist, parse_int(&streams)?));
    }

    Ok(res)
}

fn parse_int(num: &str) -> Result<u64, Box<dyn Error>> {
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
    let res = lexical_core::parse_with_options::<f64, FORMAT>(num.as_bytes(), &options)?;

    Ok(res as u64)
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn match_date_1() -> Result<(), Box<dyn Error>> {
        let expected = (2022, 1, 27);
        assert_eq!(expected, match_date("2022-01-27").unwrap());
        Ok(())
    }

    #[test]
    fn match_date_2() -> Result<(), Box<dyn Error>> {
        let expected = (2021, 5, 27);
        assert_eq!(expected, match_date("2021-05-27").unwrap());
        Ok(())
    }

    #[test]
    fn match_date_3() -> Result<(), Box<dyn Error>> {
        let expected = (2018, 9, 1);
        assert_eq!(expected, match_date("2018-09-01").unwrap());
        Ok(())
    }

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
    fn test_from_reader_1_first() -> Result<(), Box<dyn Error>> {
        let expected = SpotifyEntry {
            rank: 1,
            title: "N95".to_string(),
            artist: "Kendrick Lamar".to_string(),
            streams: parse_int("1,676,272")?,
        };
        let f = File::open("/home/ubuntu/project/chartscan/SpotifyData/US/2022-05-18.csv")?;
        let vec = from_reader(f)?;
        let actual = vec.get(0).unwrap();
        assert_eq!(*actual, expected);
        Ok(())
    }

    #[test]
    fn test_from_reader_1_middle() -> Result<(), Box<dyn Error>> {
        let expected = SpotifyEntry {
            rank: 100,
            title: "Nail Tech".to_string(),
            artist: "Jack Harlow".to_string(),
            streams: parse_int("331,769")?,
        };
        let f = File::open("/home/ubuntu/project/chartscan/SpotifyData/US/2022-05-18.csv")?;
        let vec = from_reader(f)?;
        let actual = vec.get(99).unwrap();
        assert_eq!(*actual, expected);
        Ok(())
    }

    #[test]
    fn test_from_reader_1_last() -> Result<(), Box<dyn Error>> {
        let expected = SpotifyEntry {
            rank: 200,
            title: "Good Days".to_string(),
            artist: "SZA".to_string(),
            streams: parse_int("244,228")?,
        };
        let f = File::open("/home/ubuntu/project/chartscan/SpotifyData/US/2022-05-18.csv")?;
        let vec = from_reader(f)?;
        let actual = vec.get(199).unwrap();
        assert_eq!(*actual, expected);
        Ok(())
    }

    #[test]
    fn test_from_reader_2() -> Result<(), Box<dyn Error>> {
        let f = File::open("/home/ubuntu/project/chartscan/SpotifyData/US/2022-05-18.csv")?;
        let vec = from_reader(f)?;
        assert_eq!(200, vec.len());
        Ok(())
    }
}
