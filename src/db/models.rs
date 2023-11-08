use diesel::prelude::*;
use crate::apis;

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

impl NewSeasonModel{
    pub fn from_api(ns: apis::SeasonData, image_name: Option<String>) -> Self {
        Self {
            title: ns.title,
            url: ns.url,
            language: ns.language,
            description: ns.description,
        }
    }
    fn get_genres(genres: Vec<String>) -> String {
        genres.join(",")
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
