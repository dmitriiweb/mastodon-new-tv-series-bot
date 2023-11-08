pub mod models;
mod schema;

use diesel::prelude::*;
use std::error::Error;

use diesel::{sqlite::SqliteConnection, RunQueryDsl};

pub fn db_connection(sqlite_path: &String) -> SqliteConnection {
    SqliteConnection::establish(sqlite_path)
        .unwrap_or_else(|_| panic!("Error connecting to {}", sqlite_path))
}

pub fn save_new_season(
    conn: &mut SqliteConnection,
    new_season: models::NewSeasonModel,
) -> Result<(), Box<dyn Error>> {
    use schema::new_seasons;
    let saved_season = get_by_url_and_season(conn, &new_season.url, &new_season.season_number);
    if saved_season.is_ok() {
        return Ok(());
    }
    diesel::insert_into(new_seasons::table)
        .values(new_season)
        .execute(conn)?;
    Ok(())
}

pub fn get_by_url_and_season(
    conn: &mut SqliteConnection,
    series_url: &String,
    season_number: &i32,
) -> Result<models::NewSeasonModelSelectable, Box<dyn Error>> {
    use schema::new_seasons::dsl;
    let result = dsl::new_seasons
        .filter(dsl::url.eq(series_url))
        .filter(dsl::season_number.eq(season_number))
        .first(conn)?;
    Ok(result)
}

pub fn get_unpublished(
    conn: &mut SqliteConnection,
) -> Result<Vec<models::NewSeasonModelSelectable>, Box<dyn Error>> {
    use schema::new_seasons::dsl;
    let result = dsl::new_seasons
        .filter(dsl::is_published.eq(false))
        .load::<models::NewSeasonModelSelectable>(conn)?;
    Ok(result)
}

pub fn mark_as_published(conn: &mut SqliteConnection, id: &i32) -> Result<(), Box<dyn Error>> {
    use schema::new_seasons::dsl;
    diesel::update(dsl::new_seasons.find(id))
        .set(dsl::is_published.eq(true))
        .execute(conn)?;
    Ok(())
}
