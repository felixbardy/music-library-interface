use music_library_interface::{filesys::crawler::LibraryCrawler, db};

fn main() {
    let mut connection = db::init_connection(None);
    let crawler = LibraryCrawler::try_new(&String::from("/home/felix/Music")).unwrap();
    for track in crawler {
        match db::insert_track(&mut connection, &track) {
            Ok(_) => (),
            Err(err) => println!("Error inserting track: {}", err)
        }
    }
}
