use std::collections::HashMap;

use chrono_humanize::HumanTime;
use colored::Colorize;

use crate::{
    cache::Cache,
    dto::{PersonalBest, TestResult, UserStats},
};

// TODO: dynamic padding based on content length
// TODO: dynamic output size based on terminal's size

const BANNER_SEPARATOR: &str = "·";

pub fn print_user_data(cache: Cache) {
    banner(&cache.username, cache.user_stats);
    println!("");
    personal_bests(cache.personal_bests);
    println!("");
    recent_tests(&cache.recent_tests);
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

fn personal_bests(bests: HashMap<String, PersonalBest>) {
    println!("personal bests");

    // TODO: order by mode (15s -> 120s)
    // TODO: add colored output
    // TODO: always show 2 decimal digits in acc & cons
    println!("┌────────┬──────────┬──────────┬──────────┬───────────┐");
    println!("│  mode  │    wpm   │    raw   │   acc    │    cons   │");
    println!("├────────┼──────────┼──────────┼──────────┼───────────┤");
    for (mode, best) in bests {
        println!(
            "│  {mode}s   │    {}   │    {}   │  {}%  │   {}%  │",
            best.wpm.round(),
            best.raw.round(),
            best.accuracy,
            best.consistency,
        );
    }
    println!("└────────┴──────────┴──────────┴──────────┴───────────┘");
}

// TODO: add colored output
fn recent_tests(tests: &[TestResult]) {
    println!("recent tests");
    println!("────────────────────────────────────────────────────────");
    for test in tests {
        let when = HumanTime::from(test.timestamp);
        println!(
            "▸ {} wpm  raw {}  {}%  {}/{}          {when}",
            test.wpm.round(),
            test.raw_wpm.round(),
            test.accuracy.round(),
            test.mode,
            test.mode2,
        )
    }
}
