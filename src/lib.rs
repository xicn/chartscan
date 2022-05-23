use clap::{Parser, Subcommand};
use num_format::{Locale, ToFormattedString};

use crate::spotify::parse_int;

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
    }

    Ok(())
}
