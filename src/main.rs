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
    let data = match cache.load()? {
        Some(d) => {
            println!("cache hit");
            d
        }
        None => {
            println!("cache miss");
            let api = Api::new(&cfg.api_key);
            let username = api.username()?;
            let user_stats = api.user_stats()?;
            let personal_bests = api.personal_bests()?;
            let recent_tests = api.test_results()?;
            cache.save(Cache {
                timestamp: Utc::now(),
                username,
                user_stats,
                personal_bests: personal_bests.0,
                recent_tests,
            })?
        }
    };

    print_user_data(data);

    Ok(())
}
