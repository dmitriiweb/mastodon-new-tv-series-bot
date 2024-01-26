use super::SeasonData;
use crate::requests::RequestData;
use chrono::{DateTime, Utc};
use reqwest::header;
use reqwest::header::HeaderMap;
use scraper::{Html, Selector};
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;

const TV_MAZE_URL: &str = "https://api.tvmaze.com/schedule/web";
const TARGET_SHOW_NUMBER: i32 = 1;

#[derive(Debug, Copy, Clone)]
pub struct TvMaze<'a> {
    target_date: DateTime<Utc>,
    target_genres: &'a Vec<String>,
}

impl<'a> TvMaze<'a> {
    pub fn new(target_date: DateTime<Utc>, target_genres: &Vec<String>) -> TvMaze {
        TvMaze {
            target_date,
            target_genres,
        }
    }
}

impl<'a> TvMaze<'a> {
    pub fn get_data(&self, json_source: &str) -> Result<Vec<SeasonData>, Box<dyn Error>> {
        let mut new_seasons = vec![];
        let json_seasons: Vec<NewRawSeason> = serde_json::from_str(json_source)?;
        for season in json_seasons.iter() {
            if !season.is_target_show_number(TARGET_SHOW_NUMBER) {
                continue;
            }
            if !season._embedded.show.is_target_genres(self.target_genres) {
                continue;
            }
            let new_season = SeasonData {
                title: season._embedded.show.name.to_string(),
                url: season._embedded.show.url.to_string(),
                language: season._embedded.show.language.clone(),
                description: season._embedded.show.description(),
                genres: season._embedded.show.genres.clone(),
                image_url: season._embedded.show.image_url(),
                season_number: season.season.unwrap(),
                host: season._embedded.show.host(),
            };
            new_seasons.push(new_season);
        }
        Ok(new_seasons)
    }
}

impl<'a> RequestData for TvMaze<'a> {
    fn url(&self) -> String {
        TV_MAZE_URL.to_string()
    }
    fn params(&self) -> Vec<(String, String)> {
        vec![(
            "date".to_string(),
            self.target_date.format("%Y-%m-%d").to_string(),
        )]
    }

    fn headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        let user_agent = "Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/119.0";
        headers.insert(
            header::USER_AGENT,
            header::HeaderValue::from_str(user_agent).unwrap(),
        );
        headers
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewRawSeason {
    pub season: Option<i32>,
    pub number: Option<i32>,
    pub _embedded: NewRawEmbedded,
}

impl NewRawSeason {
    pub fn is_target_show_number(&self, target_show_number: i32) -> bool {
        match self.number {
            Some(n) => n == target_show_number,
            None => false,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewRawEmbedded {
    pub show: NewRawShow,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewRawShow {
    pub url: String,
    pub name: String,
    pub language: Option<String>,
    pub genres: Vec<String>,
    image: Option<HashMap<String, String>>,
    summary: Option<String>,
    #[serde(rename = "webChannel")]
    web_channel: Option<NewRawWebChannel>,
}

impl NewRawShow {
    fn is_target_genres(&self, target_genres: &[String]) -> bool {
        for genre in target_genres.iter() {
            if self.genres.contains(genre) {
                return true;
            }
        }
        false
    }
}

impl NewRawShow {
    pub fn image_url(&self) -> Option<String> {
        match &self.image {
            Some(i) => match i.get("original") {
                Some(u) => Some(u.to_string()),
                None => None,
            },
            None => None,
        }
    }
}

impl NewRawShow {
    pub fn description(&self) -> Option<String> {
        let summary = match &self.summary {
            Some(s) => s,
            None => return None,
        };
        let fragment = Html::parse_fragment(summary);
        let selector = Selector::parse("p").unwrap();
        let p = fragment.select(&selector).next().unwrap();
        let texts = p.text().collect::<Vec<_>>();
        let texts: Vec<String> = texts.iter().map(|t| t.to_string()).collect();
        let text = texts.join("");
        Some(text)
    }
}

impl NewRawShow {
    pub fn host(&self) -> Option<String> {
        let webchannel = match &self.web_channel {
            Some(w) => w,
            None => return None,
        };
        let host = match &webchannel.name {
            Some(name) => name.to_string(),
            None => return None,
        };
        Some(host)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewRawWebChannel {
    pub name: Option<String>,
}
