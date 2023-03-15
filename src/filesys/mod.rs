use id3::{Tag, TagLike, Error};

use crate::models::NewTrack;

mod id3tags;

pub mod crawler;

fn get_text_tag(tag: &str, tags: &Tag) -> Option<String> {
    match tags.get(tag) {
        Some(t) => match t.content().text() {
            Some(text) => Some(text.to_string()),
            None => None
        },
        None => None
    }
}

fn get_int_tag(tag: &str, tags: &Tag) -> Option<i32> {
    match tags.get(tag) {
        Some(t) => match t.content().text() {
            Some(text) => match text.parse::<i32>() {
                Ok(num) => Some(num),
                Err(_) => None
            },
            None => None
        },
        None => None
    }
}

fn default_title(path: &str) -> String {
    let mut file = path.split('/').last().unwrap().to_string();
    
    if let Some(ext) = file.split('.').last() {
        file = file.replace(&format!(".{}", ext), "");
    }

    return file;
}

pub fn get_track(path: &str) -> Result<NewTrack,Error> {
    let tags = Tag::read_from_path(path)?;

    let title = get_text_tag(id3tags::TITLE, &tags)
                        .unwrap_or_else(|| default_title(path));
    
    Ok(NewTrack {
        title,
        artist: get_text_tag(id3tags::ARTIST, &tags),
        album: get_text_tag(id3tags::ALBUM, &tags),
        album_artist: get_text_tag(id3tags::ALBUM_ARTIST, &tags),
        track_number: get_int_tag(id3tags::TRACK, &tags),
        genre: get_text_tag(id3tags::GENRE, &tags),
        composer: get_text_tag(id3tags::COMPOSER, &tags),
        length: get_int_tag(id3tags::LENGTH, &tags).unwrap_or(0),
        sample_rate: get_int_tag(id3tags::SAMPLE_RATE, &tags).unwrap_or(0),
        codec: get_text_tag(id3tags::CODEC, &tags).unwrap_or("".to_string()),
        filepath: path.to_string()
    })

}
