use crate::apis;
use crate::config::Config;
use crate::requests::{upload_file, FileUpload, RequestData};
use log::error;
use reqwest::header::HeaderMap;
use std::error::Error;

const DEFAULT_HASHTAGS: &str = "#tvseries #tvshows";
const MASTODON_URL_LENGTH: i32 = 23;

#[derive(Debug)]
pub struct MastodonPost<'a> {
    pub post_text: String,
    pub config: &'a Config,
    pub image_ids: Vec<String>,
}

impl<'a> MastodonPost<'a> {
    pub fn from_season_data(
        data: &apis::SeasonData,
        config: &'a Config,
        image_id: Option<String>,
    ) -> Self {
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
        let post_text = Self::trim_post(post, config.max_post_len, &data.url);
        let image_ids = match image_id {
            Some(id) => vec![id],
            None => vec![],
        };
        Self {
            post_text,
            config,
            image_ids,
        }
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

    fn get_genres(genres: &Vec<String>) -> String {
        if genres.is_empty() {
            return "N/A".to_string();
        };
        let genres = genres
            .iter()
            .map(|g| format!("#{} ", g))
            .collect::<String>();
        genres
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

impl<'a> RequestData for MastodonPost<'a> {
    fn url(&self) -> String {
        self.config.mastodon_url.clone() + "/api/v1/statuses"
    }
    fn headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        let auth_key = format!("Bearer {}", &self.config.mastodon_token);
        headers.insert(
            reqwest::header::AUTHORIZATION,
            reqwest::header::HeaderValue::from_str(&auth_key).unwrap(),
        );
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_static("multipart/form-data"),
        );
        headers
    }

    fn json_body(&self) -> reqwest::blocking::multipart::Form {
        let status = reqwest::blocking::multipart::Part::text(self.post_text.clone());
        let visibility = reqwest::blocking::multipart::Part::text("public".to_string());
        let media_ids = self.image_ids.join(",");
        let media_ids = reqwest::blocking::multipart::Part::text(media_ids);
        let form = reqwest::blocking::multipart::Form::new()
            .part("status", status)
            .part("visibility", visibility)
            .part("media_ids[]", media_ids);
        form
    }
}

pub struct ImageUploader<'a> {
    pub config: &'a Config,
    pub image_path: &'a str,
    pub image_title: &'a str,
}

impl<'a> ImageUploader<'a> {
    // Upload image to mastodon and return image id
    pub fn upload(&self) -> Result<String, Box<dyn Error>> {
        let mut headers = HeaderMap::new();
        let auth_key = format!("Bearer {}", &self.config.mastodon_token);
        headers.insert(
            reqwest::header::AUTHORIZATION,
            reqwest::header::HeaderValue::from_str(&auth_key).unwrap(),
        );

        let file = FileUpload {
            upload_url: self.config.mastodon_image_api_url.clone(),
            file_path: self.image_path.to_string(),
            headers,
            description: self.image_title.to_string(),
            params: vec![],
        };
        let result = upload_file(file)?;
        let json_result: serde_json::Value = serde_json::from_str(&result)?;
        let id = match json_result["id"].as_str() {
            Some(id) => id,
            None => {
                error!("Cannot get image id from response: {}", result);
                std::process::exit(1);
            }
        };
        Ok(id.to_string())
    }
}
