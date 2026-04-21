use chrono_humanize::HumanTime;
use colored::Colorize;

use crate::{
    cache::Cache,
    print::{ACCENT, SUB},
};

pub fn print_minimal(cache: Cache) {
    let (sr, sg, sb) = SUB;
    let (ar, ag, ab) = ACCENT;

    let total_secs = cache.user_stats.time_typing as u32;
    let hours = total_secs / 3600;
    let minutes = (total_secs % 3600) / 60;
    let seconds = total_secs % 60;

    println!(
        "{}  {} tests  {}:{:02}:{:02}",
        cache.username.truecolor(ar, ag, ab).bold(),
        cache.user_stats.completed_tests,
        hours,
        minutes,
        seconds,
    );
    println!();

    println!("{}", "pb".truecolor(ar, ag, ab).bold());

    let mut entries = cache.personal_bests.iter().collect::<Vec<_>>();
    entries.sort_by_key(|(k, _)| k.parse::<u32>().unwrap_or(u32::MAX));

    let mode_width = entries.iter().map(|(k, _)| k.len() + 1).max().unwrap_or(4);

    for (mode, best) in &entries {
        println!(
            "  {:<mode_width$}  {} wpm  {} raw  {:.1}%  {:.1}%",
            format!("{mode}s").truecolor(sr, sg, sb),
            best.wpm.round().to_string().truecolor(ar, ag, ab),
            best.raw.round(),
            best.accuracy,
            best.consistency,
        );
    }

    println!();
    println!("{}", "recent".truecolor(ar, ag, ab).bold());

    let rows: Vec<(String, String)> = cache
        .recent_tests
        .iter()
        .map(|t| {
            let left = format!(
                "  {} wpm  {} raw  {}%  {}/{}",
                t.wpm.round(),
                t.raw_wpm.round(),
                t.accuracy.round(),
                t.mode,
                t.mode2,
            );
            let when = format!("{}", HumanTime::from(t.timestamp));
            (left, when)
        })
        .collect();

    let max_left = rows
        .iter()
        .map(|(l, _)| l.chars().count())
        .max()
        .unwrap_or(0);

    for (left, when) in &rows {
        let pad = max_left - left.chars().count() + 4;
        println!("{}{:pad$}{}", left, "", when.truecolor(sr, sg, sb),);
    }
}
