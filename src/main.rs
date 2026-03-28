mod errors;
mod config;

use clap::Parser;
use reqwest::header::AUTHORIZATION;

use crate::config::Config;

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

    // TODO: create a wrapper for monkeytype API
    let http = reqwest::blocking::Client::new();
    let res = http
        .get("https://api.monkeytype.com/users/stats")
        .header(AUTHORIZATION, format!("ApeKey {}", cfg.api_key))
        .send()
        .unwrap();
    let body = res.text().unwrap();

    println!("{body}");
}
