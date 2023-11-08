// @generated automatically by Diesel CLI.

diesel::table! {
    new_seasons (id) {
        id -> Nullable<Integer>,
        title -> Text,
        url -> Text,
        language -> Nullable<Text>,
        description -> Nullable<Text>,
        genres -> Text,
        image_url -> Nullable<Text>,
        is_published -> Bool,
        season_number -> Integer,
        image_path -> Nullable<Text>,
        host -> Nullable<Text>,
    }
}
