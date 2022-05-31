use clap::{Parser, Subcommand};
use num_format::{Locale, ToFormattedString};
use spotify::{SpotifyChart, SpotifyGain};

use crate::spotify::{parse_int, resolve_file_handle};
use spotify::regions::RegionString;

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
    Daily {},
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
        Commands::Daily {} => {
            let date = "2022-05-30";
            let mut gains: Vec<(SpotifyGain, String)> = Vec::new();
            for code in spotify::regions::Regions::regions_vec() {
                let region = code.to_region_string();
                let code = String::from(code);
                if let Ok(f) = resolve_file_handle(code.as_str(), date) {
                    let chart = SpotifyChart::from_reader(f, &date, &code)?;
                    let previous_chart = chart.previous_day()?;
                    let res = chart.song_gain(&previous_chart, Some("Potion"), None, None);
                    if res.today_rank != 0 {
                        gains.push((res, region));
                    }
                } else {
                    println!("{} - Today[{}] data missing!", code, date);
                }
            }

            gains
                .into_iter()
                .for_each(|(gain, region)| gain.print(region, fn1));
        }
    }

    Ok(())
}

fn fn1(gain: &SpotifyGain, region: String) {
    println!(
        "{:11} {:<21} {:3} {:3} [{:+4}] {:>10} {:>10} {:>10} {:>+5.2}%",
        region,
        &gain.title[0..21],
        gain.yesterday_rank,
        gain.today_rank,
        gain.rank_diff,
        gain.today_streams.to_formatted_string(&Locale::en),
        gain.yesterday_streams.to_formatted_string(&Locale::en),
        format!(
            "[{}{}]",
            if gain.streams_diff >= 0 { "+" } else { "" },
            gain.streams_diff.to_formatted_string(&Locale::en)
        ),
        gain.percent_diff * 100f64
    );
}
