use clap::{Parser, Subcommand};
use num_format::{Locale, ToFormattedString};
use spotify::resolve_file_handle;

use crate::spotify::{parse_int, SpotifyChart};

mod spotify;

#[derive(Parser)]
#[clap(name = "ChartScan")]
#[clap(author = "xicnx. <okstrategie@gmail.com>")]
#[clap(version = "0.0.1")]
struct Cli {
    // Subcommand should always be &supplied
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Calculate the rank, streams change, and pretty print to console
    Calc {
        /// Region code
        #[clap(long)]
        code: String,

        /// Previous day rank
        #[clap(long = "pr")]
        pr: i16,

        /// Today rank
        #[clap(long = "tr")]
        tr: i16,

        /// Previous day rank
        #[clap(long = "ps")]
        ps: String,

        /// Today rank
        #[clap(long = "ts")]
        ts: String,
    },
    /// Find a song gain
    Find {
        /// Region code
        #[clap(short, long)]
        code: String,

        /// Date
        #[clap(short, long)]
        date: String,

        /// Title keyword
        #[clap(short, long)]
        title: Option<String>,

        /// Title keyword
        #[clap(short, long)]
        artist: Option<String>,

        /// Print all result found
        #[clap(long)]
        all: bool,

        /// One single keyword that can be search in title and artist
        keyword: Option<String>,

        /// Option to decide whther to display gains between two entries
        #[clap(short, long)]
        gains: bool,
    },
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Calc {
            code,
            pr,
            tr,
            ps,
            ts,
        } => {
            let result: i64 = (parse_int(&ts)? - parse_int(&ps)?) as i64;
            println!("{}:", code);
            println!(
                "#{}[{:+}] - {}({}{:+})",
                tr,
                pr - tr,
                ts,
                if result >= 0 { "+" } else { "" },
                result.to_formatted_string(&Locale::en)
            );
        }
        Commands::Find {
            code,
            date,
            title,
            artist,
            all,
            keyword,
            gains,
        } => {
            // println!("{} {} {:?} {:?}", code, date, title, artist );

            let fh = resolve_file_handle(&code, &date)?;
            let chart = SpotifyChart::from_reader(fh, &date, &code)?;
            let date_code_str = format!(" date<{}> code<{}>", date, code);
            let format_str = dbg_str(&title, &artist, &keyword);

            match gains {
                true => {
                    let chart = chart.previous_day()?;
                    println!("{:?}", chart);

                    // match all {
                    //     true => todo!(),
                    //     false => match (title, artist) {
                    //         (None, None) => todo!(),
                    //         (None, Some(_artist)) => todo!(),
                    //         (Some(title), None) => {
                    //             let sp_gain =
                    //                 SpotifyGain::song_gain_by_title(chart, previous_chart, &title);
                    //             println!("{:#?}", sp_gain);
                    //         }
                    //         (Some(_title), Some(_artist)) => todo!(),
                    //     },
                    // }
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
                                println!("Find:{}{} - 0 result", date_code_str, format_str);
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
                                (Some(title), Some(artist)) => {
                                    chart.find_by_title_artist(&title, &artist)
                                }
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
