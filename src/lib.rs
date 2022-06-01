#![allow(dead_code)]
use clap::{Parser, Subcommand};
use num_format::{Locale, ToFormattedString};
use spotify::{regions::RegionString, SpotifyChart, SpotifyGain};

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

        /// Previous date
        #[clap(short, long)]
        previous_date: Option<String>,
    },
    /// Spotify chart
    Daily {
        /// Date
        #[clap(short, long)]
        date: String,

        /// Title keyword
        #[clap(short, long)]
        title: Option<String>,

        /// Title keyword
        #[clap(short, long)]
        artist: Option<String>,
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
            let result: i64 = (spotify::parse_int(&ts)? - spotify::parse_int(&ps)?) as i64;
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
            previous_date,
        } => spotify::find::find(
            code,
            date,
            title,
            artist,
            all,
            keyword,
            gains,
            previous_date,
        )?,
        Commands::Daily {
            date,
            title,
            artist,
        } => {
            let mut gains: Vec<(SpotifyGain, String)> = Vec::new();
            for code in spotify::regions::Regions::regions_vec() {
                let region = code.to_region_string();
                let code = String::from(code);

                if let Ok(f) = spotify::resolve_file_handle(code.as_str(), &date) {
                    let chart = SpotifyChart::from_reader(f, &date, &code)?;
                    if let Ok(previous_chart) = chart.previous_day() {
                        let gain = match (title.clone(), artist.clone()) {
                            (None, None) => {
                                return Err(From::from(
                                    "Either one of title or artist need to be specified!",
                                ))
                            }
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

                        if gain.today_rank != 0 {
                            gains.push((gain, region));
                        } else {
                            eprintln!("{:#?}", gain);
                        }
                    } else {
                        eprintln!("{} - Previous day[-] data missing!", code);
                    }
                } else {
                    eprintln!("{} - Today[{}] data missing!", code, date);
                }
            }

            gains.sort_by_key(|(gain, _)| gain.today_streams);
            gains.reverse();
            gains
                .into_iter()
                .for_each(|(gain, region)| gain.print(region, spotify::fmt::style2));
        }
    }

    Ok(())
}
