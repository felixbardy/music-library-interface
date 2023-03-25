-- Your SQL goes here
CREATE TABLE "playlists" (
	"id"	INTEGER NOT NULL,
	"name"	TEXT NOT NULL,
	"description"	TEXT,
	"image_path"	TEXT,
	PRIMARY KEY("id" AUTOINCREMENT)
);

CREATE TABLE "playlist_tracks" (
	"playlist_id"	INTEGER NOT NULL,
	"track_id"	INTEGER NOT NULL,
	"track_number"	INTEGER NOT NULL,
	FOREIGN KEY("track_id") REFERENCES "track" ("local_id"),
	FOREIGN KEY("playlist_id") REFERENCES "playlists" ("id"),
	PRIMARY KEY("playlist_id","track_number")
);
