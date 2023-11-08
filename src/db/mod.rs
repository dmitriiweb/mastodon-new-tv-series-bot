pub mod models;
mod schema;
pub mod services;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

pub fn db_connection(sqlite_path: &String) -> SqliteConnection {
    SqliteConnection::establish(sqlite_path)
        .unwrap_or_else(|_| panic!("Error connecting to {}", sqlite_path))
}
