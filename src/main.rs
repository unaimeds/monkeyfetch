mod error;
mod config;
mod dto;
mod api;

use clap::Parser;

use crate::{api::Api, config::Config};

#[derive(Debug, Parser)]
#[command(version)]
struct Args {
    /// Path to configuration file. Defaults to `config.toml` if not explicitly set.
    #[arg(long, default_value = "config.toml")]
    config: String,
}

fn main() {
    // TODO: handle errors
    let args = Args::parse();
    let cfg = Config::from_file(&args.config).unwrap();
    cfg.validate().unwrap();

    let api = Api::new(&cfg.api_key);

    // let stats = api.user_stats().unwrap();
    // let bests = api.personal_bests().unwrap();
    // println!("{stats:#?}");
    // println!("{bests:#?}");
    // let tests = api.test_results().unwrap();
    // println!("{tests:#?}");
    let username = api.username().unwrap();
    println!("{username}");
}
