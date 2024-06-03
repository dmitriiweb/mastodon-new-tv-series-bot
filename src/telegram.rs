use crate::apis;
use crate::config::TelegramConfig;
use crate::requests::{upload_file, FileUpload, RequestData};
use crate::utils;
use reqwest::header::HeaderMap;
use std::collections::HashMap;

const CAPTION_LENGTH: i32 = 1024;
const POST_LENGTH: i32 = 4096;

#[derive(Debug)]
enum PostMethod {
    SendMessage,
    SendPhoto,
}

impl PostMethod {
    pub fn as_string(&self) -> String {
        match &self {
            PostMethod::SendMessage => String::from("sendMessage"),
            PostMethod::SendPhoto => String::from("sendPhoto"),
        }
    }
}

#[derive(Debug)]
pub struct TelegramPost<'a> {
    pub post_text: String,
    pub config: &'a TelegramConfig,
    pub image_path: Option<String>,
    post_method: PostMethod,
}

impl<'a> TelegramPost<'a> {
    pub fn from_season_data(
        data: &apis::SeasonData,
        config: &'a TelegramConfig,
        image_path: Option<String>,
    ) -> Self {
        let language = utils::hashtag_string_or_na(&data.language);
        let genres = utils::get_genres(&data.genres);
        let when = utils::get_when();
        let description = utils::string_or_na(&data.description);
        let host = utils::hashtag_string_or_na(&data.host);
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
        let post_text = Self::trim_post(post, &image_path);

        let post_method = match image_path {
            Some(_) => PostMethod::SendPhoto,
            None => PostMethod::SendMessage,
        };

        Self {
            post_text,
            config,
            image_path,
            post_method,
        }
    }

    fn trim_post(post: String, image_path: &Option<String>) -> String {
        let post_body_length = post.chars().count() as i32;
        let mut is_post_bigger = false;

        let trimmed_post: String = match image_path {
            Some(_) => {
                if post_body_length <= CAPTION_LENGTH {
                    post
                } else {
                    let new_len = CAPTION_LENGTH - 3;
                    is_post_bigger = true;
                    post.chars().take(new_len as usize).collect::<String>()
                }
            }
            None => {
                if post_body_length <= POST_LENGTH {
                    post
                } else {
                    let new_len = POST_LENGTH - 3;
                    is_post_bigger = true;
                    post.chars().take(new_len as usize).collect::<String>()
                }
            }
        };
        if is_post_bigger {
            trimmed_post + "..."
        } else {
            trimmed_post
        }
    }
}

impl<'a> RequestData for TelegramPost<'a> {
    fn url(&self) -> String {
        format!(
            "https://api.telegram.org/bot{}/{}",
            self.config.token,
            self.post_method.as_string()
        )
    }
    fn headers(&self) -> reqwest::header::HeaderMap {
        let mut headers = HeaderMap::new();
        let content_type = match self.post_method {
            PostMethod::SendPhoto => "multipart/form-data",
            PostMethod::SendMessage => "application/json",
        };
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_static(content_type),
        );
        headers
    }

    fn json_body(&self) -> HashMap<String, String> {
        let mut body = HashMap::new();
        body.insert(String::from("chat_id"), self.config.chat_id.clone());
        body.insert(String::from("text"), self.post_text.clone());
        body.insert(String::from("parse_mode"), String::from("HTML"));
        body
    }

    fn json_multipart(&self) -> reqwest::blocking::multipart::Form {
        let chat_id = reqwest::blocking::multipart::Part::text(self.config.chat_id.clone());
        let caption = reqwest::blocking::multipart::Part::text(self.post_text.clone());
        let image_path = match self.image_path.clone() {
            Some(image_path) => image_path,
            None => String::from(""),
        };
        let form = reqwest::blocking::multipart::Form::new()
            .part("caption", caption)
            .part("chat_id", chat_id)
            .file("photo", image_path);

        match form {
            Ok(form) => form,
            Err(_) => panic!("Cant read image for posting"),
        }
    }
}
