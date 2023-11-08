use chrono;
use clap::Parser;
use log::error;
use std::error::Error;
use std::fs;

pub mod apis;
pub mod config;
pub mod db;
pub mod requests;

use crate::apis::{SeasonData, TvMaze};
use config::Config;
use requests::{download_file, FileDownload, RequestData};

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
    let response = match requests::get(&tv_maze) {
        Ok(resp) => resp,
        Err(err) => {
            error!("Cannot get listing from api: {}", err);
            std::process::exit(1);
        }
    };
    let new_seasons = match tv_maze.get_data(&response) {
        Ok(seasons) => seasons,
        Err(err) => {
            error!("Cannot parse api response: {}", err);
            std::process::exit(1);
        }
    };
    for new_season in new_seasons.iter() {
        let image_name = download_image(config, &tv_maze, new_season);
        // TODO save to db
    }
}

fn download_image(config: &Config, tv_maze: &TvMaze, new_season: &SeasonData) -> Option<String> {
    let image_url = match new_season.image_url {
        Some(ref url) => url.clone(),
        _ => return None,
    };
    let download_image = FileDownload {
        download_url: image_url.clone(),
        save_folder: config.image_dir.clone(),
        headers: tv_maze.headers().clone(),
    };
    let file_name = download_image.file_name();
    download_file(download_image).unwrap();
    Some(file_name)
}

fn main() {
    env_logger::init();
    let args = CliArguments::parse();
    let config = get_config(args.config).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });
    get_new_tv_shows(&config);
}
