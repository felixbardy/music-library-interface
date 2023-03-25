-- Your SQL goes here
-- Your SQL goes here
CREATE TABLE "tracks" (
	"local_id"	INTEGER UNIQUE NOT NULL,
	"title"	TEXT NOT NULL,
	"artist"	TEXT,
	"album"	TEXT,
	"album_artist"	TEXT,
	"track_number"	INTEGER,
	"genre"	TEXT,
	"composer"	TEXT,
	"length"	FLOAT NOT NULL,
	"sample_rate"	INTEGER NOT NULL,
	"codec"	TEXT NOT NULL,
	"filepath"	TEXT UNIQUE NOT NULL,
	PRIMARY KEY("local_id" AUTOINCREMENT)
);
