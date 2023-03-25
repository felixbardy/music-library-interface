use std::{
    iter::Filter, 
    fs::{
        DirEntry,
        self
    }, 
    io::{
        Result,
        Error,
        ErrorKind
    }
};
use lazy_static::lazy_static;
use regex::Regex;

use crate::models::NewTrack;

use super::utils::get_track;

type FilteredDirIter = Filter<fs::ReadDir,fn(&Result<DirEntry>) -> bool>;

lazy_static! {
    /// Regex matching track extensions
    static ref REGEX_TRACK_EXT: Regex = Regex::new(r"mp3|m4a|flac|ogg|wav").unwrap();
}

/// Filters entries to keep only directories
fn filter_is_dir(elt: &Result<DirEntry>) -> bool {
    match elt {
        Ok(entry) => match entry.file_type() {
            Ok(ft) => ft.is_dir(),
            Err(_) => false,
        },
        Err(_) => false,
    }
}

/// Filters entries to keep only tracks
fn filter_is_track(elt: &Result<DirEntry>) -> bool {
    match elt {
        Ok(entry) => match entry.file_type() {
            Ok(ft) => if ft.is_file() {
                matches!(REGEX_TRACK_EXT.captures(
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

pub struct LibIter {
    pub root: String,
    iter: Option<FilteredDirIter>
}

pub struct ArtistIter {
    root: String,
    artist: String,
    iter: Option<FilteredDirIter>
}

impl ArtistIter {
    pub fn root(&self) -> &str {
        self.root.as_ref()
    }

    pub fn artist(&self) -> &str {
        self.artist.as_ref()
    }
}

pub struct AlbumIter {
    root: String,
    artist: String,
    album: String,
    iter: Option<FilteredDirIter>
}

impl AlbumIter {
    pub fn root(&self) -> &str {
        self.root.as_ref()
    }

    pub fn artist(&self) -> &str {
        self.artist.as_ref()
    }

    pub fn album(&self) -> &str {
        self.album.as_ref()
    }

    pub fn next_as_newtrack(&mut self) -> Result<Option<NewTrack>> {
        match &mut self.iter {
            Some(it) => match it.next() {
                Some(Ok(entry)) => {
                    let path = entry.path().to_str().unwrap().to_string();
                    let mut track = get_track(&path)?;
                    if let None = track.album {
                        track.album = Some(self.album().to_string());
                    };
                    if let None = track.artist {
                        track.artist = Some(self.artist().to_string());
                    };
                    Ok(Some(track))
                },
                Some(Err(_)) => Ok(None),
                None => Ok(None)
            },
            None => Ok(None)
        }
    }
}

impl LibIter {
    pub fn try_new(root: &String) -> Result<Self> {
        Ok(Self {
            root: root.clone(),
            iter: match fs::read_dir(root) {
                Ok(it) => Some(it.filter(filter_is_dir)),
                Err(err) => return Err(Error::new(ErrorKind::Other, err))
            }
        })
    }
}

impl Iterator for LibIter {
    type Item = ArtistIter;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.iter {
            Some(it) => match it.next() {
                Some(Ok(entry)) => Some(ArtistIter {
                    root: entry.path().to_str().unwrap().to_string(),
                    artist: entry.file_name().into_string().unwrap(),
                    iter: match fs::read_dir(entry.path()) {
                        Ok(it) => Some(it.filter(filter_is_dir)),
                        Err(_) => None
                    }
                }),
                Some(Err(_)) => None,
                None => None
            },
            None => None
        }
    }
}

impl Iterator for ArtistIter {
    type Item = AlbumIter;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.iter {
            Some(it) => match it.next() {
                Some(Ok(entry)) => Some(AlbumIter {
                    root: entry.path().to_str().unwrap().to_string(),
                    artist: self.artist.clone(),
                    album: entry.file_name().into_string().unwrap(),
                    iter: match fs::read_dir(entry.path()) {
                        Ok(it) => Some(it.filter(filter_is_track)),
                        Err(_) => None
                    }
                }),
                Some(Err(_)) => None,
                None => None
            },
            None => None
        }
    }
}

impl Iterator for AlbumIter {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.iter {
            Some(it) => match it.next() {
                Some(Ok(entry)) => Some(entry.path().to_str().unwrap().to_string()),
                Some(Err(_)) => None,
                None => None
            },
            None => None
        }
    }
}
