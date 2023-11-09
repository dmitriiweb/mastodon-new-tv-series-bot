use crate::db::models::NewSeasonModelSelectable;
use crate::requests::RequestData;

const DEFAULT_HASHTAGS: &str = "#tvseries #tvshows";
const MASTODON_URL_LENGTH: i32 = 23;

#[derive(Debug)]
pub struct MastodonPost {
    pub post_text: String,
}

impl MastodonPost {
    pub fn from_orm(data: &NewSeasonModelSelectable, max_length: i32) -> Self {
        let language = Self::hashtag_string_or_na(&data.language);
        let genres = Self::get_genres(&data.genres);
        let when = chrono::Utc::now().format("%d %B %Y").to_string();
        let description = Self::string_or_na(&data.description);
        let host = Self::hashtag_string_or_na(&data.host);
        let post = format!(
            "{}\n\
            {}\n\n\
            Host: {}\n\
            When: {}\n\
            Season: {}\n\
            Language: {}\n\
            Genres: {}\n\n\
            {}\n",
            &data.title, &data.url, host, when, &data.season_number, language, genres, description,
        );
        let post = Self::trim_post(post, max_length, &data.url);
        Self { post_text: post }
    }

    fn trim_post(post: String, max_length: i32, source_url: &str) -> String {
        let post_body_length = post.chars().count() as i32;
        let url_length = source_url.chars().count() as i32;
        let default_hashtags_length = DEFAULT_HASHTAGS.chars().count() as i32;
        let available_post_size =
            post_body_length - url_length + default_hashtags_length + MASTODON_URL_LENGTH;
        if available_post_size <= max_length {
            let post = post.clone() + &DEFAULT_HASHTAGS;
            return post;
        }
        let new_post_length = max_length - default_hashtags_length - 5;
        let post = post
            .chars()
            .take(new_post_length as usize)
            .collect::<String>();
        let post = post + "...\n" + &DEFAULT_HASHTAGS;
        post
    }

    fn get_genres(genres: &Option<String>) -> String {
        match genres {
            Some(genres) => {
                let genres = genres.split(",").collect::<Vec<&str>>();
                let genres = genres
                    .iter()
                    .map(|g| format!("#{} ", g))
                    .collect::<String>();
                let genres = genres.replace("ScienceFiction", "SciFi");
                genres
            }
            None => "N/A".to_string(),
        }
    }
    fn hashtag_string_or_na(s: &Option<String>) -> String {
        match s {
            Some(s) => format!("#{}", s),
            None => "N/A".to_string(),
        }
    }
    fn string_or_na(s: &Option<String>) -> String {
        match s {
            Some(s) => format!("{}", s),
            None => "N/A".to_string(),
        }
    }
}

impl MastodonPost {
    pub fn upload_image() {}
}

impl RequestData for MastodonPost {
    fn url(&self) -> String {
        "".to_string()
    }
}
