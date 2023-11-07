use clap::Parser;
use std::error::Error;
use std::fs;

pub mod apis;
pub mod config;

use config::Config;

#[derive(Parser, Debug)]
struct CliArguments {
    // path to the .toml config file
    #[arg(short, long)]
    config: String,
}

fn get_config(toml_file: String) -> Result<config::Config, Box<dyn Error>> {
    let config_file_content = fs::read_to_string(toml_file)?;
    let config: Config = Config::new(&config_file_content)?;
    Ok(config)
}

fn main() {
    let args = CliArguments::parse();
    let config = get_config(args.config).unwrap_or_else(
        |err| {
            eprintln!("Problem parsing arguments: {}", err);
            std::process::exit(1);
        },
    );
    println!("{:?}", config)
}
