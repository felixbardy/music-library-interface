use diesel::prelude::*;
use music_library_interface::*;

fn main() {
    use self::schema::track::dsl::*;

    let connection = &mut db::init_connection();
    let results = track
        .filter(artist.eq("ABBA"))
        .limit(5)
        .load::<self::models::Track>(connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for t in results {
        println!("{}", t.title);
        println!("-----------");
        println!("Id: {}", t.local_id.unwrap_or(-1));
        println!("Artist: {}", t.artist.unwrap_or("None".to_string()));
    }
}
