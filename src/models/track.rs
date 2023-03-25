use diesel::prelude::*;
use crate::schema::*;

#[derive(Queryable)]
/// A track in the database
pub struct Track {
    pub local_id: i32,
    pub title: String,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub album_artist: Option<String>,
    pub track_number: Option<i32>,
    pub genre: Option<String>,
    pub composer: Option<String>,
    pub length: f32,
    pub sample_rate: i32,
    pub codec: String,
    pub filepath: String
}


#[derive(Insertable)]
#[diesel(table_name = tracks)]
/// A track that is not yet in the database
pub struct NewTrack {
    pub title:          String,
    pub artist:         Option<String>,
    pub album:          Option<String>,
    pub album_artist:   Option<String>,
    pub track_number:   Option<i32>,
    pub genre:          Option<String>,
    pub composer:       Option<String>,
    pub length:         f32,
    pub sample_rate:    i32,
    pub codec:          String,
    pub filepath:       String
}

impl Track {
    /// Get all tracks
    pub fn all(conn: &mut SqliteConnection) -> QueryResult<Vec<Track>> {
        tracks::table.load::<Track>(conn)
    }

    /// Get a track by id
    pub fn get(id: i32, conn: &mut SqliteConnection) -> QueryResult<Track> {
        tracks::table.find(id)
                    .get_result::<Track>(conn)
    }

    /// Get a track by filepath
    pub fn get_by_filepath(filepath: &str, conn: &mut SqliteConnection) -> QueryResult<Track> {
        tracks::table.filter(tracks::filepath.eq(filepath))
                    .get_result::<Track>(conn)
    }

    /// Insert a new track
    pub fn insert(track: NewTrack, conn: &mut SqliteConnection) -> QueryResult<usize> {
        diesel::insert_into(tracks::table)
            .values(&track)
            .execute(conn)
    }

    /// Delete a track
    pub fn delete(id: i32, conn: &mut SqliteConnection) -> QueryResult<usize> {
        diesel::delete(tracks::table.find(id))
            .execute(conn)
    }
}
