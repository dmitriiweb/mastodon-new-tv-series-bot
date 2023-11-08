use crate::apis;
use diesel::prelude::*;

#[derive(Insertable)]
#[diesel(table_name = crate::db::schema::new_seasons)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewSeasonModel {
    pub title: String,
    pub url: String,
    pub language: Option<String>,
    pub description: Option<String>,
    pub genres: Option<String>,
    pub image_url: Option<String>,
    pub is_published: bool,
    pub season_number: i32,
    pub image_path: Option<String>,
    pub host: Option<String>,
}

impl NewSeasonModel {
    pub fn from_api(ns: apis::SeasonData, image_name: Option<String>) -> Self {
        Self {
            title: ns.title,
            url: ns.url,
            language: ns.language,
            description: ns.description,
            genres: Self::get_genres(ns.genres),
            image_url: ns.image_url,
            season_number: ns.season_number,
            host: ns.host,
            is_published: false,
            image_path: image_name,
        }
    }
    fn get_genres(genres: Vec<String>) -> Option<String> {
        if genres.len() == 0 {
            return None;
        }
        Some(genres.join(","))
    }
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::db::schema::new_seasons)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewSeasonModelSelectable {
    pub id: Option<i32>,
    pub title: String,
    pub url: String,
    pub language: Option<String>,
    pub description: Option<String>,
    pub genres: Option<String>,
    pub image_url: Option<String>,
    pub is_published: bool,
    pub season_number: i32,
    pub image_path: Option<String>,
    pub host: Option<String>,
}
