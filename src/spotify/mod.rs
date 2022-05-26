#![allow(dead_code)]

mod regions;

use regex::Regex;
use std::{error::Error, fs::File, num::NonZeroU8};
use time::{Date, Month};

use self::regions::Regions;

#[derive(Debug, PartialEq)]
pub struct SpotifyEntry {
    pub rank: i16,
    pub title: String,
    pub artist: String,
    pub streams: i64,
}

impl SpotifyEntry {
    fn new(rank: i16, title: &str, artist: &str, streams: i64) -> Self {
        SpotifyEntry {
            rank,
            title: title.to_string(),
            artist: artist.to_string(),
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

    fn spotify_chart_build(
        region: String,
        code: String,
        date: String,
    ) -> Result<Self, Box<dyn Error>> {
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

            res.chart.push(SpotifyEntry::new(
                rank,
                &title,
                &artist,
                parse_int(&streams)?,
            ));
        }

        res.count = res.chart.len() as u8;
        Ok(res)
    }

    fn find(
        &self,
        title: Option<&str>,
        artist: Option<&str>,
        keyword: Option<&str>,
    ) -> Option<&SpotifyEntry> {
        let entry = self.find_all(title, artist, keyword);
        if let None = entry {
            None
        } else if let Some(&result) = entry.unwrap().get(0) {
            Some(result)
        } else {
            None
        }
    }

    pub fn find_by_title_artist(&self, title: &str, artist: &str) -> Option<&SpotifyEntry> {
        self.find(Some(title), Some(artist), None)
    }

    pub fn find_by_keyword(&self, keyword: &str) -> Option<&SpotifyEntry> {
        self.find(None, None, Some(keyword))
    }

    pub fn find_by_artist(&self, artist: &str) -> Option<&SpotifyEntry> {
        self.find(None, Some(artist), None)
    }

    pub fn find_by_title(&self, title: &str) -> Option<&SpotifyEntry> {
        self.find(Some(title), None, None)
    }

    fn find_all(
        &self,
        title: Option<&str>,
        artist: Option<&str>,
        keyword: Option<&str>,
    ) -> Option<Vec<&SpotifyEntry>> {
        let entry: Option<Vec<&SpotifyEntry>> = match (title, artist, keyword) {
            (None, None, None) => None,
            (None, None, Some(keyword)) => Some(
                self.chart
                    .iter()
                    .filter(|&entry| {
                        (entry.title.to_lowercase().contains(&keyword.to_lowercase()))
                            || (entry
                                .artist
                                .to_lowercase()
                                .contains(&keyword.to_lowercase()))
                    })
                    .collect::<Vec<&SpotifyEntry>>(),
            ),
            (None, Some(artist), None) => Some(
                self.chart
                    .iter()
                    .filter(|&entry| entry.artist.to_lowercase().contains(&artist.to_lowercase()))
                    .collect::<Vec<&SpotifyEntry>>(),
            ),
            (None, Some(artist), Some(keyword)) => Some(
                self.chart
                    .iter()
                    .filter(|&entry| {
                        (entry.title.to_lowercase().contains(&keyword.to_lowercase()))
                            || (entry
                                .artist
                                .to_lowercase()
                                .contains(&keyword.to_lowercase()))
                                && (entry.artist.to_lowercase().contains(&artist.to_lowercase()))
                    })
                    .collect::<Vec<&SpotifyEntry>>(),
            ),
            (Some(title), None, None) => Some(
                self.chart
                    .iter()
                    .filter(|&entry| entry.title.to_lowercase().contains(&title.to_lowercase()))
                    .collect::<Vec<&SpotifyEntry>>(),
            ),
            (Some(title), None, Some(keyword)) => Some(
                self.chart
                    .iter()
                    .filter(|&entry| {
                        (entry.title.to_lowercase().contains(&keyword.to_lowercase()))
                            || (entry
                                .artist
                                .to_lowercase()
                                .contains(&keyword.to_lowercase()))
                                && (entry.title.to_lowercase().contains(&title.to_lowercase()))
                    })
                    .collect::<Vec<&SpotifyEntry>>(),
            ),
            (Some(title), Some(artist), None) => Some(
                self.chart
                    .iter()
                    .filter(|&entry| {
                        (entry.title.to_lowercase().contains(&title.to_lowercase()))
                            && (entry.artist.to_lowercase().contains(&artist.to_lowercase()))
                    })
                    .collect::<Vec<&SpotifyEntry>>(),
            ),
            (Some(title), Some(artist), Some(keyword)) => Some(
                self.chart
                    .iter()
                    .filter(|&entry| {
                        (entry.title.to_lowercase().contains(&keyword.to_lowercase()))
                            || (entry
                                .artist
                                .to_lowercase()
                                .contains(&keyword.to_lowercase()))
                                && ((entry.title.to_lowercase().contains(&title.to_lowercase()))
                                    && (entry
                                        .artist
                                        .to_lowercase()
                                        .contains(&artist.to_lowercase())))
                    })
                    .collect::<Vec<&SpotifyEntry>>(),
            ),
        };

        if let Some(entry) = entry {
            if entry.len() > 0 {
                Some(entry)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn find_all_by_title_artist(
        &self,
        title: &str,
        artist: &str,
    ) -> Option<Vec<&SpotifyEntry>> {
        self.find_all(Some(title), Some(artist), None)
    }

    pub fn find_all_by_keyword(&self, keyword: &str) -> Option<Vec<&SpotifyEntry>> {
        self.find_all(None, None, Some(keyword))
    }

    pub fn find_all_by_artist(&self, artist: &str) -> Option<Vec<&SpotifyEntry>> {
        self.find_all(None, Some(artist), None)
    }

    pub fn find_all_by_title(&self, title: &str) -> Option<Vec<&SpotifyEntry>> {
        self.find_all(Some(title), None, None)
    }
}

#[derive(Debug, PartialEq)]
pub struct SpotifyGain {
    today_rank: i16,
    yesterday_rank: i16,
    rank_diff: i16,
    title: String,
    artist: String,
    today_streams: i64,
    yesterday_streams: i64,
    streams_diff: i64,
}

impl SpotifyGain {
    pub fn new(
        today_rank: i16,
        yesterday_rank: i16,
        title: &str,
        artist: &str,
        today_streams: i64,
        yesterday_streams: i64,
    ) -> Self {
        SpotifyGain {
            today_rank,
            yesterday_rank,
            rank_diff: yesterday_rank - today_rank,
            title: title.to_string(),
            artist: artist.to_string(),
            today_streams,
            yesterday_streams,
            streams_diff: today_streams - yesterday_streams,
        }
    }

    pub fn from_spotify_entry(
        today: &SpotifyEntry,
        yesterday: &SpotifyEntry,
    ) -> Result<SpotifyGain, Box<dyn Error>> {
        if today.title == yesterday.title && today.artist == yesterday.artist {
            Ok(SpotifyGain::new(
                today.rank,
                yesterday.rank,
                &today.title,
                &today.artist,
                today.streams,
                yesterday.streams,
            ))
        } else {
            Err(From::from(
                "Two SpotifyEntries do not have same title and artist.",
            ))
        }
    }

    pub fn song_gain_by_title(
        chart: SpotifyChart,
        previous_chart: SpotifyChart,
        title: &str,
    ) -> SpotifyGain {
        let today = chart.find_by_title(title);
        let yesterday = previous_chart.find_by_title(title);
        let sp_gain = match (today, yesterday) {
            (None, None) => SpotifyGain::new(0, 0, title, "Unknown", 0, 0),
            (None, Some(entry)) => {
                SpotifyGain::new(0, entry.rank, &entry.title, &entry.artist, 0, entry.streams)
            }
            (Some(entry), None) => {
                SpotifyGain::new(entry.rank, 0, &entry.title, &entry.artist, entry.streams, 0)
            }
            (Some(today), Some(yesterday)) => SpotifyGain::new(
                today.rank,
                yesterday.rank,
                &today.title,
                &today.artist,
                today.streams,
                yesterday.streams,
            ),
        };
        sp_gain
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

        res.push(SpotifyEntry::new(
            rank,
            &title,
            &artist,
            parse_int(&streams)?,
        ));
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

pub fn previous_day(date: &str) -> Result<String, Box<dyn Error>> {
    if let Some(date) = match_date(date).unwrap() {
        let previous = date.previous_day().unwrap();
        Ok(format!(
            "{}-{:02}-{:02}",
            previous.year(),
            previous.month() as u8,
            previous.day(),
        ))
    } else {
        Err(From::from(format!("Invalid date string: {}", date)))
    }
}
#[cfg(test)]
mod moretest {
    use super::*;

    type MyResult<T> = Result<T, Box<dyn Error>>;

    #[test]
    fn from_spotify_entry_valid_1() -> MyResult<()> {
        let title = "As It Was";
        let artist = "Harry Styles";
        let expected = SpotifyGain::new(
            1,
            1,
            title,
            artist,
            parse_int("2,432,888")?,
            parse_int("2,579,111")?,
        );

        let en1 = SpotifyEntry::new(1, title, artist, 2432888);
        let en2 = SpotifyEntry::new(1, title, artist, 2579111);

        assert_eq!(expected, SpotifyGain::from_spotify_entry(&en1, &en2)?);
        Ok(())
    }

    #[test]
    fn from_spotify_entry_valid_2() -> MyResult<()> {
        let title = "As It Was";
        let artist = "Harry Styles";
        let expected = SpotifyGain::new(
            5,
            10,
            title,
            artist,
            parse_int("2,432")?,
            parse_int("2,579")?,
        );

        let en1 = SpotifyEntry::new(5, title, artist, 2432);
        let en2 = SpotifyEntry::new(10, title, artist, 2579);

        assert_eq!(expected, SpotifyGain::from_spotify_entry(&en1, &en2)?);
        Ok(())
    }

    #[test]
    fn from_spotify_entry_invalid_1() -> MyResult<()> {
        Ok(())
    }

    #[test]
    fn from_spotify_entry_invalid_2() -> MyResult<()> {
        Ok(())
    }

    #[test]
    fn pre_day_20220520() -> MyResult<()> {
        assert_eq!("2022-05-19".to_string(), previous_day("2022-05-20")?);
        Ok(())
    }

    #[test]
    fn pre_day_20220521() -> MyResult<()> {
        assert_eq!("2022-05-20".to_string(), previous_day("2022-05-21")?);
        Ok(())
    }

    #[test]
    fn pre_day_20010127() -> MyResult<()> {
        assert_eq!("2001-01-26".to_string(), previous_day("2001-01-27")?);
        Ok(())
    }

    #[test]
    fn pre_day_20010107() -> MyResult<()> {
        assert_eq!("2001-01-06".to_string(), previous_day("2001-01-07")?);
        Ok(())
    }

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
