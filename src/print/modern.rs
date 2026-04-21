use chrono_humanize::HumanTime;
use colored::Colorize;

use crate::{
    cache::Cache,
    print::{ACCENT, SUB},
};

const BAR_MAX: usize = 40;

struct RecentRow {
    bar_len: usize,
    stats: String,
    when: String,
}

pub fn print_modern(cache: Cache) {
    let (sr, sg, sb) = SUB;
    let (ar, ag, ab) = ACCENT;

    let total_secs = cache.user_stats.time_typing as u32;
    let hours = total_secs / 3600;
    let minutes = (total_secs % 3600) / 60;

    println!(
        "{}  {} {} {} {}",
        cache.username.truecolor(ar, ag, ab).bold(),
        cache
            .user_stats
            .completed_tests
            .to_string()
            .truecolor(ar, ag, ab),
        "tests".truecolor(sr, sg, sb),
        "·".truecolor(sr, sg, sb),
        format!("{hours}h {minutes:02}m").truecolor(sr, sg, sb),
    );
    println!();

    // pb section
    let sep_line = "·".to_string() + &"—".repeat(38);
    print!("  ");
    print!("{}", "pb".truecolor(ar, ag, ab).bold());
    println!("  {}", sep_line.truecolor(sr, sg, sb));

    let mut entries = cache.personal_bests.iter().collect::<Vec<_>>();
    entries.sort_by_key(|(k, _)| k.parse::<u32>().unwrap_or(u32::MAX));

    let mode_width = entries.iter().map(|(k, _)| k.len() + 1).max().unwrap_or(4);

    for (mode, best) in &entries {
        println!(
            "  {:<mode_width$}  {} {}  {} {}  {:.1}% {}  {:.1}% {}",
            format!("{mode}s").truecolor(sr, sg, sb),
            best.wpm.round().to_string().truecolor(ar, ag, ab).bold(),
            "wpm".truecolor(sr, sg, sb),
            best.raw.round(),
            "raw".truecolor(sr, sg, sb),
            best.accuracy,
            "acc".truecolor(sr, sg, sb),
            best.consistency,
            "con".truecolor(sr, sg, sb),
        );
    }

    println!();

    // recent section
    print!("  ");
    print!("{}", "recent".truecolor(ar, ag, ab).bold());
    println!("{}", sep_line.truecolor(sr, sg, sb));

    let max_wpm = cache
        .recent_tests
        .iter()
        .map(|t| t.wpm)
        .fold(f32::NEG_INFINITY, f32::max);

    let rows: Vec<RecentRow> = cache
        .recent_tests
        .iter()
        .map(|t| {
            let bar_len = if max_wpm > 0.0 {
                ((t.wpm / max_wpm) * BAR_MAX as f32).round() as usize
            } else {
                0
            };
            let stats = format!("{} wpm  {}/{}", t.wpm.round(), t.mode, t.mode2);
            let when = format!("{}", HumanTime::from(t.timestamp));
            RecentRow {
                bar_len,
                stats,
                when,
            }
        })
        .collect();

    let max_stats = rows
        .iter()
        .map(|r| r.stats.chars().count())
        .max()
        .unwrap_or(0);

    for row in &rows {
        let bar = "█".repeat(row.bar_len);
        let bar_pad = BAR_MAX - row.bar_len;
        let stats_pad = max_stats - row.stats.chars().count();
        let (wpm, rest) = row.stats.split_once(" wpm  ").unwrap();
        println!(
            "  {}{:bar_pad$}  {} {}  {}{:stats_pad$}    {}",
            bar.truecolor(ar, ag, ab),
            "",
            wpm.truecolor(ar, ag, ab).bold(),
            "wpm".truecolor(sr, sg, sb),
            rest.truecolor(sr, sg, sb),
            "",
            row.when.truecolor(sr, sg, sb),
        );
    }
}
