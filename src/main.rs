use chrono;
use clap::Parser;
use diesel::SqliteConnection;
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
    for i in new_seasons.iter() {
        println!("{:?}", i);
    }
    let mut db_conn = db::db_connection(&config.sqlite_path);
    for new_season in new_seasons.iter() {
        let image_name = download_image(config, &tv_maze, new_season);
        save_new_season(&mut db_conn, new_season, image_name)
    }
}

fn save_new_season(
    conn: &mut SqliteConnection,
    new_season: &SeasonData,
    image_name: Option<String>,
) {
    let genres = if new_season.genres.len() > 0 {
        Some(new_season.genres.join(","))
    } else {
        None
    };
    let m = db::models::NewSeasonModel {
        genres,
        title: new_season.title.clone(),
        url: new_season.url.clone(),
        language: new_season.language.clone(),
        description: new_season.description.clone(),
        image_url: new_season.image_url.clone(),
        is_published: false,
        season_number: new_season.season_number,
        image_path: image_name,
        host: new_season.host.clone(),
    };
    let _ = match db::save_new_season(conn, &m) {
        Ok(_) => (),
        Err(err) => {
            error!("Cannot save new season {}: {}", m.title, err);
            return;
        }
    };
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

fn main() {
    env_logger::init();
    let args = CliArguments::parse();
    let config = get_config(args.config).unwrap_or_else(|err| {
        log::error!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });
    get_new_tv_shows(&config);
}
