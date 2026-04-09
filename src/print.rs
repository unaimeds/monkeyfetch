use colored::Colorize;

use crate::cache::Cache;

pub fn print_user_data(cache: Cache) {
    let total_secs = cache.user_stats.time_typing as u32;
    let hours = total_secs / 3600;
    let minutes = (total_secs % 3600) / 60;
    let seconds = total_secs % 60;

    println!(
        "{}{}{}",
        cache.username.blue().bold(),
        "@".white().bold(),
        "monkeytype".blue().bold(),
    );
    separator();
    println!(
        "{} {}",
        "Tests completed:".blue().bold(),
        cache.user_stats.completed_tests.to_string().white().bold(),
    );
    println!(
        "{} {}",
        "Typed for:".blue().bold(),
        format!("{hours}h, {minutes}m, {seconds}s").white().bold(),
    );
    separator();
}

fn separator() {
    println!("{}", "---------".white().bold());
}
