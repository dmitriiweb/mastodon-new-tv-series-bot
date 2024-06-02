use chrono::format;
use clap::Parser;
use log::error;
use mastodon::MastodonImageUploader;
use std::error::Error;
use std::fs;

pub mod apis;
pub mod config;
pub mod mastodon;
pub mod requests;
pub mod telegram;
pub mod utils;

use crate::apis::{SeasonData, TvMaze};
use config::{Config, MastodonConfig};
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

fn get_new_tv_shows(tv_maze: &apis::TvMaze) -> Vec<apis::SeasonData> {
    let response = match requests::get(tv_maze) {
        Ok(resp) => resp,
        Err(err) => {
            error!("Cannot get listing from api: {}", err);
            std::process::exit(1);
        }
    };
    match tv_maze.get_data(&response) {
        Ok(seasons) => seasons,
        Err(err) => {
            error!("Cannot parse api response: {}", err);
            std::process::exit(1);
        }
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
    match download_file(download_image) {
        Ok(_) => (),
        Err(err) => {
            error!("Cannot download image {}: {}", image_url, err);
            return None;
        }
    };
    Some(file_name)
}

fn publish_mastodon_post(
    config: &MastodonConfig,
    new_season: &apis::SeasonData,
    image_path: Option<String>,
) {
    // upload image if image_path is not None
    let image_id = match image_path {
        Some(image_path) => {
            let image_uploader = MastodonImageUploader {
                config,
                image_path: &image_path,
                image_title: &new_season.title,
            };
            match image_uploader.upload() {
                Ok(id) => Some(id),
                Err(err) => {
                    error!("Cannot upload image {}: {}", image_path, err);
                    None
                }
            }
        }
        None => None,
    };
    let mastodon_post = mastodon::MastodonPost::from_season_data(new_season, config, image_id);
    let _ = match requests::post_multipart(&mastodon_post) {
        Ok(r) => r,
        Err(err) => {
            error!("Cannot post to mastodon: {}", err);
            return;
        }
    };
}

fn main() {
    env_logger::init();
    let args = CliArguments::parse();
    let config = get_config(args.config).unwrap_or_else(|err| {
        log::error!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });
    let dt_now = chrono::Utc::now();
    let tv_maze = apis::TvMaze::new(dt_now, &config.target_genres);
    let new_shows = get_new_tv_shows(&tv_maze);
    for new_season in new_shows.iter() {
        let image: Option<String> = download_image(&config, &tv_maze, new_season);
        let image_path: Option<String> = match image {
            Some(image_name) => Some(format!("{}{}", config.image_dir, image_name)),
            None => None,
        };
        publish_mastodon_post(&config.mastodon, new_season, image_path);
    }
}
