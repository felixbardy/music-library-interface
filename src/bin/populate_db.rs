use music_library_interface::{filesys::crawler::LibraryCrawler, db};

fn main() {
    let mut connection = db::init_connection();
    let crawler = LibraryCrawler::new(&String::from("/home/felix/Music")).unwrap();
    for track in crawler {
        db::insert_track(&mut connection, &track);
    }
}
