mod api;
mod cache;
mod config;
mod dto;
mod error;
mod print;

use chrono::Utc;
use clap::Parser;
use colored::Colorize;

use crate::{
    api::Api,
    cache::{Cache, CacheManager},
    config::Config,
    dto::{PersonalBests, TestResult, UserStats},
    error::AppResult,
    print::print_user_data,
};

#[derive(Debug, Parser)]
#[command(version)]
struct Args {
    /// Path to configuration file. Defaults to `config.toml` if not explicitly set.
    #[arg(long, default_value = "config.toml")]
    config: String,
}

// All user related DTOs combined
struct FullUserData {
    username: String,
    stats: UserStats,
    personal_bests: PersonalBests,
    test_results: Vec<TestResult>,
}

fn main() {
    if let Err(why) = run() {
        eprintln!(
            "{} {}",
            "==>".red().bold(),
            "Something went wrong:".white().bold(),
        );
        eprintln!(" - {why}");
    }
}

fn run() -> AppResult<()> {
    let args = Args::parse();
    let cfg = Config::from_file(&args.config)?;
    cfg.validate()?;

    let cache = CacheManager::new();
    if let Some(data) = cache.load()? {
        println!("{data:#?}");
    }

    // let api = Api::new(&cfg.api_key);
    // let username = api.username()?;
    // let stats = api.user_stats()?;
    // let personal_bests = api.personal_bests()?;
    // let test_results = api.test_results()?;

    // let c = Cache {
    //     timestamp: Utc::now(),
    //     username,
    //     user_stats: stats,
    //     personal_bests: personal_bests.0,
    //     recent_tests: test_results,
    // };
    // cache.save(c)?;

    // print_user_data(FullUserData {
    //     username,
    //     stats,
    //     personal_bests,
    //     test_results,
    // });

    Ok(())
}
