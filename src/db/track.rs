use std::io::{Result, Error, ErrorKind};

use diesel::prelude::*;
use diesel::SqliteConnection;

use crate::models::{NewTrack, Track};


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

/// Queries the given database to find a track with the given path.
/// 
/// Returns the track if found, otherwise returns an error
/// 
/// # Errors
/// 
/// This function will return an error if the query fails for any reason.
pub fn get_track_by_path(
    con: &mut SqliteConnection,
    track_path: &str
) -> Result<Track> {
    use crate::schema::track::dsl::*;
    
    match track.filter(filepath.eq(track_path)).first(con) {
        Ok(t) => Ok(t),
        Err(err) => Err(Error::new(ErrorKind::Other, err))}
}
