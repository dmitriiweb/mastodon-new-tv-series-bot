# New TV Series Bot

![](https://github.com/dmitriiweb/mastodon-new-tv-series-bot/blob/main/img/post-example.png?raw=true)

This bot publishes new seasons from tv maze api to your mastodon account.


## Usage
1. Download the latest release
2. Create a `config.toml` file (see below)
    ```toml
    sqlite_path = "db.sqlite3"
    target_genres = ["Fantasy", "Science-Fiction"]
    mastodon_token = "mastodon token"
    mastodon_url = "https://your.mastodon.instance"
    mastodon_image_api_url = "https://your.mastodon.instance/api/v2/media"
    image_dir = "/path/to/images/dir"
    max_post_len = 500
    ```
3. Create cronjob to run the bot periodically
    ```bash
    0 15 * * * /path/to/binary --config /path/to/config.toml
    ```
   
## Known bots
- [New Sci-Fi and Fantasy Tv](https://patashnik.club/@new_tv_series)