mod errors;
mod config;
mod models;
mod api;

use clap::Parser;

use crate::{api::Api, config::Config};

#[derive(Debug, Parser)]
#[command(version)]
struct Args {
    /// Path to custom configuration file
    #[arg(long)]
    config: Option<String>,
}

fn main() {
    // TODO: use custom config path if provided
    let _args = Args::parse();
    let cfg = Config::from_file("config.toml").unwrap();
    let api = Api::new(&cfg.api_key);

    let stats = api.user_stats().unwrap();
    println!("{stats:#?}");
}
