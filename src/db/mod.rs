use diesel::sqlite::{SqliteConnection, Sqlite};
use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenvy::dotenv;
use std::env;
use std::io::{Result, Error, ErrorKind};

use crate::models::{NewTrack, Track};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

/// Initializes a connection to the given database
/// and returns the database connection.
/// 
/// Falls back to the `DATABASE_URL` environment variable if no url is given.
///
/// # Errors
/// 
/// This function will return an error if the connection fails or, in case the
/// connection succeeds, if the migrations fail.
pub fn init_connection(link: Option<String>) -> Result<SqliteConnection> {
    dotenv().ok();

    let db_url = match link {
        Some(l) => l.to_string(),
        None => env::var("DATABASE_URL").expect("DATABASE_URL was not given!")
    };

    let mut con = match SqliteConnection::establish(&db_url) {
        Ok(con) => con,
        Err(err) => return Err(Error::new(ErrorKind::Other, err))
    };
    
    match run_migrations(&mut con) {
        Ok(_) => Ok(con),
        Err(err) => Err(err)
    }
}

fn run_migrations(con: &mut impl MigrationHarness<Sqlite>) -> Result<()>{

    match con.run_pending_migrations(MIGRATIONS) {
        Ok(_) => Ok(()),
        Err(_) => Err(Error::new(ErrorKind::Other, "Error running migrations"))
    }
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
