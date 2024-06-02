use serde_derive::Deserialize;
use std::error::Error;
use toml;

#[derive(Deserialize, Debug)]
pub struct MastodonConfig {
    pub token: String,
    pub url: String,
    pub image_api_url: String,
    pub max_post_len: i32,
}
impl MastodonConfig {
    pub fn new(config_file_content: &str) -> Result<MastodonConfig, Box<dyn Error>> {
        let config: MastodonConfig = toml::from_str(config_file_content)?;
        Ok(config)
    }
}

#[derive(Deserialize, Debug)]
pub struct TelegramConfig {
    pub token: String,
    pub chat_id: String,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub target_genres: Vec<String>,
    pub send_to: Vec<String>,
    pub image_dir: String,
    pub mastodon: MastodonConfig,
    pub telegram: TelegramConfig,
}

impl Config {
    pub fn new(config_file_content: &str) -> Result<Config, Box<dyn Error>> {
        let config: Config = toml::from_str(config_file_content)?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_string() {
        let toml_string = String::from(
            r#"
            send_to = ["mastodon", "telegram"]
            
            target_genres = ["Fantasy", "Science-Fiction"]
            image_dir = "/path/to/images/dir"
            
            [mastodon]
            token = "mastodon token"
            url = "https://your.mastodon.instance"
            image_api_url = "https://your.mastodon.instance/api/v2/media"
            image_dir = "/path/to/images/dir"
            max_post_len = 500
            
            [telegram]
            token = "telegram token"
            chat_id = "telegram chat id"
        "#,
        );
        let config = Config::new(&toml_string).unwrap();
        assert_eq!(config.target_genres, vec!["Fantasy", "Science-Fiction"]);
        assert_eq!(config.mastodon.token, "mastodon token");
        assert_eq!(config.mastodon.url, "https://your.mastodon.instance");
        assert_eq!(config.image_dir, "/path/to/images/dir");
        assert_eq!(config.mastodon.max_post_len, 500);
        assert_eq!(
            config.mastodon.image_api_url,
            "https://your.mastodon.instance/api/v2/media"
        );
        assert_eq!(config.telegram.token, "telegram token");
    }

    #[test]
    fn test_invalid_string() {
        let toml_string = String::from("value");
        let config = Config::new(&toml_string);
        assert!(config.is_err());
    }
}
