use std::io::{Result, ErrorKind, Error};
use std::{fs::{self, DirEntry}, iter::Filter};

use lazy_static::lazy_static;
use regex::Regex;
use crate::models::NewTrack;

use super::get_track;

// Iterator on directories
type FilteredDirIter = Filter<fs::ReadDir,fn(&Result<DirEntry>) -> bool>;

lazy_static! {
    static ref TRACK_EXT: Regex = Regex::new(r"mp3|m4a|flac|ogg").unwrap();
}

/// Iterator crawling through a music library
/// composed of two layers of directories:
/// - Artists
/// - Albums
/// - Tracks
/// 
/// The iterator yields NewTrack objects until the whole library has been crawled 
pub struct LibraryCrawler {
    pub root: String,
    current_artist: Option<String>,
    current_album: Option<String>,
    artist_iter: Option<FilteredDirIter>,
    album_iter: Option<FilteredDirIter>,
    track_iter: Option<FilteredDirIter>
}

/**
 * Filters entries to keep only directories
 */
fn filter_is_dir(elt: &Result<DirEntry>) -> bool {
    match elt {
        Ok(entry) => match entry.file_type() {
            Ok(ft) => ft.is_dir(),
            Err(_) => false,
        },
        Err(_) => false,
    }
}

/**
 * Filters entries to keep only tracks
 */
fn filter_is_track(elt: &Result<DirEntry>) -> bool {
    match elt {
        Ok(entry) => match entry.file_type() {
            Ok(ft) => if ft.is_file() {
                matches!(TRACK_EXT.captures(
                    &entry.path().extension().unwrap()
                                .to_str().unwrap().to_string()
                ), Some(_))
            } else {
                false
            },
            Err(_) => false,
        }
        Err(_) => false
    }
}

impl LibraryCrawler {
    pub fn new(root: &String) -> Result<LibraryCrawler> {
        Ok(LibraryCrawler {
            root: root.clone(),
            current_artist: None,
            current_album: None,
            artist_iter: match fs::read_dir(&root) {
                Ok(iter) => Some(iter.filter(filter_is_dir)),
                Err(err) => return Err(err)
            },
            album_iter: None,
            track_iter: None
        })
    }

    /// Sets the next artist as the current artist
    /// and updates iterators accordingly
    /// or returns None if there is no next artist
    fn next_artist(&mut self) -> Result<Option<String>> {
       match self.artist_iter.as_mut().unwrap().next() {
            // If there is a next artist, assign it to current_artist
            Some(Ok(entry)) => {
                self.current_artist = match entry.path().canonicalize() {
                    Ok(path) => match path.into_iter().last() {
                        // Return the last component of the path
                        // (e.g. the default artist name)
                        Some(name) => 
                            Some(
                                name.to_str().unwrap().to_string()
                            ),
                        // There is no world in which the path has no "/"
                        // If there is one, I don't like it
                        None => 
                            return Err(
                                Error::new(
                                    ErrorKind::NotFound,
                                    format!(
                                        "Path '{}' is complete bullcrap",
                                        path.to_str().unwrap().to_string()
                                    )
                                )
                            ),
                    },
                    Err(err) => return Err(err)
                };
                // Set current album to None and make a new album iterator
                self.current_album = None;
                self.album_iter =  match fs::read_dir(
                    self.root.clone() + "/" + &self.current_artist.as_ref().unwrap()
                ) {
                    Ok(iter) => Some(iter.filter(filter_is_dir)),
                    Err(err) => return Err(err),
                };
                Ok(self.current_artist.clone())
            },
            // If an error occurs, relay it
            Some(Err(err)) => Err(err),
            // If there are no more artists, set current artist and iter to None
            None => {
                self.current_artist = None;
                self.album_iter = None;
                Ok(None)
            },
        }
    }

    /// Sets the next album as the current album
    /// and updates iterators accordingly
    /// or returns None if there is no next album
    fn next_album(&mut self) -> Result<Option<String>> {
        match self.album_iter.as_mut() {
            // If there is no album iterator,
            // return None
            None => Ok(None),
            Some(iter) => {
                match iter.next() {
                    // If there is one
                    Some(Ok(entry)) => {
                        self.current_album = Some(
                            entry.file_name().into_string().unwrap().to_string()
                        );
                        // Initialise track_iter
                        self.track_iter = match fs::read_dir(
                            self.root.clone() + "/" 
                            + &self.current_artist.as_ref().unwrap() + "/"
                            + &self.current_album.as_ref().unwrap()
                        ) {
                            Ok(iter) => Some(iter.filter(filter_is_track)),
                            Err(err) => return Err(err),
                        };
                        Ok(self.current_album.clone())
                    },
                    // If an error occurs, relay it
                    Some(Err(err)) => Err(err),
                    // If there are no more albums, 
                    // set current album and iter to None
                    None => {
                        self.current_album = None;
                        self.album_iter = None;
                        Ok(None)
                    },
                }
            }
        }
    }
    
    /// Returns the next track in the library
    /// or None if there is no next track
    fn next_track(&mut self) -> Result<Option<NewTrack>> {
        match self.track_iter.as_mut() {
            Some(iter) => {
                match iter.next() {
                    Some(Ok(entry)) => {
                        // Construct new track
                        // Complete missing metadata
                        // Return track
                        let result = get_track(
                            &entry.path().to_str().unwrap().to_string()
                        );
                        match result {
                            Ok(mut track) => {
                                if let Some(_) = track.album {()} else {
                                    track.album = self.current_album.clone()
                                };
                                if let Some(_) = track.artist {()} else {
                                    track.artist = self.current_artist.clone()
                                };

                                Ok(Some(track))
                            },
                            Err(_) => Ok(Some(
                                    NewTrack {
                                    title: entry.file_name().into_string().unwrap().to_string(),
                                    artist: self.current_artist.clone(),
                                    album: self.current_album.clone(),
                                    album_artist: None,
                                    track_number: None,
                                    genre: None,
                                    composer: None,
                                    length: 0,
                                    sample_rate: 0,
                                    codec: "unknown".to_string(),
                                    filepath: entry.path().to_str().unwrap().to_string()
                                }
                            ))
                        }
                    },
                    Some(Err(err)) => Err(err),
                    None => Ok(None)
                }
            },
            None => Ok(None)
        }
    }

}

impl Iterator for LibraryCrawler {
    type Item = NewTrack;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_track() {
            Ok(Some(track)) => Some(track),
            Ok(None) => {
                match self.next_album() {
                    Ok(Some(_)) => self.next(),
                    Ok(None) => {
                        match self.next_artist() {
                            Ok(Some(_)) => self.next(),
                            Ok(None) => None,
                            Err(_) => None
                        }
                    },
                    Err(_) => None
                }
            },
            Err(_) => None
        }
    }
}
