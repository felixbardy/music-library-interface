use clap::{Parser, Subcommand};
use clap_generate::generators::{generate, Zsh};
use music_library_interface::{filesys::crawler::LibraryCrawler, db};

#[derive(Parser)]
struct Cli {

    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {

    /// Populate the database with tracks from a library directory
    Generate {

        /// Path to the database to fill or create
        #[clap(short, long, value_name = "DB_FILE")]
        database: Option<String>,

        /// Path to the library directory to scan
        #[clap(short, long, value_name = "DIR")]
        root: Option<String>,
    },

    /// Add content to the database
    Add {
        
        /// A track to add to the database
        #[clap(short, long, value_name = "TRACK_FILE")]
        track: String,
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Generate { database, root } => {
            let mut connection = match db::init_connection(database) {
                Ok(conn) => conn,
                Err(err) => {
                    eprintln!("Error initializing database: {}", err);
                    return;
                }
            };
            let crawler = LibraryCrawler::try_new(&root.unwrap()).unwrap();
            for track in crawler {
                match db::insert_track(&mut connection, &track) {
                    Ok(_) => (),
                    Err(err) => eprintln!(
                        "Error inserting track {}: {}", track.title, err
                    )
                }
            }
        },
        Commands::Add { track } => ()
    }
}
