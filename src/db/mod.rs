use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use std::io::{Result, Error, ErrorKind};

use crate::models::{NewTrack, Track};

/// Initializes a connection to the given database
/// and returns the database connection.
/// 
/// Falls back to the `DATABASE_URL` environment variable if no url is given.
///
/// # Panics
///
/// Panics if :
/// - No url is given and the DATABASE_URL environment variable is not set \
/// #### OR
/// - The connection to the database fails
pub fn init_connection(link: Option<&String>) -> SqliteConnection {
    dotenv().ok();

    let db_url = match link {
        Some(l) => l.to_string(),
        None => env::var("DATABASE_URL").expect("DATABASE_URL was not given!")
    };

    SqliteConnection::establish(&db_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", db_url))
}

/// Inserts the given track into the given database
/// and returns the inserted value.
///
/// # Errors
///
/// This function will return an error if the insert fails.
pub fn insert_track(
    con: &mut SqliteConnection,
    new_track: &NewTrack
) -> Result<Track> {
    use crate::schema::track;
    
    match diesel::insert_into(track::table)
                .values(new_track)
                .get_result(con) {
        Ok(track) => Ok(track),
        Err(err) => Err(Error::new(ErrorKind::Other, err))}
}

/// Queries the given database to find a track with the given ID.
/// 
/// Returns the track if found, otherwise returns an error
///
/// # Errors
///
/// This function will return an error if the query fails for any reason.
pub fn get_track_by_id(
    con: &mut SqliteConnection,
    track_id: i32
) -> Result<Track> {
    use crate::schema::track::dsl::*;
    
    match track.find(track_id).first(con) {
        Ok(t) => Ok(t),
        Err(err) => Err(Error::new(ErrorKind::Other, err))}
}
