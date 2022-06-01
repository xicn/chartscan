use num_format::{Locale, ToFormattedString};

use super::SpotifyGain;

pub fn style1(gain: &SpotifyGain, region: String) {
    println!(
        "{:11} {:<21} {:3} {:3} [{:+4}] {:>10} {:>10} {:>10} {:>+5.2}%",
        region,
        gain.title,
        gain.yesterday_rank,
        gain.today_rank,
        gain.rank_diff,
        add_comma(gain.today_streams),
        add_comma(gain.yesterday_streams),
        add_comma_plus(gain.streams_diff),
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
        let str1 = get_html_str(
            gain,
            symbol,
            &region,
            Some(" style=\"color:#1abc9c;\"".to_string()),
        );
        println!("{}", str1);
    } else {
        let str1 = get_html_str(gain, symbol, &region, None);

        println!("{}", str1);
    }
}

fn get_html_str(gain: &SpotifyGain, symbol: &str, region: &str, style: Option<String>) -> String {
    let style = style.unwrap_or("".to_string());
    format!(
        "\
<tr>
    <td class=\"first\">{} [{:+}] {}</td>
    <td class=\"center\">{}</td>
    <td>{}</td>
    <td{}>{}</td>
    <td{}>{:+.2}%</td>
</tr>",
        gain.today_rank,
        gain.rank_diff,
        symbol,
        region,
        add_comma(gain.today_streams),
        style,
        add_comma_plus(gain.streams_diff),
        style,
        gain.percent_diff * 100f64
    )
}

fn add_comma(num: i64) -> String {
    format!("{}", num.to_formatted_string(&Locale::en))
}

fn add_comma_plus(num: i64) -> String {
    format!(
        "{}{}",
        if num >= 0 { "+" } else { "" },
        num.to_formatted_string(&Locale::en)
    )
}
