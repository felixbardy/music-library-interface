use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use std::io::{Result, Error, ErrorKind};

use crate::models::{NewTrack, Track};

pub fn init_connection() -> SqliteConnection {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL was not given!");

    SqliteConnection::establish(&db_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", db_url))
}

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
