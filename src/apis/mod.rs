pub mod tv_maze;

pub struct SeasonData {
    pub title: String,
    pub url: String,
    pub language: Option<String>,
    pub description: Option<String>,
    pub genres: Vec<String>,
    pub image_url: Option<String>,
    pub season_number: i32,
    pub host: Option<String>,
}
