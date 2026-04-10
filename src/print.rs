use colored::Colorize;

use crate::{cache::Cache, dto::UserStats};

const BANNER_SEPARATOR: &str = "·";

pub fn print_user_data(cache: Cache) {
    banner(&cache.username, cache.user_stats);
}

fn banner(username: &str, stats: UserStats) {
    let total_secs = stats.time_typing as u32;
    let hours = total_secs / 3600;
    let minutes = (total_secs % 3600) / 60;
    let seconds = total_secs % 60;

    let text = format!(
        "{} {BANNER_SEPARATOR} {} tests {BANNER_SEPARATOR} {hours:02}:{minutes:02}:{seconds:02} typing",
        username.bright_cyan(),
        stats.completed_tests,
    );
    let text_len = text.chars().count();

    print!("{}", "╔".yellow());
    for _ in 2..text_len {
        print!("{}", "═".yellow());
    }
    println!("{}", "╗".yellow());

    // unaimeds: hardcoded values to get understading of how it can be done
    // i'll add dynamic padding calculation in the next update
    println!(
        "{0} {text:>pad$} {0:>pad2$}",
        "║".yellow(),
        pad = text_len + 2,
        pad2 = 4,
    );

    print!("{}", "╚".yellow());
    for _ in 2..text_len {
        print!("{}", "═".yellow());
    }
    println!("{}", "╝".yellow());
}
