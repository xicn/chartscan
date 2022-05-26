use std::error::Error;

use regex::Regex;
use time::{Date, Month};

use super::regions::Regions;

pub fn match_path(path_name: &str) -> Result<(String, String), Box<dyn Error>> {
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

pub fn match_code(path_name: &str) -> Result<String, Box<dyn Error>> {
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

pub fn match_date(date: &str) -> Result<Option<Date>, Box<dyn Error>> {
    let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
    let caps = re.captures(&date).unwrap();

    let year = caps[1].parse::<i32>()?;
    let month: u8 = caps[2].parse::<u8>()?;
    let day = caps[3].parse::<u8>()?;

    let date = Date::from_calendar_date(year, Month::try_from(month)?, day);

    Ok(Some(date?))
}

pub fn verify_code(code: &str) -> bool {
    // Anything that isn't NOTVALID will be true, else false
    match Regions::from(code) {
        Regions::NOTVALID => false,
        _ => true,
    }
}

pub fn verify_date(date: &str) -> bool {
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    re.is_match(date)
}
