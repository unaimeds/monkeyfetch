use std::collections::HashMap;

use chrono_humanize::HumanTime;
use colored::Colorize;

use crate::{
    cache::Cache,
    dto::{PersonalBest, TestResult, UserStats},
};

// TODO: dynamic output size based on terminal's size

const BANNER_SEPARATOR: &str = "·";
const PB_COLUMNS: &[&str] = &["mode", "wpm", "raw", "acc", "cons"];

// Monkeytype palette
const SUB: (u8, u8, u8) = (100, 102, 105); // #646669
const ACCENT: (u8, u8, u8) = (226, 183, 20); // #e2b714

pub fn print_user_data(cache: Cache) {
    let (b_text, b_len) = banner(&cache.username, cache.user_stats);
    let (pb_lines, pb_len) = personal_bests(cache.personal_bests);
    let (rt_lines, rt_len) = recent_tests(&cache.recent_tests);

    let max_width = [b_len, pb_len, rt_len].into_iter().max().unwrap();
    // must be divisible by PB_COLUMNS.len() (for column borders) and by 2 (for banner centering)
    let col_count = PB_COLUMNS.len();
    let round_to = if col_count.is_multiple_of(2) {
        col_count
    } else {
        col_count * 2
    };
    let max_width = max_width.div_ceil(round_to);

    print_banner(&b_text, max_width);
    println!();
    print_personal_bests(&pb_lines, max_width);
    println!();
    print_recent_tests(&rt_lines, max_width);
}

fn banner(username: &str, stats: UserStats) -> (String, usize) {
    let total_secs = stats.time_typing as u32;
    let hours = total_secs / 3600;
    let minutes = (total_secs % 3600) / 60;
    let seconds = total_secs % 60;

    let text = format!(
        "{username} {BANNER_SEPARATOR} {} tests {BANNER_SEPARATOR} {hours:02}:{minutes:02}:{seconds:02} typing",
        stats.completed_tests,
    );

    let width = text.chars().count();
    (text, width)
}

fn print_banner(inner: &str, width: usize) {
    let (sr, sg, sb) = SUB;
    let (ar, ag, ab) = ACCENT;

    let inner_width = width - 4; // padding of 2 whitespaces on each side
    let padding = inner_width - inner.chars().count();
    let left_pad = padding / 2;
    let right_pad = padding - left_pad;

    println!(
        "{}",
        format!("╔{}╗", "═".repeat(width)).truecolor(sr, sg, sb)
    );
    println!(
        "{}  {:left_pad$}{}{:right_pad$}  {}",
        "║".truecolor(sr, sg, sb),
        "",
        inner.truecolor(ar, ag, ab).bold(),
        "",
        "║".truecolor(sr, sg, sb),
    );
    println!(
        "{}",
        format!("╚{}╝", "═".repeat(width)).truecolor(sr, sg, sb)
    );
}

fn personal_bests(bests: HashMap<String, PersonalBest>) -> (Vec<Vec<String>>, usize) {
    let mut lines = Vec::new();

    let header_line = format!(" {} ", PB_COLUMNS.join(" │ "));
    let mut max_width = header_line.chars().count();

    let mut entries: Vec<(String, PersonalBest)> = bests.into_iter().collect();
    entries.sort_by_key(|(k, _)| k.parse::<u32>().unwrap_or(u32::MAX));

    for (mode, best) in entries {
        let columns = vec![
            format!("{mode}s"),
            format!("{}", best.wpm.round()),
            format!("{}", best.raw.round()),
            format!("{:.2}%", best.accuracy),
            format!("{:.2}%", best.consistency),
        ];

        let min_line = format!(" {} ", columns.join(" │ "));
        let width = min_line.chars().count();
        if width > max_width {
            max_width = width;
        }

        lines.push(columns);
    }

    (lines, max_width)
}

fn make_border(
    width: usize,
    header_width: usize,
    start: &str,
    end: &str,
    normal: &str,
    on_cond: &str,
) -> String {
    let mut s = String::from(start);
    let mut count = 1;
    for _ in 1..width {
        if count == header_width {
            s.push_str(on_cond);
            count = 1;
        } else {
            s.push_str(normal);
            count += 1;
        }
    }
    s.push_str(end);
    s
}

fn print_personal_bests(lines: &[Vec<String>], width: usize) {
    let (sr, sg, sb) = SUB;
    let (ar, ag, ab) = ACCENT;

    println!("{}", "personal bests".truecolor(ar, ag, ab).bold());

    let header_width = width / PB_COLUMNS.len();

    println!(
        "{}",
        make_border(width, header_width, "┌", "─┐", "─", "┬").truecolor(sr, sg, sb)
    );

    print!("{}", "│".truecolor(sr, sg, sb));
    for (i, col) in PB_COLUMNS.iter().enumerate() {
        let with_sep = i < PB_COLUMNS.len() - 1;

        let padding = header_width - col.chars().count();
        let left_pad = padding / 2;
        let right_pad = padding - left_pad - if with_sep { 1 } else { 0 };

        print!(
            "{:left_pad$}{}{:right_pad$}{}",
            "",
            col.truecolor(ar, ag, ab).bold(),
            "",
            if with_sep {
                "│".truecolor(sr, sg, sb).to_string()
            } else {
                String::new()
            }
        );
    }
    println!("{}", "│".truecolor(sr, sg, sb));

    println!(
        "{}",
        make_border(width, header_width, "├", "─┤", "─", "┼").truecolor(sr, sg, sb)
    );

    for row in lines {
        print!("{}", "│".truecolor(sr, sg, sb));
        for (i, cell) in row.iter().enumerate() {
            let with_sep = i < PB_COLUMNS.len() - 1;

            let padding = header_width.saturating_sub(cell.chars().count());
            let left_pad = padding / 2;
            let right_pad = padding - left_pad - if with_sep { 1 } else { 0 };

            let colored_cell = if i == 1 {
                cell.truecolor(ar, ag, ab).to_string()
            } else {
                cell.normal().to_string()
            };

            print!(
                "{:left_pad$}{}{:right_pad$}{}",
                "",
                colored_cell,
                "",
                if with_sep {
                    "│".truecolor(sr, sg, sb).to_string()
                } else {
                    String::new()
                }
            );
        }
        println!("{}", "│".truecolor(sr, sg, sb));
    }

    println!(
        "{}",
        make_border(width, header_width, "└", "─┘", "─", "┴").truecolor(sr, sg, sb)
    );
}

fn recent_tests(tests: &[TestResult]) -> (Vec<(String, String)>, usize) {
    let mut lines = Vec::new();
    let mut max_width = 0;

    for test in tests {
        let left = format!(
            "▸ {} wpm  raw {}  {}%  {}/{}",
            test.wpm.round(),
            test.raw_wpm.round(),
            test.accuracy.round(),
            test.mode,
            test.mode2,
        );
        let when = HumanTime::from(test.timestamp);
        let right = format!("{when}");

        let min_line = format!("{left}    {right}");

        let width = min_line.chars().count();
        if width > max_width {
            max_width = width;
        }

        lines.push((left, right));
    }

    (lines, max_width)
}

fn print_recent_tests(tests: &[(String, String)], width: usize) {
    let (sr, sg, sb) = SUB;
    let (ar, ag, ab) = ACCENT;

    let w = width + 2;

    println!("{}", "recent tests".truecolor(ar, ag, ab).bold());
    println!("{}", "─".repeat(w).truecolor(sr, sg, sb));

    let max_right_width = tests
        .iter()
        .map(|t| &t.1)
        .map(|s| s.chars().count())
        .max()
        .unwrap();

    for (left, right) in tests {
        let right_pad = w - left.chars().count() - max_right_width;
        println!("{}{:right_pad$}{}", left, "", right.truecolor(sr, sg, sb));
    }
}
