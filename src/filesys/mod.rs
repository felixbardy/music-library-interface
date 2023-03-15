use id3::{Tag, TagLike, Error};
use metadata::MediaFileMetadata;
use metadata::StreamMetadata::AudioMetadata;

use std::io::Result;
use std::path::Path;

use crate::models::NewTrack;

pub mod crawler;

fn get_tag_content(tags: &Vec<(String, String)>, name: &str) -> Option<String>{
    match tags.iter().find(|elt| elt.0 == name) {
        Some(pair) => Some(pair.1.clone()),
        None => None
    }
}

fn get_codec(md: &MediaFileMetadata) -> Option<String> {
    // We're only interested in the AudioMetadata
    match md._streams_metadata.iter().find(
        |smd| if let AudioMetadata(_) = smd {true}
        else {false}
    ) {
        Some(AudioMetadata(data)) => Some(data.codec_desc.clone()),
        Some(_) => None,
        None => None,
    }
}

pub fn get_track(path: &str) -> Result<NewTrack> {
    // Extract metadata from file
    let mut md = MediaFileMetadata::new(&path)?;
    let md = md.include_tags(true);

    // Generate NewTrack from metadata
    Ok(NewTrack {
        title: match md.title.as_ref() {
            Some(title) => title.clone(),
            // If no title is found, use the filename
            None => Path::new(path).file_name().unwrap().to_str().unwrap().to_string()
        },
        artist: get_tag_content(&md.tags, "artist"),
        album: get_tag_content(&md.tags, "album"),
        album_artist: get_tag_content(&md.tags, "album_artist"),
        composer: get_tag_content(&md.tags, "composer"),
        track_number: match get_tag_content(&md.tags, "track") {
            // If there is a track number, parse it to an i32
            Some(track) => match track.parse::<i32>() {
                Ok(n) => Some(n),
                Err(_) => None
            },
            None => None
        },
        genre: get_tag_content(&md.tags, "genre"),
        // Downcast f64 to f32
        length: md._duration.unwrap() as f32,
        // Downcast u64 to i32
        sample_rate: match i32::try_from(md._bit_rate.unwrap_or(0)) {
            Ok(rate) => rate,
            Err(_) => 0
        },
        // If no codec is found, use "unknown"
        codec: get_codec(md).unwrap_or("unknown".to_string()),
        filepath: md.path.clone(),
    })
}
