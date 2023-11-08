use chrono;
use clap::Parser;
use std::error::Error;
use std::fs;

pub mod apis;
pub mod config;
pub mod requests;

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

fn get_new_tv_shows(config: &Config) {
    let dt_now = chrono::Utc::now();
    let tv_maze = apis::TvMaze::new(dt_now, &config.target_genres);
    let response = requests::get(&tv_maze).unwrap();
    let new_seasons = tv_maze.get_data(&response).unwrap();
    for i in new_seasons.iter() {
        println!("{:?}", i);
    }
    // TODO download image
    // TODO save to db
    // TODO logs if errors
}

fn main() {
    let args = CliArguments::parse();
    let config = get_config(args.config).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });
    get_new_tv_shows(&config);
}
