use music_library_interface::{*, models::NewTrack};

fn main() {

    let mut connection = &mut db::init_connection();

    let new_track = NewTrack {
        title: String::from("Les étoiles filantes"),
        artist: Some(String::from("Les Cowboys Fringuants")),
        album: Some(String::from("8 secondes")),
        album_artist: Some(String::from("Les Cowboys Fringuants")),
        track_number: None,
        genre: Some(String::from("Chanson")),
        composer: None,
        length: 120.0,
        sample_rate: 44100,
        codec: String::from("mp3"),
        filepath: String::from("/home/felix/Music/Les Cowboys Fringants/Les antipodes/1-02 Les maisons toutes pareilles.mp3")
    };

    let track = db::insert_track(&mut connection, &new_track).unwrap();

    println!("Song id: {}", track.local_id.unwrap_or(-1));

    let fs_track = filesys::get_track(&track.filepath).unwrap();

    println!("Song title: {}", fs_track.title);
    println!("Song artist: {}", fs_track.artist.unwrap_or("None".to_string()));
    println!("Song album: {}", fs_track.album.unwrap_or("None".to_string()));
    println!("Song album artist: {}", fs_track.album_artist.unwrap_or("None".to_string()));
    println!("Song track number: {}", fs_track.track_number.unwrap_or(-1));
    println!("Song genre: {}", fs_track.genre.unwrap_or("None".to_string()));
    println!("Song composer: {}", fs_track.composer.unwrap_or("None".to_string()));
    println!("Song length: {}s", fs_track.length);
    println!("Song sample rate: {}Hz", fs_track.sample_rate);
    println!("Song codec: {}", fs_track.codec);
    println!("Song filepath: {}", fs_track.filepath);
}
