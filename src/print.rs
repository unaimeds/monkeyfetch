use std::collections::HashMap;

use chrono_humanize::HumanTime;

use crate::{
    cache::Cache,
    dto::{PersonalBest, TestResult, UserStats},
};

// TODO: dynamic output size based on terminal's size
// TODO: colored output

const BANNER_SEPARATOR: &str = "·";
const PB_COLUMNS: &[&str] = &["mode", "wpm", "raw", "acc", "cons"];

pub fn print_user_data(cache: Cache) {
    let (b_text, b_len) = banner(&cache.username, cache.user_stats);
    let (pb_lines, pb_len) = personal_bests(cache.personal_bests);
    let (rt_lines, rt_len) = recent_tests(&cache.recent_tests);

    let max_width = [b_len, pb_len, rt_len].into_iter().max().unwrap();
    // must be divisible by PB_COLUMNS.len() (for column borders) and by 2 (for banner centering)
    let col_count = PB_COLUMNS.len();
    let round_to = if col_count % 2 == 0 {
        col_count
    } else {
        col_count * 2
    };
    let max_width = ((max_width + round_to - 1) / round_to) * round_to;

    print_banner(&b_text, max_width);
    println!("");
    print_personal_bests(&pb_lines, max_width);
    println!("");
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
    let inner_width = width - 4; // padding of 2 whitespaces on each side
    let padding = inner_width - inner.chars().count();

    let left_pad = padding / 2;
    let right_pad = padding - left_pad;

    println!("╔{}╗", "═".repeat(width));
    println!("║  {:left_pad$}{inner}{:right_pad$}  ║", "", "");
    println!("╚{}╝", "═".repeat(width));
}

fn personal_bests(bests: HashMap<String, PersonalBest>) -> (Vec<Vec<String>>, usize) {
    let mut lines = Vec::new();

    let header_line = format!(" {} ", PB_COLUMNS.join(" │ "));
    let mut max_width = header_line.chars().count();

    // TODO: order by mode (15s -> 120s)
    for (mode, best) in bests {
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

fn print_cond_border(
    width: usize,
    header_width: usize,
    start: &str,
    end: &str,
    normal: &str,
    on_cond: &str,
) {
    print!("{start}");
    let mut count = 1;
    for _ in 1..width {
        if count == header_width {
            print!("{on_cond}");
            count = 1;
        } else {
            print!("{normal}");
            count += 1;
        }
    }
    println!("{end}");
}

fn print_personal_bests(lines: &[Vec<String>], width: usize) {
    println!("personal bests");

    // calculate how many chars we have for each header
    let header_width = width / PB_COLUMNS.len();

    // top border
    // we print additional horizontal border because last header
    // has one more padding on right due to not having vertical border
    print_cond_border(width, header_width, "┌", "─┐", "─", "┬");

    // headers
    print!("│");
    for (i, col) in PB_COLUMNS.iter().enumerate() {
        let with_sep = i < PB_COLUMNS.len() - 1;

        let padding = header_width - col.chars().count();
        let left_pad = padding / 2;
        // -1 because we add a separator at the end of header
        let right_pad = padding - left_pad - if with_sep { 1 } else { 0 };

        print!(
            "{:left_pad$}{col}{:right_pad$}{}",
            "",
            "",
            if with_sep { "│" } else { "" }
        );
    }
    println!("│");

    // bottom header border
    // same reason for additional horizontal border at end as in top border
    print_cond_border(width, header_width, "├", "─┤", "─", "┼");

    // data rows
    for row in lines {
        print!("│");
        for (i, cell) in row.iter().enumerate() {
            let with_sep = i < PB_COLUMNS.len() - 1;

            let padding = header_width.saturating_sub(cell.chars().count());
            let left_pad = padding / 2;
            let right_pad = padding - left_pad - if with_sep { 1 } else { 0 };

            print!(
                "{:left_pad$}{cell}{:right_pad$}{}",
                "",
                "",
                if with_sep { "│" } else { "" }
            );
        }
        println!("│");
    }

    // bottom border
    print_cond_border(width, header_width, "└", "─┘", "─", "┴");
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
    let w = width + 2;

    println!("recent tests");
    println!("{}", "─".repeat(w));

    let max_right_width = tests
        .iter()
        .map(|t| &t.1)
        .map(|s| s.chars().count())
        .max()
        .unwrap();

    for (left, right) in tests {
        let right_pad = w - left.chars().count() - max_right_width;
        println!("{left}{:right_pad$}{right}", "");
    }
}
