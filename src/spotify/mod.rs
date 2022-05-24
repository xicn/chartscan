#![allow(dead_code)]

mod regions;

use regex::Regex;
use std::{error::Error, fs::File, num::NonZeroU8};
use time::{Date, Month};

use self::regions::Regions;

#[derive(Debug, PartialEq)]
pub struct SpotifyEntry {
    rank: i16,
    title: String,
    artist: String,
    streams: i64,
}

impl SpotifyEntry {
    fn new(rank: i16, title: String, artist: String, streams: i64) -> Self {
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
    chart: Vec<SpotifyEntry>,
    count: u8,
}

impl SpotifyChart {
    pub fn new() -> Self {
        SpotifyChart {
            region: String::from("Unkown"),
            code: String::from("Unkown"),
            date: Date::from_calendar_date(2001, Month::January, 27).unwrap(), // This date should be always valid
            chart: Vec::new(),
            count: 0,
        }
    }

    fn from(region: String, code: String, date: String) -> Result<Self, Box<dyn Error>> {
        Ok(SpotifyChart {
            region,
            code,
            date: match_date(&date)?.unwrap(),
            chart: Vec::new(),
            count: 0,
        })
    }

    pub fn from_reader(f: File) -> Result<Self, Box<dyn std::error::Error>> {
        let mut res = Self::new();
        let mut csv_rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .delimiter(b'*')
            .from_reader(f);

        for rec in csv_rdr.deserialize() {
            let (rank, title, artist, streams): Record = rec?;

            res.chart
                .push(SpotifyEntry::new(rank, title, artist, parse_int(&streams)?));
        }

        res.count = res.chart.len() as u8;
        Ok(res)
    }

    pub fn find_by_title(&self, title: &str) -> Option<&SpotifyEntry> {
        let res: Vec<&SpotifyEntry> = self
            .chart
            .iter()
            .filter(|&entry| entry.title.to_lowercase().contains(&title.to_lowercase()))
            .collect();
        if let Some(&res) = res.get(0) {
            Some(res)
        } else {
            None
        }
    }

    pub fn find_all_by_title(&self, title: &str) -> Option<Vec<&SpotifyEntry>> {
        let res: Vec<&SpotifyEntry> = self
            .chart
            .iter()
            .filter(|&entry| entry.title.to_lowercase().contains(&title.to_lowercase()))
            .collect();
        if res.len() > 0 {
            Some(res)
        } else {
            None
        }
    }
}

fn match_path(path_name: &str) -> Result<(String, String), Box<dyn Error>> {
    let re = Regex::new(r"/\w*/\w*/\w*/\w*/\w*/(\w*)/(\w*-\w*-\w*)").unwrap();

    match re.is_match(path_name) {
        true => match re.captures(path_name) {
            Some(caps) => Ok((
                String::from(caps.get(1).unwrap().as_str()),
                String::from(caps.get(2).unwrap().as_str()),
            )),
            None => Err(From::from("Something is wrong with regex!")),
        },
        false => Err(From::from(
            "Can not match the code, please enter a valid path!",
        )),
    }
}

fn match_code(path_name: &str) -> Result<String, Box<dyn Error>> {
    let re = Regex::new(r"/\w*/\w*/\w*/\w*/\w*/(\w*)/\w*").unwrap();

    match re.is_match(path_name) {
        true => match re.captures(path_name) {
            Some(res) => Ok(String::from(&res[1])),
            None => Err(From::from("Something is wrong with regex!")),
        },
        false => Err(From::from(
            "Can not match the code, please enter a valid path!",
        )),
    }
}

fn match_date(date: &str) -> Result<Option<Date>, Box<dyn Error>> {
    let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
    let caps = re.captures(&date).unwrap();

    let year = caps[1].parse::<i32>()?;
    let month: u8 = caps[2].parse::<u8>()?;
    let day = caps[3].parse::<u8>()?;

    let date = Date::from_calendar_date(year, Month::try_from(month)?, day);

    Ok(Some(date?))
}

type Record = (i16, String, String, String);

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

pub fn parse_int(num: &str) -> Result<i64, Box<dyn Error>> {
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

    Ok(res as i64)
}

// Verify code and date is valid, check whther a file with this code and date exists
pub fn resolve_file_handle(code: &str, date: &str) -> Result<File, Box<dyn std::error::Error>> {
    match (verify_code(code), verify_date(date)) {
        // Only attempt to open the file when both code and date are valid
        (true, true) => {
            let path = format!(
                "/home/ubuntu/project/chartscan/SpotifyData/{}/{}.csv",
                code, date
            );
            match File::open(path.clone()) {
                Ok(f) => Ok(f),
                Err(e) => Err(From::from(format!("{}: {}", e, path))),
            }
        }
        // Code is valid but date is invalid
        (true, false) => Err(From::from(format!("Invalid date: \"{}\"", date))),
        (false, true) => Err(From::from(format!("Invalid code: \"{}\"", code))),
        (false, false) => Err(From::from(format!(
            "Invalid code and date: {} -- {}",
            code, date
        ))),
    }
}

fn verify_code(code: &str) -> bool {
    // Anything that isn't NOTVALID will be true, else false
    match Regions::from(code) {
        Regions::NOTVALID => false,
        _ => true,
    }
}

fn verify_date(date: &str) -> bool {
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    re.is_match(date)
}

#[cfg(test)]
mod moretest {
    use super::*;

    type MyResult<T> = Result<T, Box<dyn Error>>;

    #[test]
    fn verify_date_20001_01_27_invalid() -> MyResult<()> {
        assert!(!verify_date("20001-01-27"));
        Ok(())
    }

    #[test]
    fn verify_date_2001_01_270_invalid() -> MyResult<()> {
        assert!(!verify_date("2001-01-270"));
        Ok(())
    }

    #[test]
    fn verify_date_2001_1_27_invalid() -> MyResult<()> {
        assert!(!verify_date("2001-1-27"));
        Ok(())
    }

    #[test]
    fn verify_date_2001_01_27_valid() -> MyResult<()> {
        assert!(verify_date("2001-01-27"));
        Ok(())
    }

    #[test]
    fn verify_date_2022_05_27_valid() -> MyResult<()> {
        assert!(verify_date("2022-05-27"));
        Ok(())
    }

    #[test]
    fn verify_code_us() -> MyResult<()> {
        assert!(verify_code("us")); // Should retuen True
        Ok(())
    }

    #[test]
    fn verify_code_global() -> MyResult<()> {
        assert!(verify_code("global"));
        Ok(())
    }

    #[test]
    fn verify_code_invalid_oo() -> MyResult<()> {
        assert!(!verify_code("oo")); // oo is not a valid region code
        Ok(())
    }
}
