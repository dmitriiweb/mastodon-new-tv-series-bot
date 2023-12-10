use chrono;
use clap::Parser;
use log::error;
use std::error::Error;
use std::fs;

pub mod apis;
pub mod config;
pub mod mastodon;
pub mod requests;

use crate::apis::{SeasonData, TvMaze};
use crate::mastodon::ImageUploader;
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

fn get_new_tv_shows(config: &Config, tv_maze: &apis::TvMaze) -> Vec<apis::SeasonData> {
    let response = match requests::get(tv_maze) {
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
    new_seasons
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
    let _ = match download_file(download_image) {
        Ok(_) => (),
        Err(err) => {
            error!("Cannot download image {}: {}", image_url, err);
            return None;
        }
    };
    Some(file_name)
}

fn publish_new_post(config: &Config, new_season: &apis::SeasonData, image_name: Option<String>) {
    // upload image if image_path is not None
    // let image_id = match &new_season.image_path {
    //     Some(image_path) => {
    //         let image_path = format!("{}{}", config.image_dir, image_path);
    //         let image_uploader = ImageUploader {
    //             config,
    //             image_path: &image_path,
    //             image_title: &new_season.title,
    //         };
    //         match image_uploader.upload() {
    //             Ok(id) => Some(id),
    //             Err(err) => {
    //                 error!("Cannot upload image {}: {}", image_path, err);
    //                 None
    //             }
    //         }
    //     }
    //     None => None,
    // };
    // let mastodon_post = mastodon::MastodonPost::from_orm(new_season, config, image_id);
    // let _ = match requests::post(&mastodon_post) {
    //     Ok(r) => r,
    //     Err(err) => {
    //         error!("Cannot post to mastodon: {}", err);
    //     }
    // };
}

fn main() {
    env_logger::init();
    let args = CliArguments::parse();
    let config = get_config(args.config).unwrap_or_else(|err| {
        log::error!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });
    // let dt_now = chrono::Utc::now();
    let dt_now = chrono::NaiveDate::parse_from_str("2023-11-29", "%Y-%m-%d")
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_local_timezone(chrono::Utc)
        .unwrap();
    let tv_maze = apis::TvMaze::new(dt_now, &config.target_genres);
    let new_shows = get_new_tv_shows(&config, &tv_maze);
    for new_season in new_shows.iter() {
        print!("{:?}", new_season);
        // let image = download_image(&config, &tv_maze, &new_season);
        // publish_new_post(&config, new_season, image);
    }
}
