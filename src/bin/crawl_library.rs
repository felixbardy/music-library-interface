use music_library_interface::filesys::crawler::LibraryCrawler;

fn main() {
    let crawler = LibraryCrawler::new(&String::from("/home/felix/Music")).unwrap();
    for track in crawler {
        println!("Track: {}", track.title);
    }
}
