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
    /// Path to configuration file. Defaults to {CONFIG_DIR}/monkeyfetch/config.toml,
    /// where {CONFIG_DIR} is the default user's config directory based on OS.
    /// For example, in Linux that would be: ~/.config/monkeyfetch/config.toml
    #[arg(long)]
    config: Option<String>,
}

fn main() {
    if let Err(why) = run() {
        eprintln!("{}", "Something went wrong!".white().bold());
        eprintln!("{}", why.to_string().red());
    }
}

fn run() -> AppResult<()> {
    let args = Args::parse();
    let config_path = match args.config {
        Some(ref p) => std::path::PathBuf::from(p),
        None => Config::default_path()?,
    };
    let cfg = Config::from_file(&config_path)?;
    cfg.validate()?;

    let cache = CacheManager::new();
    let data = match cache.load()? {
        Some(d) => d,
        None => {
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
