use diesel::prelude::*;
use crate::schema::track;

#[derive(Queryable)]
/// A track in the database
pub struct Track {
    pub local_id: Option<i32>,
    pub title: String,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub album_artist: Option<String>,
    pub track_number: Option<i32>,
    pub genre: Option<String>,
    pub composer: Option<String>,
    pub length: i32,
    pub sample_rate: i32,
    pub codec: String,
    pub filepath: String
}


#[derive(Insertable)]
#[diesel(table_name = track)]
/// A track that is not yet in the database
pub struct NewTrack {
    pub title:          String,
    pub artist:         Option<String>,
    pub album:          Option<String>,
    pub album_artist:   Option<String>,
    pub track_number:   Option<i32>,
    pub genre:          Option<String>,
    pub composer:       Option<String>,
    pub length:         i32,
    pub sample_rate:    i32,
    pub codec:          String,
    pub filepath:       String
}
