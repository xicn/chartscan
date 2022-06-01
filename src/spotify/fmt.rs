use num_format::{Locale, ToFormattedString};

use super::SpotifyGain;

pub fn style1(gain: &SpotifyGain, region: String) {
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

pub fn style2(gain: &SpotifyGain, region: String) {
    let new_entry = "<span style=\"color: #4687d7\">●</span>";
    let increase = "<span style=\"color: #83be28\">▲</span>";
    // let decrease = "<span style=\"color: #3e3e40\">▼</span>";"
    let decrease = "<span style=\"color: #ba3219\">▼</span>";
    let same = "<span style=\"color: #3e3e40\">■</span>";

    if gain.yesterday_rank == 0 {
        let str1 = format!("<tr>\n\t<td class=\"first\">{} [NE] {}</td>\n\t<td class=\"center\">{}</td>\n\t<td>{}</td>\n\t<td>N/A</td>\n\t<td>N/A</td>\n</tr>\n",gain.today_rank, new_entry, region, gain.today_streams.to_formatted_string(&Locale::en));
        return println!("{}", str1);
    }

    let symbol = if gain.rank_diff > 0 {
        increase
    } else if gain.rank_diff == 0 {
        same
    } else {
        decrease
    };

    if gain.percent_diff > 0.03 {
        let str1 = format!("<tr>\n\t<td class=\"first\">{} [{:+}] {}</td>\n\t<td class=\"center\">{}</td>\n\t<td>{}</td>\n\t<td style=\"color:#1abc9c;\">{}</td>\n\t<td style=\"color:#1abc9c;\">{:+.2}%</td>\n</tr>\n",gain.today_rank, gain.rank_diff,symbol,  region, gain.today_streams.to_formatted_string(&Locale::en), format!("{}{}", if gain.streams_diff >= 0 { "+" } else { "" },
        gain.streams_diff.to_formatted_string(&Locale::en)), gain.percent_diff * 100f64);
        println!("{}", str1);
    } else {
        let str1 = format!("<tr>\n\t<td class=\"first\">{} [{:+}] {}</td>\n\t<td class=\"center\">{}</td>\n\t<td>{}</td>\n\t<td>{}</td>\n\t<td>{:+.2}%</td>\n</tr>\n",gain.today_rank, gain.rank_diff,symbol,  region, gain.today_streams.to_formatted_string(&Locale::en), format!("{}{}", if gain.streams_diff >= 0 { "+" } else { "" },
        gain.streams_diff.to_formatted_string(&Locale::en)), gain.percent_diff * 100f64);
        println!("{}", str1);
    }
}
