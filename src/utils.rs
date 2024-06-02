pub fn hashtag_string_or_na(s: &Option<String>) -> String {
    let raw_string = match s {
        Some(s) => s.clone(),
        None => "N/A".to_string(),
    };
    if raw_string == "N/A" {
        raw_string
    } else {
        let mut hash_tag = String::from("#");
        for i in raw_string.chars() {
            if i.is_alphabetic() | i.is_numeric() {
                hash_tag.push(i)
            }
        }
        hash_tag
    }
}

pub fn string_or_na(s: &Option<String>) -> String {
    match s {
        Some(s) => s.clone(),
        None => "N/A".to_string(),
    }
}

pub fn get_genres(genres: &Vec<String>) -> String {
    if genres.is_empty() {
        return "N/A".to_string();
    };
    let mut genres_tags: Vec<String> = vec![];
    for i in genres.iter() {
        let new_tag = hashtag_string_or_na(&Some(i.clone()));
        genres_tags.push(new_tag);
    }
    genres_tags.join(" ")
}

pub fn get_when() -> String {
    chrono::Utc::now().format("%d %B %Y").to_string()
}
