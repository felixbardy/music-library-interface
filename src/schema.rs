// @generated automatically by Diesel CLI.

diesel::table! {
    track (local_id) {
        local_id -> Nullable<Integer>,
        title -> Text,
        artist -> Nullable<Text>,
        album -> Nullable<Text>,
        album_artist -> Nullable<Text>,
        track_number -> Nullable<Integer>,
        genre -> Nullable<Text>,
        composer -> Nullable<Text>,
        length -> Float,
        sample_rate -> Integer,
        codec -> Text,
        filepath -> Text,
    }
}
