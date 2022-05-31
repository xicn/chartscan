use std::error::Error;

use super::{resolve_file_handle, SpotifyChart};

pub fn find(
    code: String,
    date: String,
    title: Option<String>,
    artist: Option<String>,
    all: bool,
    keyword: Option<String>,
    gains: bool,
    previous_date: Option<String>,
) -> Result<(), Box<dyn Error>> {
    // println!("{} {} {:?} {:?}", code, date, title, artist );

    let fh = resolve_file_handle(&code, &date)?;
    let chart = SpotifyChart::from_reader(fh, &date, &code)?;
    let mut date_code_str = format!(" date<{}> code<{}>", date, code);
    let format_str = dbg_str(&title, &artist, &keyword);

    match gains {
        true => {
            let previous_chart = if let Some(date_str) = previous_date {
                date_code_str = format!(" date<{}> previous<{}> code<{}>", date, date_str, code);
                chart.previous_chart(&date_str)?
            } else {
                chart.previous_day()?
            };
            match all {
                true => {
                    let sp_gain_all = match (title, artist) {
                        (None, None) => match keyword {
                            Some(keyword) => {
                                chart.song_gain_all(&previous_chart, None, None, Some(&keyword))
                            }
                            None => todo!(),
                        },
                        (None, Some(artist)) => {
                            chart.song_gain_all(&previous_chart, None, Some(&artist), None)
                        }
                        (Some(title), None) => {
                            chart.song_gain_all(&previous_chart, Some(&title), None, None)
                        }
                        (Some(title), Some(artist)) => {
                            chart.song_gain_all(&previous_chart, Some(&title), Some(&artist), None)
                        }
                    };
                    if let Some(entry) = sp_gain_all {
                        println!(
                            "Find gain all:{}{} - {} results",
                            date_code_str,
                            format_str,
                            entry.len()
                        );
                        println!("{:#?}", entry);
                    } else {
                        println!("Find gain all:{}{} - 0 result", date_code_str, format_str);
                    }
                }
                false => {
                    let sp_gain = match (title, artist) {
                        (None, None) => match keyword {
                            Some(keyword) => {
                                chart.song_gain(&previous_chart, None, None, Some(&keyword))
                            }
                            None => todo!(),
                        },
                        (None, Some(artist)) => {
                            chart.song_gain(&previous_chart, None, Some(&artist), None)
                        }
                        (Some(title), None) => {
                            chart.song_gain(&previous_chart, Some(&title), None, None)
                        }
                        (Some(title), Some(artist)) => {
                            chart.song_gain(&previous_chart, Some(&title), Some(&artist), None)
                        }
                    };

                    println!("Find gain:{}{} - 1 result", date_code_str, format_str);
                    println!("{:#?}", sp_gain);
                }
            }
        }
        false => {
            match all {
                true => {
                    let entry = match (title, artist) {
                        (None, None) => match keyword {
                            Some(keyword) => chart.find_all_by_keyword(&keyword),
                            None => todo!(),
                        },
                        (None, Some(artist)) => chart.find_all_by_artist(&artist),
                        (Some(title), None) => chart.find_all_by_title(&title),
                        (Some(title), Some(artist)) => {
                            chart.find_all_by_title_artist(&title, &artist)
                        }
                    };

                    if let Some(entry) = entry {
                        println!(
                            "Find all:{}{} - {} results",
                            date_code_str,
                            format_str,
                            entry.len()
                        );
                        println!("{:#?}", entry);
                    } else {
                        println!("Find all:{}{} - 0 result", date_code_str, format_str);
                    }
                }
                false => {
                    let entry = match (title, artist) {
                        (None, None) => match keyword {
                            Some(keyword) => chart.find_by_keyword(&keyword),
                            None => todo!(),
                        },
                        (None, Some(artist)) => chart.find_by_artist(&artist),
                        (Some(title), None) => chart.find_by_title(&title),
                        (Some(title), Some(artist)) => chart.find_by_title_artist(&title, &artist),
                    };
                    if let Some(entry) = entry {
                        println!("Find:{}{} - 1 result", date_code_str, format_str);
                        println!("{:#?}", entry);
                    } else {
                        println!("Find:{}{} - 0 result", date_code_str, format_str);
                    }
                }
            };
        }
    }
    Ok(())
}

fn dbg_str(title: &Option<String>, artist: &Option<String>, keyword: &Option<String>) -> String {
    let title_str = if title.is_none() {
        "".to_string()
    } else {
        format!(" title<\"{}\">", title.clone().unwrap())
    };
    let artist_str = if artist.is_none() {
        "".to_string()
    } else {
        format!(" artist<\"{}\">", artist.clone().unwrap())
    };
    let keyword_str = if keyword.is_none() {
        "".to_string()
    } else {
        format!(" keyword<\"{}\">", keyword.clone().unwrap())
    };

    format!("{}{}{}", title_str, artist_str, keyword_str)
}
