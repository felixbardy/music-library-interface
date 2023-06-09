use diesel::prelude::*;
use diesel::sqlite::{Sqlite, SqliteConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenvy::dotenv;
use std::env;
use std::io::{Error, ErrorKind, Result};

use crate::filesys::{lib_iter::LibIter, utils::get_track};
use crate::models::Track;

/// The database migrations, embedded into the binary
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
        None => env::var("DATABASE_URL").expect("DATABASE_URL was not given!"),
    };

    let mut con = match SqliteConnection::establish(&db_url) {
        Ok(con) => con,
        Err(err) => return Err(Error::new(ErrorKind::Other, err)),
    };

    match run_migrations(&mut con) {
        Ok(_) => Ok(con),
        Err(err) => Err(err),
    }
}

/// Runs the embedded database migrations on the given connection.
///
/// # Errors
///
/// This function will return an error if the migrations fail.
pub fn run_migrations(con: &mut impl MigrationHarness<Sqlite>) -> Result<()> {
    match con.run_pending_migrations(MIGRATIONS) {
        Ok(_) => Ok(()),
        Err(_) => Err(Error::new(ErrorKind::Other, "Error running migrations")),
    }
}

/// Loads the library from the given root directory into the given database.
///
/// # Errors
///
/// This function will return an error if the [`LibIter`]
/// cannot be created
pub fn load_library(root: &String, db: &mut SqliteConnection) -> Result<()> {
    let lib_iter = LibIter::try_new(root)?;

    for artist_iter in lib_iter {
        for album_iter in artist_iter {
            for trackpath in album_iter {
                match get_track(&trackpath) {
                    Ok(new_track) => match Track::insert(new_track, db) {
                        Ok(_) => (),
                        Err(err) => eprintln!("Error inserting track: {}", err),
                    },
                    Err(err) => eprintln!("Error parsing track: {}", err),
                }
            }
        }
    }
    Ok(())
}
