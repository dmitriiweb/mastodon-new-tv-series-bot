use serde_derive::Deserialize;
use std::error::Error;
use toml;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub sqlite_path: String,
    pub target_genres: Vec<String>,
    pub mastodon_token: String,
    pub mastodon_url: String,
    pub image_dir: String,
    pub max_post_len: usize,
}

impl Config {
    pub fn new(config_file_content: &String) -> Result<Config, Box<dyn Error>> {
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
            sqlite_path = "db.sqlite3"
            target_genres = ["Anime", "Drama"]
            mastodon_token = "<mastodon api token>"
            mastodon_url = "https://mastodon.social"
            image_dir = "images"
            max_post_len = 500
        "#,
        );
        let config = Config::new(&toml_string).unwrap();
        assert_eq!(config.sqlite_path, "db.sqlite3");
        assert_eq!(config.target_genres, vec!["Anime", "Drama"]);
        assert_eq!(config.mastodon_token, "<mastodon api token>");
        assert_eq!(config.mastodon_url, "https://mastodon.social");
        assert_eq!(config.image_dir, "images");
        assert_eq!(config.max_post_len, 500);
    }

    #[test]
    fn test_invalid_string() {
        let toml_string = String::from("value");
        let config = Config::new(&toml_string);
        assert!(config.is_err());
    }
}
