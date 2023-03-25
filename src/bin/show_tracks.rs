use diesel::prelude::*;
use music_library_interface::*;

fn main() {
    use self::schema::track::dsl::*;

    let connection = &mut db::init_connection(None).unwrap();
    let results = track
        .filter(artist.eq("ABBA"))
        .limit(5)
        .load::<self::models::Track>(connection)
        .expect("Error loading tracks");

    println!("Displaying {} tracks", results.len());
    for t in results {
        println!("{}", t.title);
        println!("-----------");
        println!("Id: {}", t.local_id);
        println!("Artist: {}", t.artist.unwrap_or("None".to_string()));
        println!("Album: {}", t.album.unwrap_or("None".to_string()));
        println!("\n");
    }
}
