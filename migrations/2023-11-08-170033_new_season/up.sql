CREATE TABLE new_seasons
(
    id            INTEGER PRIMARY KEY AUTOINCREMENT,
    title         TEXT    NOT NULL,
    url           TEXT    NOT NULL,
    language      TEXT,
    description   TEXT,
    genres        TEXT    NOT NULL,
    image_url     TEXT,
    is_published  BOOLEAN NOT NULL,
    season_number INTEGER NOT NULL,
    image_path    TEXT,
    host          TEXT
)
