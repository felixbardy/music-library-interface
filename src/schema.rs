// @generated automatically by Diesel CLI.

diesel::table! {
    playlist_tracks (playlist_id, track_number) {
        playlist_id -> Integer,
        track_id -> Integer,
        track_number -> Integer,
    }
}

diesel::table! {
    playlists (id) {
        id -> Integer,
        name -> Text,
        description -> Nullable<Text>,
        image_path -> Nullable<Text>,
    }
}

diesel::table! {
    tracks (local_id) {
        local_id -> Integer,
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

diesel::joinable!(playlist_tracks -> playlists (playlist_id));

diesel::allow_tables_to_appear_in_same_query!(
    playlist_tracks,
    playlists,
    tracks,
);
